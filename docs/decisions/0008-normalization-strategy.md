# 0008: Normalization and conversion strategy

- **Status:** Accepted for the primary Core v0.1 implementation plan
- **Date:** 2026-08-27

## Decision

The primary checker will implement beta-delta-iota conversion by normalization
by evaluation without eta expansion. The metatheory will target strong
normalization using a universe-indexed logical relation or reducibility
argument.

## Reason

Normalization by evaluation separates evaluation from quotation, handles
binders cleanly, and is a well-understood route to practical conversion
checking. A logical-relations proof can establish the fundamental theorem for
well-typed terms and yield strong normalization and canonicity consequences.

## Consequences

- Judgmental equality remains exactly the relation specified by Core v0.1;
  implementation optimization may not enlarge it.
- Quotation will not eta-expand functions or pairs.
- A simpler reference normalizer may be used by the independent checker if it
  decides the same relation.
- Strong normalization is a metatheoretic target, not an extra kernel rule and
  not a prerequisite for beginning implementation.
