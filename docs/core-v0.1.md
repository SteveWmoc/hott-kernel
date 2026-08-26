# Core v0.1 direction

**Status:** Phase 0 draft. This document records the accepted direction; the
complete inference-rule specification will follow in a dedicated change.

## Intended theory

Core v0.1 is a predicative intensional Martin-Löf type theory with
proof-relevant identity types.

The core will support four principal judgments:

[
Gamma;mathsf{ctx}
]

[
Gamma dash A;mathsf{type}
]

[
Gamma dash t : A
]

[
Gamma dash t equiv u : A
]

The final specification must also state the well-formedness conditions for
every premise and the symmetry, transitivity, congruence, substitution, and
conversion behavior of judgmental equality.

## Initial type formers

The intended Core v0.1 term language contains:

- variables and global declarations;
- universes (mathcal U_i);
- dependent functions: (Pi), lambda abstraction, and application;
- dependent pairs: (Sigma), pairing, and projections;
- identity types: (mathsf{Id}), reflexivity, and (J);
- the empty type and its eliminator;
- the unit type, its point, and its eliminator;
- natural numbers and primitive recursion.

The serialized core will use de Bruijn indices. Surface names are not part of
the kernel theory.

## Universes

Universe levels are explicit natural numbers:

[
mathcal U_i : mathcal U_{i+1}.
]

Core v0.1 universes are predicative and noncumulative. Formation rules for
dependent products and sums compute their result level from the levels of the
domain and codomain. Universe polymorphism and explicit lifting are later
surface-language and library concerns.

## Identity

For (A : mathcal U_i) and (a,b:A),

[
mathsf{Id}_A(a,b) : mathcal U_i.
]

Reflexivity supplies

[
mathsf{refl}_a : mathsf{Id}_A(a,a).
]

The eliminator (J) provides path induction, with a judgmental computation
rule on reflexivity. Identity proofs are not placed in a proof-irrelevant
universe and are not definitionally collapsed.

Core v0.1 contains no special rule for UIP, axiom K, equality reflection,
function extensionality, propositional extensionality, or univalence.

## Judgmental equality

The initial conversion relation is intentionally small. It will include:

- equivalence closure and congruence;
- alpha-equivalence through the binder representation;
- unfolding of transparent global definitions;
- beta computation for dependent functions;
- projection computation for dependent pairs;
- the (J) computation rule at reflexivity;
- computation rules for the empty, unit, and natural-number eliminators.

Core v0.1 will not initially include judgmental eta rules.

## Global declarations

The environment will distinguish:

- transparent definitions, which may unfold during conversion;
- opaque declarations, whose bodies are checked but do not unfold outside
  their defining boundary;
- postulates, which have a checked type but no body.

“Definition” and “theorem” are presentation-level labels. They do not create a
proof-irrelevant class of terms.

## Required completion work

Before implementation, this document must be replaced or supplemented by:

- complete inference rules;
- a precise raw grammar;
- capture-avoiding substitution or an equivalent binder semantics;
- all reduction and congruence rules;
- an algorithmic checking and conversion specification;
- representative derivation trees;
- an explicit account of errors and malformed environments.
