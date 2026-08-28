> **Nothing foundational is implicit.**

# hott-kernel

A small, auditable homotopy type theory formalizer with a proof-relevant
kernel and explicit tracking of foundational rules, axioms, and extensions.

## Status

This repository is in **Phase 0: specification**. There is intentionally no
implementation yet. The formal system, trusted boundary, and audit vocabulary
will be fixed before kernel code is written.

The repository now contains a normative candidate for the Core v0.1 calculus,
including its declarative judgments, computation rules, bidirectional checking
algorithm, and conversion relation. The Core v0.1 interchange, semantic-hash,
manifest, and failure-class formats are frozen specifications. The theory
remains a candidate until the Phase 0 implementability review is complete.

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
- [Foundational decisions](docs/decisions/)
- [Accepted specification examples](tests/specification/accepted.md)
- [Rejected specification examples](tests/specification/rejected.md)
- [Exact format fixtures](tests/format/)

## Roadmap

1. Freeze the Core v0.1 judgments, rules, formats, and audit schema.
2. Implement a small safe-Rust checker for fully explicit core terms.
3. Add a surface elaborator, modules, and deterministic foundation manifests.
4. Develop path algebra, equivalences, and homotopy levels.
5. Add univalence and selected higher inductive types as auditable extensions
   or postulates, according to their exact presentation.
6. Build an independent checker and begin a separate computational cubical
   track.

Compatibility with Lean or Mathlib, general-purpose programming, powerful
automation, and a large standard library are not Phase 0 goals.

## License

This project is available under the [MIT License](LICENSE).
