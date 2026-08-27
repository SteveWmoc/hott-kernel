# 0006: Mixed-level dependent products and sums

- **Status:** Accepted for Core v0.1
- **Date:** 2026-08-27

## Decision

Dependent products and sums may have domain and codomain in different universe
levels. Their result lies in the maximum of those levels:

$$
A:\mathcal U_i,\quad B(x):\mathcal U_j
\quad\Longrightarrow\quad
\Pi(x:A).B(x),\;\Sigma(x:A).B(x):\mathcal U_{\max(i,j)}.
$$

## Reason

A same-level rule would require explicit lifts for routine mixed-level
constructions without improving foundational visibility. The maximum rule is
predicative and does not imply universe cumulativity.

## Consequences

- Universe formation is deterministic from the two premise levels.
- Core v0.1 still has no rule promoting an arbitrary
  $A:\mathcal U_i$ to $A:\mathcal U_{i+1}$.
- The checker must compute natural-number maxima when synthesizing dependent
  product and sum types.
- Changing to a same-level, cumulative, or more elaborate algebra of levels
  requires a theory-version change.
