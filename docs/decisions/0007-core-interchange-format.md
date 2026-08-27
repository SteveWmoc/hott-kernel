# 0007: Canonical textual core interchange

- **Status:** Accepted direction; finalized by Decision 0009
- **Date:** 2026-08-27

## Decision

The primary independent-checker interchange will be a canonical textual
S-expression containing fully explicit core terms. Local variables use de
Bruijn indices and global references use indices into the preceding declaration
sequence. Foundation manifests remain JSON.

The canonical artifact hash covers the exact canonical bytes, including
diagnostic display names. A separately labeled semantic hash covers a
versioned, name-free projection of the parsed core syntax while retaining the
theory version, declaration order and kind, types, and bodies.

## Reason

A small textual grammar is straightforward to audit and independently parse.
It avoids forcing a second checker to share Rust serialization libraries or a
complex surface parser.

## Consequences

- The format carries explicit syntax and theory versions.
- Metavariables, tactics, implicit arguments, and local binder names are absent.
- Renaming a display name changes the artifact hash but not the semantic hash.
- Manifests must label the two hashes and their algorithms unambiguously.
- A binary format is deferred and cannot silently replace the textual format.
- The exact grammar and canonical hashing rules must be frozen before Core
  v0.1 compatibility is promised.

See [`../core-format.md`](../core-format.md) for the frozen specification and
[`0009-freeze-core-format.md`](0009-freeze-core-format.md) for the finalizing
decision.
