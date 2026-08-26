# Metatheory and validation program

## Status

This document lists obligations and intended validation methods. It does not
claim that any metatheorem has yet been proved.

## Core metatheoretic obligations

For Core v0.1 we intend to establish:

- context and type well-formedness invariants;
- weakening;
- substitution;
- uniqueness of inferred types up to judgmental equality;
- preservation of typing under reduction;
- decidability of algorithmic type checking;
- decidability of algorithmic conversion;
- normalization for the initial core;
- canonicity for closed natural-number terms;
- consistency relative to a documented model.

Because the theory is proof-relevant, a homotopically nontrivial model is also
important for demonstrating that UIP is not silently forced by the specified
rules. This is distinct from unit tests and from consistency alone.

## Axiomatic extensions

Opaque axioms such as univalence may block ordinary computation and alter
canonicity statements. Each extension must document:

- its rule or constant signature;
- known semantic models;
- expected effects on normalization and canonicity;
- whether its computation rules are judgmental or propositional;
- interactions with every previously accepted extension.

## Computational cubical track

Cubical type theory is a later, separately versioned theory. It should not be
presented as a performance optimization or silently substituted for axiomatic
HoTT. Its interval, cofibration, composition, filling, glue, and higher
inductive computation rules will require their own specification and
metatheory.

## Implementation validation

The planned validation layers are:

1. executable positive and negative specification tests;
2. unit and property tests for substitution, evaluation, and conversion;
3. fuzzing of parsers and explicit core terms;
4. deterministic environment and manifest hashing;
5. a second independently written checker for the stable core format;
6. differential checking across implementations;
7. paper metatheory with machine formalization where practical.

## Trusted computing base

Initially, the trusted base consists of:

- the published theory specification;
- the primary core checker;
- the Rust compiler and relevant platform implementation;
- the bytes of the explicit declaration environment being checked.

The parser, elaborator, tactics, editor, build scripts, and AI systems are not
trusted for logical validity.

An independent checker in a different implementation language will reduce,
but cannot eliminate, common implementation and compiler assumptions.
