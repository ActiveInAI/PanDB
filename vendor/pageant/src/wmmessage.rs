use std::ffi::CString;
use std::io::IoSlice;
use std::pin::Pin;
use std::task::{Context, Poll};

use byteorder::{BigEndian, ByteOrder};
use bytes::BytesMut;
use delegate::delegate;
use log::debug;
use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt, DuplexStream, ReadBuf};
use windows::Win32::Foundation::{CloseHandle, HANDLE, HWND, INVALID_HANDLE_VALUE, LPARAM, WPARAM};
use windows::Win32::System::DataExchange::COPYDATASTRUCT;
use windows::Win32::System::Memory::{
    CreateFileMappingW, FILE_MAP_WRITE, MEMORY_MAPPED_VIEW_ADDRESS, MapViewOfFile, PAGE_READWRITE, UnmapViewOfFile,
};
use windows::Win32::UI::WindowsAndMessaging::{FindWindowW, SendMessageA, WM_COPYDATA};
use windows::core::HSTRING;

use crate::Error;

/// Pageant transport stream. Implements [AsyncRead] and [AsyncWrite].
///
/// The stream has a unique cookie and requests made in the same stream are considered the same "session".
pub struct PageantStream {
    stream: DuplexStream,
}

impl PageantStream {
    pub async fn new() -> Result<Self, Error> {
        let (one, mut two) = tokio::io::duplex(_AGENT_MAX_MSGLEN * 100);

        let cookie = rand::random::<u64>().to_string();
        tokio::spawn(async move {
            let mut buf = BytesMut::new();
            while let Ok(n) = two.read_buf(&mut buf).await {
                if n == 0 {
                    break;
                }
                if buf.len() < 4 {
                    continue;
                }
                let len = BigEndian::read_u32(&buf) as usize;
                // The shared memory mapping is _AGENT_MAX_MSGLEN bytes, so larger
                // frames can never be sent. Reject before buffering them.
                if len.saturating_add(4) > _AGENT_MAX_MSGLEN {
                    debug!("dropping oversized Pageant request: {len} bytes");
                    break;
                }
                if buf.len() < len + 4 {
                    continue;
                }
                let msg = buf.split_to(len + 4).freeze();
                let Ok(response) = query_pageant_direct(cookie.clone(), &msg).map_err(|e| {
                    debug!("Pageant query failed: {:?}", e);
                    e
                }) else {
                    break;
                };
                two.write_all(&response).await?
            }
            std::io::Result::Ok(())
        });

        Ok(Self { stream: one })
    }
}

impl AsyncRead for PageantStream {
    delegate! {
        to Pin::new(&mut self.stream) {
            fn poll_read(
                mut self: Pin<&mut Self>,
                cx: &mut Context<'_>,
                buf: &mut ReadBuf<'_>,
            ) -> Poll<Result<(), std::io::Error>>;

        }
    }
}

impl AsyncWrite for PageantStream {
    delegate! {
        to Pin::new(&mut self.stream) {
            fn poll_write(
                mut self: Pin<&mut Self>,
                cx: &mut Context<'_>,
                buf: &[u8],
            ) -> Poll<Result<usize, std::io::Error>>;

            fn poll_flush(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), std::io::Error>>;

            fn poll_write_vectored(
                mut self: Pin<&mut Self>,
                cx: &mut Context<'_>,
                bufs: &[IoSlice<'_>],
            ) -> Poll<Result<usize, std::io::Error>>;

            fn poll_shutdown(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), std::io::Error>>;
        }

        to Pin::new(&self.stream) {
            fn is_write_vectored(&self) -> bool;
        }
    }
}

struct MemoryMap {
    filemap: HANDLE,
    view: MEMORY_MAPPED_VIEW_ADDRESS,
    length: usize,
    pos: usize,
}

impl MemoryMap {
    fn new(name: String, length: usize) -> Result<Self, Error> {
        let length_u32 = u32::try_from(length).map_err(|_| Error::Overflow)?;
        let filemap = unsafe {
            CreateFileMappingW(
                INVALID_HANDLE_VALUE,
                // Default security descriptor from the process token: owned by the
                // current user, which is what Pageant's owner check expects
                None,
                PAGE_READWRITE,
                0,
                length_u32,
                &HSTRING::from(name.clone()),
            )
        }?;
        if filemap.is_invalid() {
            return Err(Error::from_win32());
        }
        let view = unsafe { MapViewOfFile(filemap, FILE_MAP_WRITE, 0, 0, 0) };
        if view.Value.is_null() {
            let error = Error::from_win32();
            unsafe {
                let _ = CloseHandle(filemap);
            }
            return Err(error);
        }
        Ok(Self { filemap, view, length, pos: 0 })
    }

    fn seek(&mut self, pos: usize) {
        self.pos = pos;
    }

    fn write(&mut self, data: &[u8]) -> Result<(), Error> {
        if self.pos.checked_add(data.len()).is_none_or(|end| end > self.length) {
            return Err(Error::Overflow);
        }

        if data.is_empty() {
            return Ok(());
        }

        unsafe {
            #[allow(clippy::indexing_slicing)] // length checked
            std::ptr::copy_nonoverlapping(&data[0] as *const u8, self.view.Value.add(self.pos) as *mut u8, data.len());
        }
        self.pos += data.len();
        Ok(())
    }

