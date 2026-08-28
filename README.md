> **Nothing foundational is implicit.**

# hott-kernel

A small, auditable homotopy type theory formalizer with a proof-relevant
kernel and explicit tracking of foundational rules, axioms, and extensions.

## Status

**Phase 0 is complete.** Core v0.1 is frozen for implementation by
[Decision 0012](docs/decisions/0012-freeze-core-v0.1.md). Its declarative
judgments, computation rules, bidirectional algorithm, conversion relation,
interchange format, semantic projection, foundation manifest, and failure
classes are now versioned implementation contracts.

There is intentionally no kernel implementation yet. Implementation may now
begin against the frozen specification and conformance fixtures; code does not
silently supersede either one.

## Purpose

Most proof assistants can report named axioms used by a declaration, but their
reports necessarily presuppose the rules built into the kernel. `hott-kernel`
makes both layers visible. Its long-term goal is to check proof-relevant
dependent type theory while producing a foundation manifest for every checked
declaration.

The initial core is a small, predicative, intensional Martin-Löf type theory
with:

- dependent function types;
- dependent pair types;
- proof-relevant identity types;
- explicit, noncumulative universes;
- empty, unit, and natural-number types;
- no proof-irrelevant `Prop`;
- no choice, excluded middle, extensionality, univalence, or higher inductive
  types unless they are introduced explicitly and reported.

## Principles

- Kernel rules, extension rules, postulates, and proof-generation provenance
  are distinct categories.
- No foundational principle is added merely for convenience.
- Automation is untrusted and must emit a term checked by the kernel.
- Proofs are data; the core does not erase or definitionally identify them.
- Foundational changes require a recorded design decision and a theory-version
  change.
- AI-assisted contributions are welcome and disclosed. Trust comes from the
  specification, the checker, tests, and independent validation—not from the
  identity of the term's author.

See the [project charter](CHARTER.md) for the governing commitments and the
[glossary](docs/glossary.md) for the project's normative vocabulary.

## Phase 0 documents

- [Core v0.1 calculus](docs/core-v0.1.md)
- [Core interchange format](docs/core-format.md)
- [Foundation manifest v0.1](docs/foundation-manifest-v0.1.md)
- [Foundation audit model](docs/audit-model.md)
- [Result and failure classes](docs/failure-classes.md)
- [Metatheory and validation program](docs/metatheory.md)
- [Core v0.1 implementability review](docs/implementability-review-v0.1.md)
- [Foundational decisions](docs/decisions/)
- [Accepted specification examples](tests/specification/accepted.md)
- [Rejected specification examples](tests/specification/rejected.md)
- [Exact format fixtures](tests/format/)
- [Typing and conversion conformance fixtures](tests/conformance/)

## Roadmap

1. Freeze the Core v0.1 judgments, rules, formats, and audit schema.
   **Complete.**
2. Implement the Core AST, strict parser, canonical printer, and byte-for-byte
   round-trip tests in safe Rust.
3. Implement the bidirectional checker, conversion, and deterministic
   foundation manifests in safe Rust.
4. Add a surface elaborator and modules.
5. Develop path algebra, equivalences, and homotopy levels.
6. Add univalence and selected higher inductive types as auditable extensions
   or postulates, according to their exact presentation.
7. Build an independent checker and begin a separate computational cubical
   track.

Compatibility with Lean or Mathlib, general-purpose programming, powerful
automation, and a large standard library are not Phase 0 goals.

## License

This project is available under the [MIT License](LICENSE).
