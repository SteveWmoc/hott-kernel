# Core v0.1 implementability review

**Status:** Complete; Core v0.1 frozen for implementation.

**Review baseline:** repository `main` at
`4191294b96ff45d4b654491fdfc6f3bc5af368dc`.

**Reviewed candidate:** `841be2681abdf5ea3b3fa21c66c76a37ccbc73ac`.

**Freeze record:** [Decision 0012](decisions/0012-freeze-core-v0.1.md).

## 1. Question and standard

This review asks whether two independent implementers can build checkers for
the published `mltt-core/0.1` theory and agree on parsing, typing, conversion,
declaration validation, and audit extraction without making an unrecorded
foundational choice.

The review passes only if every core constructor and declaration kind has:

- fixed syntax and binding behavior;
- a declarative role;
- a deterministic algorithmic treatment;
- fixed conversion behavior;
- an exact serialized form;
- deterministic audit extraction;
- at least one representative conformance witness.

An operational detail may be clarified without changing the theory. A choice
that would change which judgments are accepted is not a clarification: it
requires a foundational decision and a new theory version. No such blocker was
found.

“Implementable” does not mean “proved.” The checker can now be written without
inventing rules. Soundness, completeness, normalization, confluence,
decidability, canonicity, and semantic results remain explicit metatheoretic
obligations.

## 2. Reviewed specification

The review covers these mutually supporting contracts:

- the [project charter](../CHARTER.md);
- the [Core v0.1 calculus](core-v0.1.md);
- the [core interchange format](core-format.md);
- the [foundation manifest](foundation-manifest-v0.1.md) and
  [audit model](audit-model.md);
- the [result and failure classes](failure-classes.md);
- the [metatheory program](metatheory.md);
- accepted decisions through Decision 0011;
- the specification, format, and conformance fixtures under `tests/`.

The version identities remain:

| Contract | Version |
| --- | --- |
| Theory | `mltt-core/0.1` |
| Text transport | `hott-core/0.1` |
| Semantic projection | `hott-semantic/0.1` |
| Foundation manifest | `hott-foundation-manifest/0.1` |
| Feature vocabulary | `mltt-core-features/0.1` |

This review changes none of those versions.

## 3. Constructor crosswalk

The raw term grammar has exactly 23 constructors. The next two tables form one
crosswalk: the first follows each constructor through the calculus and
algorithm; the second follows it through serialization, audit extraction, and
testing.

### 3.1 Calculus and algorithm