    fn read(&mut self, n: usize) -> Result<Vec<u8>, Error> {
        if self.pos.checked_add(n).is_none_or(|end| end > self.length) {
            return Err(Error::Overflow);
        }
        let mut out = vec![0; n];
        unsafe {
            std::ptr::copy_nonoverlapping(self.view.Value.add(self.pos) as *const u8, out.as_mut_ptr(), n);
        }
        self.pos += n;
        Ok(out)
    }
}

impl Drop for MemoryMap {
    fn drop(&mut self) {
        unsafe {
            let _ = UnmapViewOfFile(self.view);
            let _ = CloseHandle(self.filemap);
        }
    }
}

fn find_pageant_window() -> Result<HWND, Error> {
    let w = unsafe { FindWindowW(&HSTRING::from("Pageant"), &HSTRING::from("Pageant")) }?;
    if w.is_invalid() {
        return Err(Error::NotFound);
    }
    Ok(w)
}

const _AGENT_COPYDATA_ID: u64 = 0x804E50BA;
const _AGENT_MAX_MSGLEN: usize = 8192;

pub fn is_pageant_running() -> bool {
    find_pageant_window().is_ok()
}

/// Send a one-off query to Pageant and return a response.
pub fn query_pageant_direct(cookie: String, msg: &[u8]) -> Result<Vec<u8>, Error> {
    let hwnd = find_pageant_window()?;
    let map_name = format!("PageantRequest{cookie}");

    let mut map: MemoryMap = MemoryMap::new(map_name.clone(), _AGENT_MAX_MSGLEN)?;
    map.write(msg)?;

    let char_buffer = CString::new(map_name.as_bytes()).map_err(|_| Error::InvalidCookie)?;
    let char_buffer_bytes = char_buffer.as_bytes_with_nul();
    let cds = COPYDATASTRUCT {
        dwData: _AGENT_COPYDATA_ID as usize,
        cbData: char_buffer_bytes.len() as u32,
        lpData: char_buffer_bytes.as_ptr() as *mut _,
    };

    let response = unsafe {
        SendMessageA(
            hwnd,
            WM_COPYDATA,
            WPARAM(0), // Should be window handle to requesting app, which we don't have
            LPARAM(&cds as *const _ as isize),
        )
    };

    if response.0 == 0 {
        return Err(Error::NoResponse);
    }

    read_pageant_response(&mut map)
}

fn read_pageant_response(map: &mut MemoryMap) -> Result<Vec<u8>, Error> {
    map.seek(0);
    let mut buf = map.read(4)?;
    let header: [u8; 4] = buf.as_slice().try_into().map_err(|_| Error::NoResponse)?;
    let size = u32::from_be_bytes(header) as usize;
    if size > _AGENT_MAX_MSGLEN - 4 {
        return Err(Error::Overflow);
    }
    buf.extend(map.read(size)?);
    Ok(buf)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};

    fn test_map(length: usize) -> Result<MemoryMap, Error> {
        static NEXT: AtomicUsize = AtomicUsize::new(1);
        MemoryMap::new(
            format!("PanDBPageantRegression-{}-{}", std::process::id(), NEXT.fetch_add(1, Ordering::Relaxed)),
            length,
        )
    }

    #[test]
    fn malicious_response_lengths_are_rejected_before_allocation() -> Result<(), Error> {
        for size in [u32::MAX, 524288, 8192, 8189] {
            let mut map = test_map(_AGENT_MAX_MSGLEN)?;
            map.write(&size.to_be_bytes())?;
            assert!(matches!(read_pageant_response(&mut map), Err(Error::Overflow)));
        }
        Ok(())
    }

    #[test]
    fn maximum_valid_response_preserves_header_and_payload() -> Result<(), Error> {
        let mut map = test_map(_AGENT_MAX_MSGLEN)?;
        let payload = vec![42; _AGENT_MAX_MSGLEN - 4];
        let mut expected = ((_AGENT_MAX_MSGLEN - 4) as u32).to_be_bytes().to_vec();
        expected.extend_from_slice(&payload);
        map.write(&expected)?;
        assert_eq!(read_pageant_response(&mut map)?, expected);
        Ok(())
    }

    #[test]
    fn mapping_access_checks_cursor_overflow_and_exact_end() -> Result<(), Error> {
        let mut map = test_map(_AGENT_MAX_MSGLEN)?;
        map.seek(usize::MAX);
        assert!(matches!(map.read(1), Err(Error::Overflow)));
        assert!(matches!(map.write(&[1]), Err(Error::Overflow)));
        map.seek(_AGENT_MAX_MSGLEN);
        assert!(map.read(0)?.is_empty());
        assert!(matches!(map.read(1), Err(Error::Overflow)));
        Ok(())
    }

    #[test]
    fn a_truncated_header_is_rejected() -> Result<(), Error> {
        let mut map = test_map(3)?;
        map.write(&[0, 0, 0])?;
        assert!(matches!(read_pageant_response(&mut map), Err(Error::Overflow)));
        Ok(())
    }

    #[test]
    fn invalid_mapping_sizes_are_rejected() {
        assert!(test_map(0).is_err());
        assert!(matches!(test_map(usize::MAX), Err(Error::Overflow)));
    }
}
