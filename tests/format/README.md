# Core format v0.1 fixtures

These fixtures supply exact byte-level examples for the normative
[core-format](../../docs/core-format.md),
[manifest](../../docs/foundation-manifest-v0.1.md), and
[failure-class](../../docs/failure-classes.md) specifications.

## Canonical artifacts and projections

`canonical/` contains complete `hott-core` artifacts. `semantic/` contains the
exact name-free projection bytes used to compute semantic hashes.

[`expected-hashes.json`](expected-hashes.json) records the expected SHA-256
values. Hashes apply to the checked-in bytes, including each file's final line
feed.

The important comparisons are:

- `identity-u0.core` and `identity-u0-renamed.core` have different artifact
  hashes but share `identity-u0.semantic` and its semantic hash;
- `unit-transparent.core` and `unit-opaque.core` differ only in declaration
  kind and consequently have different artifact and semantic hashes.

## Noncanonical transport input

`noncanonical/identity-whitespace.input` is parseable transport syntax but is
not a canonical artifact. Its canonical output is byte-for-byte
`canonical/identity-u0.core`. Artifact verification reports
`noncanonical-artifact` before rewriting it.

## Malformed inputs

Files under `malformed/` each require `malformed-encoding`:

- `forbidden-escape.core` uses the forbidden display-name escape `\n`;
- `leading-zero.core` spells a natural number as `00`;
- `wrong-arity.core` supplies two arguments to `var`;
- `duplicate-name.core` repeats a decoded display name;
- `trailing-token.core` contains a token after the complete module.

`invalid-utf8.hex` is an ASCII hexadecimal description rather than a Core
artifact. Removing spaces and decoding hexadecimal produces a byte sequence
whose theory-name string contains the overlong UTF-8 bytes `c0 af`. A future
test harness should decode the description before presenting it to the parser.

## Grammatically valid but logically invalid inputs

Files under `invalid-judgment/` parse successfully but require
`invalid-judgment`:

- `forward-reference.core` refers from declaration `0` to global `0`;
- `out-of-scope-variable.core` uses local variable `0` in an empty context.

## Unsupported version

`unsupported/format-0.2.core` is sufficiently well-formed to identify Core
format 0.2. A v0.1-only implementation reports `unsupported-version`, not
`malformed-encoding`.

## Manifest

`manifests/identity-u0.manifest.json` is the complete schema-valid manifest for
`canonical/identity-u0.core`. Its deterministic audit record is derived by
scanning the `pi`, `universe`, and variable/lambda constructors. The resulting
feature set is `pi` and `universe`; variables contribute no feature of their
own.

`manifests/invalid-lone-surrogate.json` is valid UTF-8 JSON but fails the
manifest schema because its provenance generator name decodes to the lone
surrogate U+D800 rather than a Unicode scalar value.