| Constructor | Binding | Declarative role | Bidirectional treatment | Conversion behavior |
| --- | --- | --- | --- | --- |
| `var n` | none | local variable rule, Section 4 | synthesize the cutoff-shifted context entry | neutral |
| `global n` | none | earlier-global rule, Sections 2 and 4 | synthesize the type of an earlier declaration | unfold at head iff transparent; otherwise neutral |
| `universe i` | none | predicative universe formation, Section 5 | synthesize `universe (i+1)` | visible constructor; distinct levels do not convert |
| `pi A B` | `B` binds one | dependent-product formation, Section 6 | infer both universe levels and synthesize their maximum | visible type former; no eta rule |
| `lam t` | `t` binds one | dependent-function introduction, Section 6 | checking only, after exposing the expected `pi` | beta-reduces when applied; no eta expansion |
| `app f a` | none | dependent-function elimination, Section 6 | synthesize `f`, expose `pi`, check `a`, substitute in codomain | beta at a lambda head; otherwise a neutral application |
| `sigma A B` | `B` binds one | dependent-pair formation, Section 7 | infer both universe levels and synthesize their maximum | visible type former; no eta rule |
| `pair a b` | none | dependent-pair introduction, Section 7 | checking only, after exposing the expected `sigma` | selected by projections; no pair eta |
| `fst p` | none | first projection, Section 7 | synthesize `p`, expose `sigma`, return its domain | `fst (pair a b)` reduces to `a`; otherwise neutral |
| `snd p` | none | dependent second projection, Section 7 | synthesize `p`, expose `sigma`, substitute `fst p` in its codomain | `snd (pair a b)` reduces to `b`; otherwise neutral |
| `id A a b` | none | identity formation, Section 8 | infer the literal universe of `A`, then check both endpoints | visible type former; proof relevance retained |
| `refl a` | none | identity introduction, Section 8 | synthesize the type of `a` and its reflexive identity type | visible constructor and the `J` computation pattern |
| `j A a C d b p` | none | based identity elimination, Section 8 | validate `A` and `a`, expose the exact motive, check branch, endpoint, and path | reduces to `d` when the validated path exposes `refl` |
| `empty` | none | empty-type formation, Section 9 | synthesize `universe 0` | visible type former |
| `empty-elim C e` | none | empty elimination, Section 9 | expose an `empty`-indexed universe motive, then check `e` | no constructor reduction; neutral when `e` is neutral |
| `unit` | none | unit-type formation, Section 9 | synthesize `universe 0` | visible type former |
| `star` | none | unit introduction, Section 9 | synthesize `unit` | visible constructor and unit-elimination pattern |
| `unit-elim C c u` | none | unit elimination, Section 9 | expose a `unit`-indexed universe motive and check branch and scrutinee | reduces to `c` when `u` exposes `star` |
| `nat` | none | natural-number formation, Section 10 | synthesize `universe 0` | visible type former |
| `zero` | none | natural-number introduction, Section 10 | synthesize `nat` | visible constructor and zero-elimination pattern |
| `succ n` | none | natural-number introduction, Section 10 | check `n` against `nat`, then synthesize `nat` | visible constructor and successor-elimination pattern |
| `nat-elim C z s n` | none | dependent natural-number elimination, Section 10 | expose a `nat`-indexed universe motive and check both branches and scrutinee | separate iota reductions at `zero` and `succ` |
| `ann t A` | none | checked annotation, Section 11 | infer the universe of `A`, check `t`, then synthesize `A` | erases at the head; adds no equality rule |

### 3.2 Serialization, audit, and witnesses

“Feature” is the direct constructor contribution. Child terms are always
scanned, and `global` also contributes its referenced declaration index.

| Constructor | Canonical `hott-core` form | Audit feature | Positive witness | Distinguishing rejection |
| --- | --- | --- | --- | --- |
| `var n` | `(var NAT)` | none | `dependent-lookup.core`, `nested-substitution.core` | `out-of-scope-variable.core` |
| `global n` | `(global NAT)` | none + declaration dependency | `declaration-kinds.core`, `transparent-delta.core` | `forward-reference.core`, `opaque-no-delta.core` |
| `universe i` | `(universe NAT)` | `universe` | `dependent-lookup.core`, `mixed-universes.core` | `universe-noncumulative.core` |
| `pi A B` | `(pi TERM TERM)` | `pi` | `dependent-lookup.core`, `mixed-universes.core` | `no-pi-eta.core` |
| `lam t` | `(lam TERM)` | `pi` | `beta.core`, `nested-substitution.core` | `bare-lambda-synthesis.core`, `no-pi-eta.core` |
| `app f a` | `(app TERM TERM)` | `pi` | `beta.core` | `bare-lambda-synthesis.core` |
| `sigma A B` | `(sigma TERM TERM)` | `sigma` | `mixed-universes.core`, `pair-projections.core` | `no-sigma-eta.core` |
| `pair a b` | `(pair TERM TERM)` | `sigma` | `pair-projections.core` | `bare-pair-synthesis.core`, `no-sigma-eta.core` |
| `fst p` | `(fst TERM)` | `sigma` | `pair-projections.core` | `bare-pair-synthesis.core` |
| `snd p` | `(snd TERM)` | `sigma` | `pair-projections.core` | `no-sigma-eta.core` |
| `id A a b` | `(id TERM TERM TERM)` | `identity` | all positive computation witnesses | `false-refl.core`, `no-uip.core` |
| `refl a` | `(refl TERM)` | `identity` | all positive computation witnesses | `false-refl.core`, `no-uip.core` |
| `j A a C d b p` | `(j TERM TERM TERM TERM TERM TERM)` | `identity` | `j-refl.core` | the three `j-*.core` rejected fixtures |
| `empty` | `empty` | `empty` | `empty-neutral.core` | — |
| `empty-elim C e` | `(empty-elim TERM TERM)` | `empty` | `empty-neutral.core` | `empty-no-computation.core` |
| `unit` | `unit` | `unit` | `unit-elim-star.core` | `bad-body.core`, `unit-motive-domain.core` |
| `star` | `star` | `unit` | `unit-elim-star.core` | `unit-no-uniqueness.core` |
| `unit-elim C c u` | `(unit-elim TERM TERM TERM)` | `unit` | `unit-elim-star.core` | `bare-motive.core`, `unit-motive-domain.core` |
| `nat` | `nat` | `natural-numbers` | both `nat-elim-*.core` fixtures | `bad-body.core` |
| `zero` | `zero` | `natural-numbers` | `nat-elim-zero.core` | `false-refl.core`, `j-wrong-branch.core` |
| `succ n` | `(succ TERM)` | `natural-numbers` | `nat-elim-succ.core` | `false-refl.core`, `j-path-mismatch.core` |
| `nat-elim C z s n` | `(nat-elim TERM TERM TERM TERM)` | `natural-numbers` | both `nat-elim-*.core` fixtures | — |
| `ann t A` | `(ann TERM TERM)` | none | `annotation-erasure.core` and every annotated motive | `bare-motive.core` |

