# 0007: Canonical textual core interchange

- **Status:** Accepted direction; exact grammar remains pre-freeze
- **Date:** 2026-08-27

## Decision

The primary independent-checker interchange will be a canonical textual
S-expression containing fully explicit core terms. Local variables use de
Bruijn indices and global references use indices into the preceding declaration
sequence. Foundation manifests remain JSON.

## Reason

A small textual grammar is straightforward to audit and independently parse.
It avoids forcing a second checker to share Rust serialization libraries or a
complex surface parser.

## Consequences

- The format carries explicit syntax and theory versions.
- Metavariables, tactics, implicit arguments, and local binder names are absent.
- A binary format is deferred and cannot silently replace the textual format.
- The exact grammar and canonical hashing rules must be frozen before Core
  v0.1 compatibility is promised.

See [`../core-format.md`](../core-format.md) for the current sketch.
