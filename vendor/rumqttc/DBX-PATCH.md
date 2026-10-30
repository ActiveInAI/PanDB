

2026-10-30: Ported runtime patches onto registry rumqttc 0.25.1. Registry archives were checksum verified before the three-way port.

The legacy 0.24 TLS retirement guard is replaced for the 0.25 backend by enforced rustls >=0.23.46 and rustls-webpki >=0.103.12 requirements. Certificate verification and explicit opt-in insecure mode remain unchanged. This is a backend migration, not a suppressed audit.