The exact tags, natural-number grammar, string grammar, arities, whitespace,
and final line feed remain governed by `core-format.md`; the table is a
cross-check, not a second grammar.

## 4. Declaration-kind crosswalk

| Kind | Validation | Conversion after insertion | Serialized form | Audit treatment | Witness |
| --- | --- | --- | --- | --- | --- |
| postulate | closed declared type inhabits a universe; no body | neutral | `(postulate STRING TERM)` | scan type; kind is retained; references add a postulate dependency | `declaration-kinds.core` |
| transparent | closed type inhabits a universe and body checks at that type | body unfolds during delta reduction | `(transparent STRING TERM TERM)` | scan type and body; kind retained | `transparent-delta.core` |
| opaque | closed type inhabits a universe and body checks at that type | always neutral | `(opaque STRING TERM TERM)` | scan type and checked body; kind retained; never operationally unfold for extraction | `declaration-kinds.core`, `opaque-no-delta.core` |

Declarations are checked sequentially in an initially empty local context.
The current declaration is absent until all its checks succeed. Therefore only
strictly earlier `global` indices resolve; the positive backward reference and
negative forward reference distinguish this contract.

## 5. Computation crosswalk

Core v0.1 has exactly these nine head reductions. Positive identity witnesses
are constructed so reflexivity checks only if the named reduction occurs.

| Reduction | Head condition | Result | Witness |
| --- | --- | --- | --- |
| beta | `app` function exposes `lam t` | capture-avoiding `t[a/x]` | `beta.core`, `nested-substitution.core` |
| first projection | `fst` principal exposes `pair a b` | `a` | `pair-projections.core` |
| second projection | `snd` principal exposes `pair a b` | `b` | `pair-projections.core` |
| identity iota | validated `J` path exposes `refl` | branch `d` | `j-refl.core` |
| unit iota | unit scrutinee exposes `star` | branch `c` | `unit-elim-star.core` |
| natural zero iota | natural scrutinee exposes `zero` | zero branch `z` | `nat-elim-zero.core` |
| natural successor iota | natural scrutinee exposes `succ n` | `s n (nat-elim C z s n)` | `nat-elim-succ.core` |
| annotation erasure | head is `ann t A` | `t` | `annotation-erasure.core` |
| transparent delta | head is a transparent `global n` | its checked body | `transparent-delta.core` |

No other head reduction is permitted. In particular, empty elimination has no
constructor case; opaque definitions and postulates do not unfold; functions
and pairs are not eta-expanded; identity proofs are not collapsed.

## 6. Algorithmic clarification ledger

The audit exposed implementation details that were conventional but not yet
stated as executable contracts. Each resolution is representation- or
strategy-level and preserves the declarative judgments.

