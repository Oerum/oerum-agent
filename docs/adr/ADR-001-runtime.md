# ADR-001: Runtime and Distribution

## Status
Accepted

## Decision
Use Rust for the CLI and service components, compiled into self-contained binaries.

## Rationale
- No user-managed runtime dependency.
- Strong cross-platform support.
- Deterministic and fast startup.

## Consequences
- Higher implementation complexity for rapid prototyping.
- Excellent long-term portability and operability.
