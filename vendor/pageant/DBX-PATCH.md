# DBX Windows 7 compatibility patch

This is `pageant` 0.2.1 with WinRT `HSTRING` removed from Pageant window and
mapping names. It uses null-terminated UTF-16 strings instead. The Win7 target
uses Windows bindings 0.61.3 to avoid COMBASE and WinRT imports unavailable on
Windows 7, while other Windows targets retain the upstream 0.62 dependency.
Named-pipe username lookup also falls back to the local account name when a
user principal is unavailable or empty.


2026-10-30 security maintenance: backport GHSA-g4mp-vgx3-xrvm / CVE-2026-102820 from upstream 0.2.3. Shared-memory reads now check checked-add bounds before allocation/copy, and response frames are capped to the 8192-byte mapping. The SID retains owned aligned storage through CreateFileMapping, SECURITY_ATTRIBUTES borrows the live parameter, opened token handles use RAII, and a failed mapping view is rejected. Windows tests exercise real kernel mappings and malformed response lengths without contacting an installed Pageant agent.
