# 0002: Explicit predicative universes

- **Status:** Accepted for Core v0.1
- **Date:** 2026-08-26

## Decision

Core v0.1 uses an explicit hierarchy of predicative universes indexed by
natural numbers:

$$
\mathcal U_i : \mathcal U_{i+1}.
$$

The hierarchy is initially noncumulative.

## Reason

Explicit noncumulative levels make universe dependencies visible and keep the
initial checking and conversion rules small. Predicativity excludes
type-in-type and an impredicative proposition universe from the core.

## Consequences

- Universe-polymorphic surface declarations are elaborator work.
- Moving a type between levels requires an explicit, later-specified
  construction rather than silent cumulativity.
- Product and sum formation rules must specify their resulting level.
- Any future cumulativity rule is a foundational change.
