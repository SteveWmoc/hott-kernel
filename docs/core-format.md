# Core interchange format v0.1

**Status:** Normative Phase 0 specification.

This document defines the textual interchange accepted by Core format v0.1,
its canonical byte encoding, and its name-free semantic projection. It is
intended to be implementable without sharing parser or serialization code.

The format contains fully explicit core terms. It contains no surface syntax,
name resolution, implicit arguments, metavariables, tactics, imports, or
executable code.

## 1. Version identities

This specification fixes three independently versioned objects:

- transport format: `hott-core/0.1`;
- semantic projection: `hott-semantic/0.1`;
- kernel theory: `mltt-core/0.1`.

An incompatible textual change increments the transport-format version. A
change to the name-free projection increments the projection version. A change
to accepted judgments increments the theory version. None silently implies a
change to either of the others.

## 2. Input bytes

An input is a finite sequence of bytes that must decode as well-formed UTF-8.
The following are rejected:

- an initial UTF-8 byte-order mark;
- overlong UTF-8 encodings;
- surrogate code points;
- incomplete or otherwise invalid UTF-8;
- any byte following the complete module other than permitted whitespace.

Outside strings, only these four ASCII whitespace characters are permitted:

| Name | Code point | Byte |
| --- | ---: | ---: |
| horizontal tab | U+0009 | `09` |
| line feed | U+000A | `0a` |
| carriage return | U+000D | `0d` |
| space | U+0020 | `20` |

Whitespace may occur before the first token, between tokens, and after the
complete module. It may not split an atom, integer, or escape. Comments do not
exist in Core format v0.1.

## 3. Atoms and natural numbers

Every unquoted atom is one of the fixed lowercase tags in the grammar below.
Unknown atoms are malformed.

A natural number has grammar

```text
NAT ::= "0" | NONZERO DIGIT*
NONZERO ::= "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9"
DIGIT ::= "0" | NONZERO
```

Leading zeros are malformed, including `00` and `01`. Natural numbers are
mathematically unbounded. An implementation may impose defensive storage or
resource limits only by reporting `resource-exhausted`, never
`invalid-judgment`.

## 4. Display-name strings

Strings exist only for diagnostic display names and the fixed theory name.
They are delimited by ASCII double quotes.

A decoded display name is a nonempty sequence of Unicode scalar values. A
scalar may be written directly only when it lies in one of these ranges:

- U+0020–U+0021;
- U+0023–U+005B;
- U+005D–U+007E;
- U+00A0–U+D7FF;
- U+E000–U+10FFFF.

Thus C0 controls, DEL, C1 controls, double quote, backslash, and surrogate
code points never occur directly. Exactly two escape sequences are permitted:

| Decoded scalar | Encoding |
| --- | --- |
| U+0022 double quote | `\"` |
| U+005C backslash | `\\` |

No other escape exists. In particular, `\n`, `\t`, `\xNN`, and `\uNNNN` are
malformed. The canonical printer emits every allowed non-quote,
non-backslash scalar directly in its shortest UTF-8 encoding and uses the two
escapes above where required.

The format performs no Unicode normalization. Names are compared by their
decoded scalar sequences. Two canonically equivalent Unicode spellings may
therefore be different display names and produce different artifact hashes.
Display names do not participate in semantic hashes.

## 5. Token grammar

The grammar below is written at token level. Parentheses and fixed atoms are
literal. Uppercase words are metavariables. Permitted whitespace may separate
tokens as described above.

```text
MODULE ::=
  (hott-core FORMAT THEORY DECLARATIONS)

FORMAT ::= (format NAT NAT)
THEORY ::= (theory STRING NAT NAT)
DECLARATIONS ::= (declarations DECLARATION*)

DECLARATION ::=
    (postulate STRING TERM)
  | (transparent STRING TERM TERM)
  | (opaque STRING TERM TERM)

TERM ::=
    (var NAT)
  | (global NAT)
  | (universe NAT)
  | (pi TERM TERM)
  | (lam TERM)
  | (app TERM TERM)
  | (sigma TERM TERM)
  | (pair TERM TERM)
  | (fst TERM)
  | (snd TERM)
  | (id TERM TERM TERM)
  | (refl TERM)
  | (j TERM TERM TERM TERM TERM TERM)
  | empty
  | (empty-elim TERM TERM)
  | unit
  | star
  | (unit-elim TERM TERM TERM)
  | nat
  | zero
  | (succ TERM)
  | (nat-elim TERM TERM TERM TERM)
  | (ann TERM TERM)
```

Constructor arities are exact. For example, `(var 0 1)` and `(refl)` are
malformed, not ill-typed terms.

The envelope is parsed far enough to select versions before the versioned
declaration grammar is applied. This specification accepts only `(format 0 1)`
with `(theory "mltt-core" 0 1)`. Another well-formed version header produces
`unsupported-version`, not `malformed-encoding`.

## 6. Binding and declaration indices

Local references use de Bruijn indices. Index `0` denotes the newest local
binder. Only `pi`, `sigma`, and `lam` bind a local variable in their final
term argument.

Global references use zero-based absolute indices into the declaration
sequence. Index `0` denotes the first declaration. A reference must be smaller
than the index of the declaration currently being checked. Out-of-scope local
indices and forward, self, or out-of-range global references are
`invalid-judgment`, because they are grammatically valid terms that fail
context or environment well-formedness.

Display names are diagnostic metadata and do not resolve references. Decoded
display names must be unique within a module. A duplicate is
`malformed-encoding` because it violates the module envelope before typing.

