use std::ffi::CString;
use std::io::IoSlice;
use std::pin::Pin;
use std::task::{Context, Poll};

use crate::windows::Win32::Foundation::{CloseHandle, HANDLE, HWND, INVALID_HANDLE_VALUE, LPARAM, WPARAM};
use crate::windows::Win32::Security::{
    GetTokenInformation, InitializeSecurityDescriptor, PSECURITY_DESCRIPTOR, PSID, SECURITY_ATTRIBUTES,
    SECURITY_DESCRIPTOR, SetSecurityDescriptorOwner, TOKEN_QUERY, TOKEN_USER, TokenUser,
};
use crate::windows::Win32::System::DataExchange::COPYDATASTRUCT;
use crate::windows::Win32::System::Memory::{
    CreateFileMappingW, FILE_MAP_WRITE, MEMORY_MAPPED_VIEW_ADDRESS, MapViewOfFile, PAGE_READWRITE, UnmapViewOfFile,
};
use crate::windows::Win32::System::Threading::{GetCurrentProcess, OpenProcessToken};
use crate::windows::Win32::UI::WindowsAndMessaging::{FindWindowW, SendMessageA, WM_COPYDATA};
use crate::windows::core::PCWSTR;
use byteorder::{BigEndian, ByteOrder};
use bytes::BytesMut;
use delegate::delegate;
use log::debug;
use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt, DuplexStream, ReadBuf};

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
    fn new(name: String, length: usize, security_attributes: Option<SECURITY_ATTRIBUTES>) -> Result<Self, Error> {
        let length_u32 = u32::try_from(length).map_err(|_| Error::Overflow)?;
        let wide_name = wide_null(&name);
        let filemap = unsafe {
            CreateFileMappingW(
                INVALID_HANDLE_VALUE,
                security_attributes.as_ref().map(|sa| sa as *const _),
                PAGE_READWRITE,
                0,
                length_u32,
                PCWSTR(wide_name.as_ptr()),
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
        let end = self.pos.checked_add(data.len()).filter(|end| *end <= self.length).ok_or(Error::Overflow)?;

        if data.is_empty() {
            return Ok(());
        }

        unsafe {
            #[allow(clippy::indexing_slicing)] // length checked
            std::ptr::copy_nonoverlapping(&data[0] as *const u8, self.view.Value.add(self.pos) as *mut u8, data.len());
        }
        self.pos = end;
        Ok(())
    }

    fn read(&mut self, n: usize) -> Result<Vec<u8>, Error> {
        let end = self.pos.checked_add(n).filter(|end| *end <= self.length).ok_or(Error::Overflow)?;
        if n == 0 {
            return Ok(Vec::new());
        }
        let mut out = vec![0; n];
        // SAFETY: the mapping is non-null and the entire source range was checked
        // before allocating or copying. The destination owns n writable bytes.
        unsafe {
            std::ptr::copy_nonoverlapping(self.view.Value.add(self.pos) as *const u8, out.as_mut_ptr(), n);
        }
        self.pos = end;
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
    let pageant = wide_null("Pageant");
    let w = unsafe { FindWindowW(PCWSTR(pageant.as_ptr()), PCWSTR(pageant.as_ptr())) }?;
    if w.is_invalid() {
        return Err(Error::NotFound);
    }
    Ok(w)
}

fn wide_null(value: &str) -> Vec<u16> {
    value.encode_utf16().chain(std::iter::once(0)).collect()
}

const _AGENT_COPYDATA_ID: u64 = 0x804E50BA;
const _AGENT_MAX_MSGLEN: usize = 8192;

pub fn is_pageant_running() -> bool {
    find_pageant_window().is_ok()
}

struct OwnedProcessToken(HANDLE);
impl Drop for OwnedProcessToken {
    fn drop(&mut self) {
        // SAFETY: this guard owns the successfully opened process token.
        unsafe {
            let _ = CloseHandle(self.0);
        }
    }
}

struct OwnedTokenUser {
    // usize storage keeps TOKEN_USER aligned and retains its embedded SID.
    buffer: Vec<usize>,
}
impl OwnedTokenUser {
    fn sid(&self) -> PSID {
        // SAFETY: GetTokenInformation initialized an aligned TOKEN_USER. Its SID
        // points into this owned buffer, which remains alive through mapping creation.
        unsafe { std::ptr::read(self.buffer.as_ptr().cast::<TOKEN_USER>()).User.Sid }
    }
}

fn get_current_process_user() -> Result<OwnedTokenUser, Error> {
    unsafe {
        let mut process_token = HANDLE::default();
        OpenProcessToken(GetCurrentProcess(), TOKEN_QUERY, &mut process_token as *mut _)?;
        let token = OwnedProcessToken(process_token);
        let mut info_size = 0;
        let _ = GetTokenInformation(token.0, TokenUser, None, 0, &mut info_size);
        if (info_size as usize) < size_of::<TOKEN_USER>() {
            return Err(Error::Overflow);
        }
        let words = (info_size as usize).div_ceil(size_of::<usize>());
        let mut buffer = vec![0usize; words];
        GetTokenInformation(token.0, TokenUser, Some(buffer.as_mut_ptr().cast()), info_size, &mut info_size)?;
        Ok(OwnedTokenUser { buffer })
    }
}

/// Send a one-off query to Pageant and return a response.
pub fn query_pageant_direct(cookie: String, msg: &[u8]) -> Result<Vec<u8>, Error> {
    let hwnd = find_pageant_window()?;
    let map_name = format!("PageantRequest{cookie}");

    let user = get_current_process_user()?;

    let mut sd = SECURITY_DESCRIPTOR::default();
    let sa = SECURITY_ATTRIBUTES {
        lpSecurityDescriptor: &mut sd as *mut _ as *mut _,
        bInheritHandle: true.into(),
        nLength: size_of::<SECURITY_ATTRIBUTES>() as u32,
    };

    let psd = PSECURITY_DESCRIPTOR(&mut sd as *mut _ as *mut _);

    unsafe {
        InitializeSecurityDescriptor(psd, 1)?;
        SetSecurityDescriptorOwner(psd, Some(user.sid()), false)?;
    }

    let mut map: MemoryMap = MemoryMap::new(map_name.clone(), _AGENT_MAX_MSGLEN, Some(sa))?;
    map.write(msg)?;

    let char_buffer = CString::new(map_name.as_bytes()).map_err(|_| Error::InvalidCookie)?;
    let cds = COPYDATASTRUCT {
        dwData: _AGENT_COPYDATA_ID as usize,
        cbData: char_buffer.as_bytes().len() as u32,
        lpData: char_buffer.as_bytes().as_ptr() as *mut _,
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

    fn test_map(length: usize, attributes: Option<SECURITY_ATTRIBUTES>) -> Result<MemoryMap, Error> {
        static NEXT: AtomicUsize = AtomicUsize::new(1);
        MemoryMap::new(
            format!("PanDBPageantRegression-{}-{}", std::process::id(), NEXT.fetch_add(1, Ordering::Relaxed)),
            length,
            attributes,
        )
    }

    #[test]
    fn malicious_response_lengths_are_rejected_before_allocation() -> Result<(), Error> {
        for size in [u32::MAX, 524288, 8192, 8189] {
            let mut map = test_map(_AGENT_MAX_MSGLEN, None)?;
            map.write(&size.to_be_bytes())?;
            assert!(matches!(read_pageant_response(&mut map), Err(Error::Overflow)));
        }
        Ok(())
    }

    #[test]
    fn maximum_valid_response_preserves_header_and_payload() -> Result<(), Error> {
        let mut map = test_map(_AGENT_MAX_MSGLEN, None)?;
        let payload = vec![42; _AGENT_MAX_MSGLEN - 4];
        let mut expected = ((_AGENT_MAX_MSGLEN - 4) as u32).to_be_bytes().to_vec();
        expected.extend_from_slice(&payload);
        map.write(&expected)?;
        assert_eq!(read_pageant_response(&mut map)?, expected);
        Ok(())
    }

    #[test]
    fn mapping_access_checks_cursor_overflow_and_exact_end() -> Result<(), Error> {
        let mut map = test_map(_AGENT_MAX_MSGLEN, None)?;
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
        let mut map = test_map(3, None)?;
        map.write(&[0, 0, 0])?;
        assert!(matches!(read_pageant_response(&mut map), Err(Error::Overflow)));
        Ok(())
    }

    #[test]
    fn token_sid_and_security_attributes_outlive_mapping_creation() -> Result<(), Error> {
        let user = get_current_process_user()?;
        let churn: Vec<_> = (0..256).map(|_| vec![0xff_u8; user.buffer.len() * size_of::<usize>()]).collect();
        assert!(unsafe { crate::windows::Win32::Security::IsValidSid(user.sid()).as_bool() });
        let mut sd = SECURITY_DESCRIPTOR::default();
        let psd = PSECURITY_DESCRIPTOR(&mut sd as *mut _ as *mut _);
        unsafe {
            InitializeSecurityDescriptor(psd, 1)?;
            SetSecurityDescriptorOwner(psd, Some(user.sid()), false)?;
        }
        let attributes = SECURITY_ATTRIBUTES {
            lpSecurityDescriptor: &mut sd as *mut _ as *mut _,
            bInheritHandle: false.into(),
            nLength: size_of::<SECURITY_ATTRIBUTES>() as u32,
        };
        let mut map = test_map(_AGENT_MAX_MSGLEN, Some(attributes))?;
        map.write(&[0, 0, 0, 1, 42])?;
        assert_eq!(read_pageant_response(&mut map)?, vec![0, 0, 0, 1, 42]);
        drop(churn);
        Ok(())
    }
}
