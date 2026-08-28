# 0012: Freeze Core v0.1 for implementation

- **Status:** Accepted for Core v0.1 implementation
- **Date:** 2026-08-28
- **Reviewed candidate:** `841be2681abdf5ea3b3fa21c66c76a37ccbc73ac`

## Decision

The reviewed candidate identified above passes the implementability review.
Core theory `mltt-core/0.1` is frozen for implementation, and the charter's
Phase 0 exit condition is satisfied.

The freeze covers:

- the declarative judgments, inference rules, and universe policy in
  `core-v0.1.md`;
- the beta-delta-iota conversion relation and bidirectional algorithmic
  contracts;
- the three declaration kinds and their sequential validation;
- `hott-core/0.1`, `hott-semantic/0.1`, the foundation-manifest schema and
  feature vocabulary, and the failure-class meanings;
- the format and conformance fixtures that distinguish those contracts.

The commit adding this attestation changes status and governance prose only.
It does not alter the reviewed candidate's accepted judgments or serialized
contracts.

## Reason

The implementability review traces all 23 term constructors through syntax,
declarative rules, algorithmic checking, conversion, serialization, audit
extraction, and tests. It also covers all declaration kinds and all nine head
computation rules.

The review resolved conventional but previously implicit mechanics: cutoff
shifting, substitution under binders, shifted lookup, deterministic weak-head
exposure, motive decomposition, validated-input conversion, and sequential
environment construction. Each clarification preserves the declarative
theory. No remaining ambiguity forces implementers to choose between different
accepted judgments.

## Consequences

- Kernel implementation may begin against the frozen contracts.
- The first implementation step may define the AST, strict parser, canonical
  printer, and byte-for-byte round trips without implementing typing.
- Any change to accepted typing or conversion behavior requires a later
  foundational decision and a new theory version.
- Changes to transport, semantic projection, manifest, or feature vocabulary
  follow their independent version policies.
- Editorial clarifications and implementation fixes do not require version
  changes when they preserve specified behavior.
- The metatheorems listed in the review and `metatheory.md` remain unproved.
  The freeze asserts implementability, not soundness, normalization,
  confluence, canonicity, consistency, or correctness of future code.
- A conforming implementation treats `resource-exhausted` as inconclusive and
  never as a mathematical rejection.

The complete review is
[`../implementability-review-v0.1.md`](../implementability-review-v0.1.md).
