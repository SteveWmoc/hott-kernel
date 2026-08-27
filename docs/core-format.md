# Core interchange format

**Status:** Phase 0 format sketch.

The stable interchange format will allow independently implemented checkers to
consume exactly the same declarations without sharing a parser, elaborator, or
runtime. This document fixes the direction and a provisional grammar; it is not
yet a compatibility promise.

## Design requirements

The format must be:

- fully explicit;
- deterministic;
- easy to parse without a large dependency;
- independent of surface notation and local binder names;
- versioned by both syntax version and kernel-theory version;
- suitable for streaming and content hashing;
- capable of preserving declaration kind and transparency;
- free of tactics, metavariables, and unresolved names.

## Textual S-expression encoding

The primary interchange form is a canonical UTF-8 S-expression. Term tags
have fixed arity:

```text
(var n)
(global n)
(universe i)
(pi A B)
(lam t)
(app f a)
(sigma A B)
(pair a b)
(fst p)
(snd p)
(id A a b)
(refl a)
(j A a C d b p)
empty
(empty-elim C e)
unit
star
(unit-elim C c u)
nat
zero
(succ n)
(nat-elim C z s n)
(ann t A)
```

Local references use de Bruijn indices, with zero denoting the newest local
binder. Global references use zero-based absolute indices into the declaration
sequence, with zero denoting the first declaration; a reference must be
strictly smaller than the index of the declaration currently being checked.
Consequently the checker does not perform name resolution.

## Module envelope

A provisional module has this shape:

```text
(hott-core
  (format 0 1)
  (theory "mltt-core" 0 1)
  (declarations
    DECLARATION ...))
```

Declarations have one of these forms:

```text
(postulate "display-name" TYPE)
(transparent "display-name" TYPE BODY)
(opaque "display-name" TYPE BODY)
```

The display name is diagnostic metadata. Logical references use declaration
indices. A module must not contain duplicate display names, but renaming a
display name does not alter the meaning of a term.

For example, polymorphic identity at level zero can be represented as:

```text
(hott-core
  (format 0 1)
  (theory "mltt-core" 0 1)
  (declarations
    (transparent "id-U0"
      (pi (universe 0)
          (pi (var 0) (var 1)))
      (lam (lam (var 0))))))
```

The outer declaration type is

$$
\Pi(A:\mathcal U_0).\Pi(x:A).A.
$$

In the inner codomain, `var 0` is $x$ and `var 1` is $A$.

## Canonicalization

The canonical printer will use:

- lowercase tags exactly as listed;
- unsigned decimal natural numbers without leading zeros;
- JSON-style escaping for display-name strings;
- one space between adjacent atoms;
- no comments;
- a final newline;
- a fixed declaration order.

Whitespace outside strings is semantically irrelevant to parsing. Hashing a
serialized artifact uses the canonical printer's bytes.

The canonical artifact hash covers those exact bytes, so diagnostic display
names participate in it. This hash answers which serialized artifact was
checked; changing a display name changes the artifact hash even though it does
not change the represented judgment.

A separate semantic hash covers a versioned, name-free projection of the
parsed abstract syntax. That projection retains the theory version,
declaration order and kind, types, and bodies, but omits display names.
Generation provenance is already outside the core artifact and does not
participate. Thus a display-name-only change preserves the semantic hash while
changing the artifact hash. The two hashes answer different questions and
must never share an unlabeled manifest field.

## Foundation manifests

Foundation manifests remain JSON because they are explanatory metadata rather
than kernel terms. A manifest identifies the exact core artifact hash and
theory version it describes. Once the semantic-hash encoding is frozen, the
manifest will record both hashes with explicit algorithm and projection
identifiers.

The core term is authoritative. A malformed or dishonest manifest cannot make
an invalid term valid, because a verifier recomputes the manifest after
checking.

## Binary formats

No binary encoding is planned for Core v0.1. A future binary representation
must decode to the same abstract syntax and will require its own format version
and decision record.

## Open items before format freeze

- exact string-escape grammar;
- size and recursion-depth limits for defensive implementations;
- module composition and imported-environment hashes;
- hash algorithms and the exact versioned semantic projection;
- the final foundation-manifest JSON schema.
