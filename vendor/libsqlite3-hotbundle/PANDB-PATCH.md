Rust wrapper and native duplicate-symbol fix retained from zipg/libsqlite3-hotbundle commit 7f12f203. The C engine uses checksum-verified official SQLite3MultipleCiphers 2.6.1 / SQLite 3.54.0. Existing cipher flags and original licenses are retained.

Validation on 2026-10-30: the new engine opened a genuinely encrypted SQLite 3.51.3 fixture read-only, retained all expected values, rejected a wrong key, and left the fixture SHA-256 unchanged. The default desktop feature graph compiles successfully with the new engine.
