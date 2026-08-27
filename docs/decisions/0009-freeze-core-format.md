# 0009: Freeze Core interchange format v0.1

- **Status:** Accepted for Core format v0.1
- **Date:** 2026-08-27

## Decision

This decision finalizes the direction accepted in Decision 0007.

Core format v0.1 is a canonical UTF-8 S-expression format for self-contained
modules. Constructor tags and arities are fixed. Canonical output uses shortest
decimal natural numbers, one-space list separation, no comments, no internal
line breaks, and one final line feed.

Display-name strings admit the Unicode scalar ranges fixed by the format and
exactly the escapes `\"` and `\\`. Unicode normalization and `\u` escapes are
absent.

The artifact hash is SHA-256 of exact canonical `hott-core` bytes. The semantic
hash is SHA-256 of a separately versioned canonical `hott-semantic` projection
that omits display names and the transport-format version while retaining the
theory version, declaration order and kind, types, bodies, and indices.

## Reason

Independent checkers need one unambiguous byte representation and testable hash
preimages without sharing serialization libraries. A strict string subset
avoids surrogate, normalization, and multiple-escape ambiguities. Separate
artifact and semantic hashes distinguish exact provenance-bearing bytes from
name-free mathematical identity.

Self-contained modules close the Phase 0 format without prematurely designing
imports or external environment resolution.

## Consequences

- Display-name edits change the artifact hash but not the semantic hash.
- Transparent, opaque, and postulate declarations remain distinct in the
  semantic projection.
- A future binary format may share semantic hashes only by producing the same
  versioned semantic projection.
- Imports, new escapes, new term tags, or incompatible canonicalization require
  an explicit format decision and version change.
- The exact specification and fixtures live in
  [`../core-format.md`](../core-format.md) and `tests/format/`.
