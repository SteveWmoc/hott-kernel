# Metatheory and validation program

## Status

This document records proof obligations and intended strategies. It does not
claim that any metatheorem has yet been proved. Metatheorems justify the
specified rules and algorithms; they are not additional object-theoretic axioms
or kernel inference rules.

## Separation of layers

Core v0.1 has three relevant layers:

1. the declarative judgments define well-formed contexts, typing, and
   judgmental equality;
2. the bidirectional algorithm attempts to decide those judgments;
3. the Rust checker implements the algorithm.

The metatheory must connect these layers. A successful program test is not a
proof that the algorithm decides the declarative theory, and an elegant
declarative calculus is not yet an executable checker.

## Structural obligations

For well-formed environments and contexts, we intend to prove:

- renaming and weakening;
- simultaneous substitution;
- identity and composition laws for substitutions;
- preservation of typing by substitution;
- regularity: if $\Gamma\vdash t:A$, then
  $\Gamma\vdash A:\mathcal U_i$ for some $i$;
- inversion lemmas for every type former;
- uniqueness of declarative typing up to judgmental equality;
- uniqueness of synthesized types up to judgmental equality;
- subject reduction.

Substitution is defined in the core specification. The substitution lemma is a
theorem about that definition, not a rule available to object-language terms.

## Normalization strategy

The target result is strong normalization for well-typed Core v0.1 terms, not
merely weak normalization. The intended proof route is a universe-indexed
logical relation or reducibility interpretation.

At a high level:

1. define reducibility predicates for types and terms, indexed by universe
   level and stable under weakening and substitution;
2. prove closure under every beta-delta-iota reduction and neutral expansion;
3. prove the fundamental theorem: every well-typed term inhabits the
   interpretation of its type;
4. derive strong normalization and subject reduction;
5. obtain uniqueness of normal forms and decidability of conversion;
6. derive canonicity for closed natural-number terms.

Identity types require no proof-irrelevance assumption in this argument.
Neutral identity proofs remain neutral; reflexivity is the canonical
introduction form.

The exact logical relation will be developed alongside a paper proof. We will
not claim strong normalization merely because the implementation terminates on
the test suite.

## Normalization by evaluation

The primary conversion checker will use normalization by evaluation (NbE):

1. evaluate syntax into semantic values and neutrals;
2. unfold transparent globals while leaving opaque globals and postulates
   neutral;
3. quote semantic values back to beta-delta-iota normal forms;
4. compare quoted forms structurally.

Quotation performs no eta expansion in Core v0.1. The NbE correctness
obligations are:

- soundness: quoted equality implies declarative judgmental equality;
- completeness: declaratively equal well-typed terms quote identically;
- stability under environment extension and substitution;
- termination on all well-typed inputs presented by the checker.

An independent checker may use weak-head evaluation followed by structural
conversion or another normalization algorithm, provided it decides the same
specified relation.

## Bidirectional checking

For the synthesis and checking judgments, we intend to prove:

- synthesis soundness;
- checking soundness;
- conversion soundness and completeness;
- annotation completeness: every declaratively typable term admits enough
  annotations to be accepted by the bidirectional checker;
- decidability for well-formed finite environments and raw inputs.

Surface elaboration is not part of these theorems. It is an untrusted producer
of annotated core terms.

## Canonicity

The first canonicity theorem should state:

> If the empty context proves $t:\mathbb N$ in Core v0.1 and the global
> environment contains no postulates or opaque definitions capable of
> producing a natural number, then $t$ reduces to a unique numeral.

The environment qualification matters. An opaque postulate
$c:\mathbb N$ is a closed neutral natural number and would invalidate an
unqualified claim.

Later extension documents must restate the strongest canonicity theorem they
preserve. Axiomatic univalence may block ordinary computation; a computational
cubical account has different obligations.

## Judgmental eta policy

Core v0.1 has no judgmental eta rules for dependent functions or dependent
pairs. There is no commitment to add them later.

Any proposal to add eta requires:

- a new decision record;
- a theory-version change;
- revised normalization and conversion algorithms;
- revised soundness, completeness, and canonicity arguments;
- positive and negative tests distinguishing the two theories.

Propositional principles derivable inside the theory do not silently become
judgmental computation rules.

## Consistency and nontrivial identity

Consistency may first be established relative to a standard model. Because a
set model validates UIP, consistency alone would not demonstrate the intended
proof-relevant character of the syntax.

We therefore also seek a homotopically nontrivial model—initially along the
lines of the groupoid interpretation—in which the Core v0.1 rules hold and UIP
fails. Such a model would show that UIP is not derivable from the specified
rules, assuming the metatheory used to construct the model.

## Extensions and postulates

Each optional extension must document:

- its exact formation, introduction, elimination, and computation rules;
- whether each computation law is judgmental or propositional;
- a semantic consistency account;
- its effects on normalization, decidability, and canonicity;
- its interactions with every previously supported extension;
- additions to the foundation-manifest vocabulary.

The project distinguishes two possible presentations of univalence:

- an opaque term is recorded as a postulate such as
  `postulate.univalence`;
- cubical primitives that make univalence computational are recorded as kernel
  extensions and list their added rules.

Higher inductive types are introduced individually. There is no unqualified
“all HITs” switch.

## Implementation validation

The planned validation layers are:

1. executable positive and negative specification tests;
2. unit tests for shifting, substitution, evaluation, quotation, and
   conversion;
3. property tests for substitution composition and normalization stability;
4. fuzzing of the core parser and malformed explicit terms;
5. deterministic environment, artifact, and manifest hashing;
6. a second independently written checker for the stable core format;
7. differential checking across implementations;
8. paper metatheory with machine formalization where practical.

The independent checker should use a different language and should share only
the published core grammar and theory specification.

## Trusted computing base

Initially, the trusted base consists of:

- the published theory specification;
- the primary core checker;
- the Rust compiler and relevant platform implementation;
- the bytes of the explicit declaration environment being checked.

The parser, surface elaborator, tactics, editor, build scripts, and AI systems
are not trusted for logical validity.

An independent checker reduces, but cannot eliminate, common implementation,
compiler, hardware, and specification assumptions.
