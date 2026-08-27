# 0010: Freeze foundation manifest v0.1

- **Status:** Accepted for manifest schema v0.1
- **Date:** 2026-08-27

## Decision

One JSON foundation manifest covers one complete Core artifact and contains one
ordered audit record per declaration. It records the transport, semantic
projection, theory, feature-vocabulary, artifact hash, and semantic hash
versions explicitly.

Each declaration has direct and transitive sets of kernel features, extensions,
postulates, and declaration dependencies. These sets are extracted
structurally from constructor occurrences and global references, not from a
checker's reduction or normalization trace.

Deterministic audit fields are separated from asserted generation provenance.

## Reason

A module-level document is practical for streaming, storage, and independent
verification while retaining declaration-level accountability. Structural
extraction makes conforming checkers agree even when their evaluation
algorithms differ. Separating provenance prevents historical claims from being
mistaken for recomputable logical dependencies.

## Consequences

- Dependency arrays are deduplicated and canonically ordered.
- Transitive records include direct dependencies and cached closures of
  referenced earlier declarations.
- Referencing an opaque or postulate global unions its recorded feature sets
  without unfolding it.
- An opaque declaration's own checked body contributes to its own audit record.
- Core v0.1 extension arrays are empty but remain categorically separate.
- JSON Schema validates shape; additional ordering and closure constraints are
  normative prose obligations.
- The schema and extraction algorithm are specified in
  [`../foundation-manifest-v0.1.md`](../foundation-manifest-v0.1.md).