| Finding | Resolution | Distinguishing evidence | Theory effect |
| --- | --- | --- | --- |
| IR-01: de Bruijn weakening and substitution lacked exact equations | define cutoff-aware shifting, lifted simultaneous substitution, top substitution, and shifted lookup in Sections 3–4 | `dependent-lookup.core`, `nested-substitution.core` | none |
| IR-02: “expose a type” did not fix a head strategy | define deterministic WHNF, visible and neutral heads, `exposePi`, and `exposeSigma` in Section 15.1 | all computation fixtures; eta and opacity rejections | none |
| IR-03: eliminator motive recognition could be mistaken for metavariable search | define exact unary and `J` motive decomposition and literal universe extraction | annotated positive motives and motive-shape rejections | none |
| IR-04: conversion's input precondition was implicit | state that normalization and conversion receive only validated types or already well-typed terms | algorithm invariant in Section 15 | none |
| IR-05: environment construction needed an executable order | define one-pass declaration checking with insertion only after success | `declaration-kinds.core`, `forward-reference.core` | none |
| IR-06: semantic fixtures and byte fixtures risked being conflated | separate `tests/conformance/` from `tests/format/` and state each result contract | both fixture READMEs | none |
| IR-07: uniqueness of normal forms was listed without an explicit confluence obligation | add confluence and its intended Church-Rosser proof route to the metatheory program | explicit open obligation | none |

The deterministic reference procedures do not forbid a conforming checker from
using NbE, explicit substitution, different internal context storage, or other
optimizations. An alternative must decide the same specified judgments and
produce the same deterministic artifact and audit data.

## 7. Conformance inventory

The inventory and per-file expectations are in
[`tests/conformance/README.md`](../tests/conformance/README.md). It contains 13
accepted and 19 rejected canonical modules.

The suite covers:

- all 23 term constructors and all three declaration kinds;
- dependent lookup, nested binders, and capture-avoiding substitution;
- mixed-level product and sum formation without cumulativity;
- annotations in every synthesis-critical position;
- all nine computation rules;
- correct and incorrect unary and identity motives;
- transparent unfolding and opaque neutrality;
- sequential global visibility and closed declaration boundaries;
- rejection of judgmental function eta, pair eta, UIP, and equality reflection.

These fixtures do not prove algorithmic soundness or completeness. They make
the frozen distinctions executable and give independently written checkers a
common finite regression corpus.

## 8. Charter exit review

| Phase 0 exit condition | Evidence | Result |
| --- | --- | --- |
| every Core v0.1 judgment and inference rule specified | `core-v0.1.md`, Sections 2–12 | complete |
| judgmental equality and computation specified | `core-v0.1.md`, Sections 13–15; Section 5 above | complete |
| universe and transparency policies fixed | Core Sections 5 and 12 and the accepted foundational decisions | complete |
| representative accepted and rejected judgments listed | `tests/specification/` and `tests/conformance/` | complete |
| first foundation-manifest schema defined | manifest specification and checked JSON Schema | complete |
| trusted computing base documented | charter, audit model, and metatheory document | complete |
| implementability review completed | this review and constructor crosswalk | complete |

## 9. Still-open metatheory

The following remain unproved and must not be implied by a Phase 0 freeze:

- weakening, renaming, simultaneous substitution, and substitution
  composition;
- regularity, inversion, subject reduction, and uniqueness of typing up to
  judgmental equality;
- strong normalization and confluence for well-typed beta-delta-iota
  reduction, hence uniqueness of normal forms;
- soundness and completeness of algorithmic conversion and bidirectional
  checking, including annotation completeness;
- decidability and termination of checking on every well-formed finite input;
- canonicity for closed natural numbers under the stated environment
  qualification;
- consistency relative to a documented model;
- a homotopically nontrivial model showing that the rules do not force UIP;
- correctness of NbE, manifest extraction, hashing, and each concrete checker
  implementation with respect to the frozen specifications.

The [metatheory program](metatheory.md) owns the intended proof strategies.
Implementation tests, this finite conformance suite, and agreement between two
checkers are evidence, not substitutes for those proofs.

## 10. Conclusion

Every Core v0.1 constructor, declaration kind, computation rule, serialized
form, and audit contribution has an implementation path with distinguishing
fixtures. The clarifications found during review preserve the accepted theory.
No unresolved design choice blocks an independent implementation.

Decision 0012 ties this review to the candidate commit above. Core v0.1
satisfies the charter's Phase 0 exit condition and is frozen for
implementation.
