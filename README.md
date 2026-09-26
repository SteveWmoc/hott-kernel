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

**Roadmap item 3 is complete.** The safe-Rust format layer implements the Core
AST, strict parser, canonical and semantic printers, and the frozen SHA-256
artifact and semantic identities. The public checker validates complete Core
v0.1 modules in one forward pass using the frozen bidirectional typing,
motive-recognition, and beta-delta-iota conversion rules. The 13 accepted and 19
rejected logical conformance modules are executable checker regressions.
Deterministic structural foundation-audit extraction reports direct and
transitive kernel-feature, postulate, and declaration dependencies for accepted
modules. The crate combines those checked audit records with the frozen artifact
identities and emits complete `hott-foundation-manifest/0.1` JSON with an empty
asserted-provenance array, either from a checked module or directly from canonical
Core artifact bytes without silently canonicalizing the input. A separate strict
manifest parser validates UTF-8 and JSON encoding, the frozen schema, decoded
Unicode strings, duplicate keys, canonical set ordering, backward dependency
indices, and asserted-provenance shape/order without treating untrusted input as
recomputed audit data. A byte-level verifier requires canonical Core bytes,
recomputes both hashes and the checked structural audit, and compares the supplied
deterministic manifest fields while preserving asserted provenance only for
separate reporting.

**Surface v0.1 base implementation is complete.** The named AST and strict
dependency-free parser implement the frozen lexical and grammatical contract.
Deterministic elaboration resolves nearest local binders and earlier global
declarations into explicit Core de Bruijn/global indices, rejects unknown and
duplicate global names, and translates all Surface constructors structurally.
The byte-level compiler emits deterministic canonical Core v0.1 artifacts
without invoking the checker, while a separate checked wrapper delegates
logical validity exactly to the existing frozen Core checker before emitting
the same canonical bytes. Surface v0.1 still intentionally has no imports,
implicit arguments, metavariables, tactics, or richer notation. See
[Surface v0.1](docs/surface-v0.1.md).

**Roadmap item 5 is underway.** The checked HoTT library now begins with
universe-zero path inversion derived solely from the Core identity eliminator
`J`, together with its reflexivity computation law. The library layer is
ordinary untrusted Surface source: it introduces no new kernel rule, postulate,
or extension. See [library/README.md](library/README.md).

## Release compatibility

The crate is versioned `0.1.0`. Its first release line implements this exact
compatibility envelope:

| Layer | Version |
| --- | --- |
| Kernel theory | `mltt-core/0.1` |
| Core text transport | `hott-core/0.1` |
| Semantic projection | `hott-semantic/0.1` |
| Surface format | `hott-surface/0.1` |
| Foundation manifest | `hott-foundation-manifest/0.1` |
| Feature vocabulary | `mltt-core-features/0.1` |
| Rust toolchain / minimum Rust | `1.98.1` |

These versioned contracts have compatibility rules independent of the Cargo
package version. In particular, the frozen Core theory and serialized formats
must not change silently. The Rust library API is still pre-1.0 and may evolve
without implying a change to the accepted theory.

See [CHANGELOG.md](CHANGELOG.md) for release notes and
[RELEASING.md](RELEASING.md) for the release procedure.

## Build and validation

The repository pins Rust through `rust-toolchain.toml`, including `rustfmt`
and Clippy. The release checks are:

```text
cargo fmt --all --check
cargo clippy --all-targets --locked -- -D warnings
cargo test --all-targets --locked
RUSTDOCFLAGS="-D warnings" cargo doc --no-deps --locked
cargo package --locked
```

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
- [Surface v0.1](docs/surface-v0.1.md)
- [Foundational decisions](docs/decisions/)
- [Accepted specification examples](tests/specification/accepted.md)
- [Rejected specification examples](tests/specification/rejected.md)
- [Exact format fixtures](tests/format/)
- [Typing and conversion conformance fixtures](tests/conformance/)

## Roadmap

1. Freeze the Core v0.1 judgments, rules, formats, and audit schema.
   **Complete.**
2. Implement the Core AST, strict parser, canonical printer, and byte-for-byte
   round-trip tests in safe Rust. **Complete.**
3. Implement the bidirectional checker, conversion, and deterministic
   foundation manifests in safe Rust. **Complete.**
4. Add a surface elaborator and modules. **Surface v0.1 base implementation
   complete; imports and richer surface conveniences remain later-version work.**
5. Develop path algebra, equivalences, and homotopy levels.
6. Add univalence and selected higher inductive types as auditable extensions
   or postulates, according to their exact presentation.
7. Build an independent checker and begin a separate computational cubical
   track.

Compatibility with Lean or Mathlib, general-purpose programming, powerful
automation, and a large standard library are not Phase 0 goals.

## License

This project is available under the [MIT License](LICENSE).
