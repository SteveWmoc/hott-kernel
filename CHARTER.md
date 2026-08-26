# Project charter

## Mission

`hott-kernel` will be a small, auditable formalizer for proof-relevant
dependent type theory and homotopy type theory.

Its central commitment is:

> **Nothing foundational is implicit.**

The project will expose the inference rules accepted by the kernel, distinguish
those rules from optional extensions and postulates, and report the transitive
foundational dependencies of checked declarations.

## Governing principles

### 1. The foundation is a versioned public interface

The kernel theory is specified independently of its implementation. A release
must identify the exact version of the theory it checks. A change to a typing,
conversion, computation, universe, or elimination rule is a foundational
change, not an implementation detail.

### 2. Proof relevance is the default

Core identity types live in data-relevant universes. The kernel does not
definitionally identify proofs of the same proposition and does not validate
UIP or axiom K by a special rule. There is no distinguished proof-irrelevant
`Prop` universe in Core v0.1.

### 3. Different kinds of assumptions remain different

The project distinguishes:

- **kernel rules**: primitive judgments and computation principles;
- **extensions**: optional additions such as univalence, truncation, higher
  inductive types, or cubical composition;
- **postulates**: opaque assumed terms such as excluded middle or choice;
- **declaration dependencies**: previously checked definitions and theorems;
- **generation provenance**: tactics, external solvers, AI systems, or humans
  that produced a candidate term.

These categories must not be collapsed into a single misleading word such as
“axioms.”

### 4. The trusted core stays small

The parser, surface elaborator, tactic system, editor integration, and external
automation are untrusted. They may propose fully explicit terms, but only the
core checker can accept a declaration.

The primary kernel will be implemented in safe Rust with unsafe code forbidden.
A stable serialized core format will permit independent checkers.

### 5. Convenience does not silently change the theory

An implementation shortcut may not add equality reflection, proof irrelevance,
extensionality, universe cumulativity, new computation laws, or additional
elimination power without an accepted decision record and a theory-version
change.

### 6. Foundational dependencies are transitive and reproducible

Every accepted declaration will eventually carry a deterministic foundation
manifest recording its kernel version, primitive features, optional extensions,
postulates, and imported declarations.

Generated metadata is not a substitute for rechecking the explicit term.

### 7. Authorship is not part of the trusted computing base

Human-written and AI-assisted contributions are evaluated by the same formal
specification, review standards, and tests. Material use of generators should
be recorded as provenance, while validity rests solely on successful checking.

## Core v0.1 scope

The initial theory is intended to contain:

- explicit predicative universes;
- dependent function types;
- dependent pair types;
- proof-relevant identity types and the J eliminator;
- empty, unit, and natural-number types;
- transparent and opaque global declarations;
- a deliberately small judgmental equality.

The initial theory does not contain:

- an impredicative or proof-irrelevant proposition universe;
- proof irrelevance, UIP, or axiom K;
- equality reflection;
- function or propositional extensionality;
- quotients or truncations;
- univalence;
- higher inductive types;
- excluded middle or choice;
- unrestricted recursion.

Optional principles may later be added as separately named and audited
extensions.

## Initial non-goals

Phase 0 does not attempt to provide:

- compatibility with Lean, Mathlib, or another proof assistant;
- a general-purpose programming language;
- code extraction;
- a complete inductive-family mechanism;
- powerful tactics, typeclasses, or automation;
- a mature mathematical library;
- computational cubical type theory.

## Decision process

A change affecting the foundation or trusted boundary requires:

1. a numbered record in `docs/decisions/`;
2. a precise statement of the proposed rule or boundary change;
3. examples that distinguish the old and new behavior;
4. an account of metatheoretic and audit consequences;
5. an explicit decision before implementation.

Accepted foundational decisions are immutable historical records. They may be
superseded by a later record, not silently edited into a different decision.

## Meaning of “checked”

A declaration is checked only when a kernel conforming to the identified theory
version accepts its fully explicit core term in a previously checked
environment. Parsing, elaboration, successful code generation, or acceptance by
an external solver is not sufficient.

## Phase 0 exit condition

Kernel implementation begins only after the project has:

- specified every Core v0.1 judgment and inference rule;
- specified judgmental equality and all computation rules;
- fixed the universe and transparency policies;
- listed representative accepted and rejected judgments;
- defined the first foundation-manifest schema;
- documented the trusted computing base;
- completed an implementability review of the specification.
