# ADR-002: Local State Model

## Status
Accepted

## Decision
Use an append-only event log with materialized snapshots under `.brain/`.

## Rationale
- Deterministic replay.
- Better conflict handling than single-file mutable state.
- Auditable evolution and migration path.

## Consequences
- Additional migration and compaction logic required.
