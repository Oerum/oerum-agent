# Troubleshooting

Start with `brain doctor` — it reports the state of `.brain/`, the event
log, and the sync configuration with concrete remediation hints.

## `brain resume` fails with missing snapshot

Run `brain init` in the repository root.

## `brain sync` reports not configured

Create `.brain/sync.json` with `bucket`, `prefix`, and `region` fields.

## `brain sync` errors with "S3 upload not yet implemented"

The S3 backend is intentionally gated in MVP. The encryption path is
implemented (see [ADR-003](adr/ADR-003-sync-encryption.md)); the network
upload arrives in the next phase. Until then, ship state via your
existing repo (`.brain/snapshot.json` is committed).

## `brain sync` errors with "BRAIN_SYNC_PASSPHRASE must be set"

Sync deliberately refuses to use a default passphrase. Export one:

```sh
export BRAIN_SYNC_PASSPHRASE='your-long-random-passphrase'
```

If you rotate this value, you lose the ability to decrypt previously
uploaded blobs — back up via a password manager.

## Context seems stale

Run `brain checkpoint "manual refresh"` then `brain resume`. The
checkpoint note appears at the top of the resume brief.

## Two AI tools running concurrently

Safe by design — every state mutation is serialised behind
`.brain/locks/events.lock`. Concurrent `brain checkpoint` calls will
block briefly rather than overwrite each other.

## Snapshot file shows up as `snapshot.json.tmp.<uuid>`

A previous `brain` invocation was killed mid-write before the atomic
rename. The previous `snapshot.json` is intact; delete the stray
`.tmp.<uuid>` file safely.
