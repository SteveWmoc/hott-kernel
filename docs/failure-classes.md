# Core v0.1 result and failure classes

**Status:** Normative Phase 0 vocabulary.

This document separates logical rejection from format, integrity, audit, and
operational failures. Only one class below asserts that a grammatically valid
term fails a judgment of the selected Core theory.

Implementations may provide richer diagnostic codes beneath these classes.
They may not relabel an inconclusive or nonlogical failure as
`invalid-judgment`.

## Success

`success` means that:

- the selected versions are supported;
- the artifact is well-formed and canonical;
- every declaration is accepted by the selected theory;
- any requested hash and manifest comparisons succeed;
- the implementation completed without exhausting its configured resources.

Tools that check only a subset of these conditions must state the scope of
their success result.

## `unsupported-version`

The input is sufficiently well-formed to expose a format, projection, manifest,
feature-vocabulary, or theory version that the implementation does not support.

This result makes no claim about validity under the identified version.

## `malformed-encoding`

The bytes do not satisfy the lexical, grammatical, arity, string, or module
envelope requirements of the selected format. Examples include:

- invalid UTF-8;
- a forbidden escape;
- a leading-zero natural number;
- an unknown term tag;
- a wrong constructor arity;
- duplicate display names;
- a token after the complete module.

No Core judgment is asserted because no valid raw module was obtained.

## `noncanonical-artifact`

The bytes parse as a permitted transport encoding, but they differ from the
canonical printer's output for the decoded module. Flexible whitespace is the
principal Core v0.1 example.

The tool may emit canonical bytes. This result alone does not assert that the
decoded declarations are well-typed or ill-typed.

## `invalid-judgment`

The artifact is grammatically valid, the relevant versions are supported, and
at least one declaration fails a judgment of the selected Core theory.
Examples include:

- an out-of-scope local variable;
- a forward, self, or out-of-range global reference;
- a declaration type that does not inhabit a universe;
- a body that does not check against its declared type;
- a failed conversion.

This is the only failure class that constitutes logical rejection by Core
v0.1. A diagnostic should identify the declaration index and failed checking
operation when practical, but diagnostic wording is not normative.

## `artifact-hash-mismatch`

The SHA-256 hash of the canonical `hott-core` bytes differs from a supplied or
manifested artifact hash.

This is an integrity failure. It does not override a kernel result obtained for
the actual bytes.

## `semantic-hash-mismatch`

The SHA-256 hash of the canonical `hott-semantic` projection differs from a
supplied or manifested semantic hash.

This is an integrity or identity failure. It does not establish that the
artifact is ill-typed.

## `manifest-mismatch`

The deterministic fields of a schema-valid manifest differ from the fields
recomputed from the checked artifact. Examples include incorrect dependency
sets, declaration kinds, or feature identifiers.

This is an audit failure. Asserted provenance is not recomputable and is
excluded from this comparison.

## `resource-exhausted`

The implementation did not finish because a configured or physical resource
limit was reached, including limits on:

- input bytes;
- allocation;
- term or recursion depth;
- natural-number size;
- evaluation steps;
- wall-clock time.

This result is inconclusive. It must never be converted to
`malformed-encoding` or `invalid-judgment` merely because another conforming
implementation could process more input.

## Multiple defects

An artifact may have more than one defect, and resource exhaustion may occur at
any stage. Core v0.1 does not standardize which independently discoverable
failure is reported first. A tool must report the correct class for the defect
it does report.

In particular, an implementation may reject noncanonical bytes before type
checking or may canonicalize them in a separate operation. It may not claim
`success` for artifact verification while silently changing the bytes.

## Implementation failures

A crash, violated internal invariant, unavailable cryptographic primitive, or
other implementation defect is not one of the normative input-failure classes
and carries no logical verdict. Implementations should report such failures
distinctly rather than presenting them as user input errors.
