# ADR-003: Optional Remote Sync and Encryption

## Status
Accepted

## Decision
Use optional S3 remote backend with client-side encryption before upload.

## Rationale
- Local mode remains fully functional.
- S3 is enterprise-compatible and backend-agnostic.
- Encrypted payloads keep remote store untrusted.

## Consequences
- Key management and credential diagnostics are required.