## 7. Self-contained modules

Core format v0.1 modules are self-contained. They have no import, include,
external-name, or environment-hash form. Every global reference addresses an
earlier declaration in the same module.

Module composition and imported environments require a later format decision.
Their absence does not restrict the Core v0.1 theory, whose environment remains
an arbitrary finite ordered sequence of declarations.

## 8. Canonical artifact encoding

The token grammar admits flexible whitespace for transport. The canonical
printer emits exactly one representation of the parsed module:

1. fixed atoms appear exactly as in the grammar;
2. natural numbers use their shortest decimal spelling;
3. strings use the canonical encoding from Section 4;
4. there is no whitespace immediately after `(` or before `)`;
5. adjacent elements of a list are separated by one ASCII space;
6. the entire module occupies one line;
7. one ASCII line feed follows the final `)`;
8. no other byte precedes or follows the module.

For example, the canonical empty module is exactly:

```text
(hott-core (format 0 1) (theory "mltt-core" 0 1) (declarations))
```

The code block includes a final line feed that is part of the artifact.

A parseable input whose bytes differ from the canonical printer's output is a
valid transport encoding but not a canonical artifact. A tool may rewrite it,
but artifact verification reports `noncanonical-artifact` until that rewrite
occurs.

## 9. Artifact hash

The artifact hash is SHA-256 applied directly to the canonical artifact bytes.
It is rendered as 64 lowercase hexadecimal digits. No prefix, salt, newline,
or other byte is added beyond the canonical bytes themselves.

Because the canonical bytes begin with the `hott-core` envelope, their purpose
and transport version are in the hash preimage. Display names participate. A
display-name-only edit therefore changes the artifact hash.

This hash answers: “Which exact canonical artifact was checked?” It does not
identify mathematical content up to diagnostic renaming.

## 10. Semantic projection

The semantic projection is derived structurally from a decoded module. It has
grammar:

```text
SEMANTIC ::=
  (hott-semantic PROJECTION THEORY SEMANTIC-DECLARATIONS)

PROJECTION ::= (projection 0 1)
THEORY ::= (theory "mltt-core" 0 1)
SEMANTIC-DECLARATIONS ::=
  (declarations SEMANTIC-DECLARATION*)

SEMANTIC-DECLARATION ::=
    (postulate TERM)
  | (transparent TERM TERM)
  | (opaque TERM TERM)
```

The projection omits:

- the `hott-core` transport-format marker and version;
- every diagnostic display name.

It retains:

- its own projection version;
- the exact kernel theory name and version;
- declaration order;
- declaration kind: postulate, transparent, or opaque;
- every declaration type and body;
- every local and global index.

Declaration kind and order are semantically significant. In particular,
otherwise identical transparent and opaque declarations have different
semantic projections.

The semantic projection uses the same canonical token, integer, term, string,
spacing, and final-line-feed rules as the artifact. It is a hash preimage and
testable intermediate representation, not Core input accepted by the checker.

## 11. Semantic hash

The semantic hash is SHA-256 applied directly to the canonical semantic
projection bytes and rendered as 64 lowercase hexadecimal digits.

The `hott-semantic` envelope and projection version separate this hash domain
from `hott-core` artifacts. Because the transport-format version is absent, a
future encoding may share a semantic hash only by producing exactly the same
versioned semantic projection.

This hash answers: “Which name-free mathematical content was checked?” It does
not replace the artifact hash.

## 12. Declaration validation

After parsing and version selection, declarations are checked in sequence:

- a postulate contains a type and no body;
- a transparent definition contains a type and body;
- an opaque definition contains a type and body;
- all types and bodies are closed with respect to local variables at the
  declaration boundary;
- each body checks against its declared type;
- every global reference points backward.

These are judgments of the selected Core theory. Failure is
`invalid-judgment`, not malformed encoding.

## 13. Foundation manifests

Foundation manifests are JSON metadata rather than kernel terms. Their schema,
structural extraction procedure, and separation of deterministic audit data
from asserted provenance are specified in
[`foundation-manifest-v0.1.md`](foundation-manifest-v0.1.md).

A manifest records both hashes with explicit algorithm and format identifiers.
Neither a manifest nor a claimed hash can make an invalid term valid.

## 14. Resource limits

Core format v0.1 places no mathematical upper bound on:

- module byte length;
- term depth;
- declaration count;
- natural numbers used as levels or indices;
- display-name length.

Implementations may impose defensive limits. Exceeding one must produce
`resource-exhausted`, an inconclusive operational result. It must not produce
`malformed-encoding` or `invalid-judgment` merely because another conforming
implementation could process more input.

## 15. Failure vocabulary

The normative result classes and their logical force are specified in
[`failure-classes.md`](failure-classes.md). Diagnostic wording and the order in
which independent errors are discovered are not serialized in Core v0.1.

## 16. Binary formats and future versions

No binary encoding exists in Core format v0.1. A future binary representation
must have its own transport-format version. It may claim the same semantic hash
only by decoding to the same abstract syntax and producing the exact same
versioned semantic projection.

Future import syntax, module composition, recursive declarations, additional
term constructors, or string escapes require explicit decisions and the
appropriate format or theory version change.

## 17. Exact fixtures

The byte-level examples under [`tests/format/`](../tests/format/) fix canonical
artifacts, semantic projections, expected hashes, noncanonical transport
input, and classified failures. Fixtures illustrate this document and must be
updated whenever a compatible clarification changes an example. They do not
silently override the normative grammar.
