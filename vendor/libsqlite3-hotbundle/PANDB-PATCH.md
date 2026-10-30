PanDB uses this package as a local Cargo path dependency. Native duplicate-symbol fixes are retained from zipg/libsqlite3-hotbundle commit 7f12f203.

The cipher implementation is stable SQLite3MultipleCiphers 2.6.1 (tag commit 45d4ed3487bc02cb08ac73afff47c031c3f88ee0). The SQLite core and headers use official 3.54.1 from maintenance commit 30f1abc0e6cf1d7ee792d2902cd20cebdf46b52a; sqlite3.c matches the official release SHA3-256. The stable tag's reviewed patchsqlite3.sh and amalgamate.py regenerate the checked-in C file. All codec hook counts and original licenses are preserved. Reproduction inputs and the generated hash are in PANDB-SOURCE.json.

Validation on 2026-10-30: all seven genuinely encrypted SQLite 3.51.3 fixtures were opened read-only by 3.54.1. Expected values survived, wrong keys were rejected, and file SHA-256 values remained unchanged.
