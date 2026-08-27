# 0011: Separate logical and operational failures

- **Status:** Accepted for Core tooling v0.1
- **Date:** 2026-08-27

## Decision

Core v0.1 tooling distinguishes:

- `unsupported-version`;
- `malformed-encoding`;
- `noncanonical-artifact`;
- `invalid-judgment`;
- `artifact-hash-mismatch`;
- `semantic-hash-mismatch`;
- `manifest-mismatch`;
- `resource-exhausted`.

Only `invalid-judgment` asserts that a grammatically valid declaration fails a
judgment of the selected theory. Resource exhaustion is inconclusive.

## Reason

Parser rejection, logical rejection, integrity failure, audit disagreement,
and insufficient compute answer different questions. Collapsing them would
make defensive limits or corrupted metadata appear to be mathematical
counterexamples.

## Consequences

- Mathematical levels, indices, and term sizes remain unbounded by the theory.
- Implementations may enforce defensive limits only through
  `resource-exhausted`.
- Diagnostic wording and first-error order remain implementation details.
- Crashes and internal failures carry no logical verdict.
- The normative meanings are specified in
  [`../failure-classes.md`](../failure-classes.md).
