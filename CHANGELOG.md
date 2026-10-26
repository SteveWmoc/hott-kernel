# Changelog

Significant user-visible and compatibility-relevant changes to `hott-kernel`
are recorded here.

## [Unreleased]

### Added

- Proved that function action on paths preserves identity functions and
  composition, with exact audits and higher reflexivity computation tests.

- Proved homotopy naturality at universe zero by path induction, with
  exact foundation audits and proof-relevant reflexivity computation tests.

- Added transfer of chosen quasi-inverse data along a homotopy, preserving
  the inverse map, with exact audits and computation regressions.

- Added composition of chosen quasi-inverse data using `ap` and path
  concatenation, with complete audits and projection-computation tests.

- Added symmetry of chosen quasi-inverse data, with complete foundation
  audits and generic computation tests for all three projections.

- Packaged transport with inverse-path transport and both inverse laws as
  chosen quasi-inverse data, with exact audits and computation regressions.

- Added chosen quasi-inverse data, its constructor and projections, and the
  identity example, with complete foundation audits.

- Defined dependent pointwise homotopy at universe zero and proved its
  reflexive, symmetric, and transitive laws, with complete foundation audits.

- Began the checked HoTT library with explicit universe-zero path inversion,
  path concatenation, dependent transport, and function action on paths
  (`ap`), all defined from `J`, including the judgmental right-unit
  computation for concatenation, a propositional left-unit law, associativity
  proved by path induction, judgmental reflexivity computations for transport
  and `ap`, propositional involutivity of path inversion, both concatenation
  inverse laws, reversal of concatenation by inversion, and propositional
  laws that `ap` preserves path concatenation and path inversion, and that
  transport along concatenation agrees with successive transports.
- Proved both pointwise transport inverse laws at universe zero by path
  induction, with complete direct and transitive foundation audits.
- Documented library-layer discipline: Surface source remains untrusted,
  universe levels remain explicit, and the constructive base library introduces
  no postulates or extensions.

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
