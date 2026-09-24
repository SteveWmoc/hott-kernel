# Changelog

Significant user-visible and compatibility-relevant changes to `hott-kernel`
are recorded here.

## [Unreleased]

Development after the first tagged release will be recorded here.

## [0.1.0] - 2026-09-24

First implementation release of the frozen Core v0.1 and Surface v0.1
bootstrap.

### Added

- Frozen kernel theory `mltt-core/0.1`, including the declarative judgments,
  bidirectional algorithm, beta-delta-iota conversion, explicit noncumulative
  universes, dependent functions and pairs, proof-relevant identity types,
  empty, unit, and natural-number types.
- Strict `hott-core/0.1` parsing plus canonical and semantic serialization,
  with `hott-semantic/0.1` projection and deterministic SHA-256 identities.
- Safe-Rust Core checker with 13 accepted and 19 rejected frozen logical
  conformance fixtures.
- Deterministic structural foundation audits and
  `hott-foundation-manifest/0.1` generation, strict parsing, and verification
  using feature vocabulary `mltt-core-features/0.1`.
- Surface v0.1 (`hott-surface/0.1`): strict parsing, deterministic lexical
  name resolution, structural elaboration to Core, canonical byte emission,
  and an optional compile-and-check wrapper.
- Manual, advisory adversarial-review harness with exact-SHA review packets,
  checked-in risk profiles, bounded publication, usage/cost auditing, and no
  place in the trusted logical core.

### Toolchain

- Rust 1.98.1 is the minimum declared and pinned toolchain for this release.
- The crate forbids unsafe Rust with `#![forbid(unsafe_code)]`.

### Compatibility notes

The versioned theory, transport, semantic projection, Surface format,
foundation-manifest schema, feature vocabulary, and failure semantics follow
their own documented compatibility rules. The Rust library API is still
pre-1.0 and is not frozen by the Core v0.1 theory freeze.
