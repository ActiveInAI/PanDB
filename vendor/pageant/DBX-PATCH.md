# Pageant 0.2.5 maintenance overlay

Based on the official 0.2.5 crates.io archive, checksum and license recorded in PANDB-SOURCE.json. Windows 11+ is the supported desktop baseline. The official Cargo.toml is preserved. Source overlays are listed in PANDB-SOURCE.json.

The overlay rejects oversized mappings and write cursor overflow, checks failed MapViewOfFile before dereferencing, uses a mutable read destination, and bounds response framing. Five Windows native regression tests cover malicious lengths, exact boundaries, truncated headers, and invalid mapping sizes.

The upstream 0.2.5 process-token default security descriptor replaces the older custom SID/security-attribute implementation. No Win7 binding aliases or custom token/SID storage remain. The existing account-name normalization and empty-principal fallback are retained for modern Windows accounts, with eight regression tests. Earlier 0.2.1 test results are historical and do not prove the new version passes.
