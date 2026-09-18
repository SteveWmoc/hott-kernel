# Surface v0.1

**Status:** Implementation contract.

**Surface format:** `hott-surface/0.1`.

**Target core format:** `hott-core/0.1`.

## 1. Purpose and trust boundary

Surface v0.1 is the first untrusted, human-writable layer above Core v0.1.
It provides names for local binders and global declarations while preserving
the explicit structure of the core calculus.

Surface elaboration is not logical validation. A successful elaboration means
only that a source module was parsed, its names were resolved deterministically,
and a Core v0.1 artifact was produced. The resulting artifact is valid only if
the Core checker accepts it.

The surface parser, name resolver, elaborator, module-name handling, comments,
and diagnostics are outside the trusted logical core. They may not add a
typing, conversion, computation, universe, or elimination rule.

An incompatible change to this surface contract requires a new surface-format
version. It does not by itself change the `mltt-core/0.1` theory.

## 2. Deliberately small scope

Surface v0.1 adds only:

- a versioned source file header;
- a diagnostic module name;
- ASCII identifiers;
- named local binders;
- named references resolved to local variables or earlier global declarations;
- the three existing Core declaration kinds;
- comments and insignificant whitespace;
- deterministic emission of canonical Core v0.1 bytes.

Surface v0.1 deliberately does **not** add:

- implicit arguments;
- metavariables or holes;
- typeclass search;
- tactics or macros;
- overloaded notation;
- type-directed name resolution;
- automatic universe inference;
- automatic insertion of missing annotations;
- recursive definitions;
- imports;
- namespaces or qualified names;
- separate compilation;
- new declaration kinds;
- new object-theoretic rules.

Those features may be designed later as untrusted elaboration conveniences.
None may silently enlarge the theory accepted by the Core checker.

## 3. Source encoding and lexical grammar

A Surface v0.1 source file is UTF-8 text without a byte-order mark.

Whitespace consists of ASCII space, tab, carriage return, and line feed.
Whitespace may occur before the first token, between any two tokens, after an
opening parenthesis, before a closing parenthesis, and after the complete
source file. Whitespace may not occur inside a token.

A semicolon begins a line comment. The comment extends through the next line
feed or the end of the file. For tokenization, each discarded comment is
replaced conceptually by one whitespace separator. In particular, removing a
comment may never concatenate the tokens on its two sides.

Natural numbers use the grammar

```text
NAT ::= 0 | [1-9][0-9]*
```

Surface v0.1 identifiers use the grammar

```text
IDENT ::= [A-Za-z_][A-Za-z0-9_]*
```

Identifiers are case-sensitive and are compared by their exact UTF-8 bytes.
Because the grammar is ASCII, no Unicode normalization question arises in
Surface v0.1.

The following words are reserved and may not be used in any name position,
including module names, declaration names, binder names, and the operand of
`ref`:

```text
surface module
postulate transparent opaque
ref universe pi lam app sigma pair fst snd
id refl j empty empty-elim unit star unit-elim
nat zero succ nat-elim ann
```

Quoted identifiers and arbitrary display-name strings are intentionally absent
from Surface v0.1. A declaration identifier becomes that declaration's Core
display name.

## 4. Concrete grammar

Surface format selection occurs before applying the version-specific grammar.
An implementation first parses the leading version envelope

```text
(surface NAT NAT)
```

using the lexical rules of this section. The only supported version pair in
this contract is `0 1`. Any other well-formed pair is rejected as
`unsupported surface version`, not as malformed Surface v0.1 syntax. If the
leading envelope itself is not well formed, the input is malformed surface
syntax.

After selecting version `0 1`, the complete source file has one module header
and zero or more declarations:

```text
FILE ::=
  (surface 0 1)
  (module IDENT)
  DECL*

DECL ::=
    (postulate IDENT TERM)
  | (transparent IDENT TERM TERM)
  | (opaque IDENT TERM TERM)

TERM ::=
    (ref IDENT)
  | (universe NAT)
  | (pi IDENT TERM TERM)
  | (lam IDENT TERM)
  | (app TERM TERM)
  | (sigma IDENT TERM TERM)
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

Parentheses and arities are exact. Unknown term or declaration tags are
rejected by the surface parser.

The term grammar intentionally mirrors Core v0.1. The only constructor not
represented literally is the Core distinction between `var n` and
`global n`; Surface v0.1 writes both as `(ref IDENT)`, and deterministic
name resolution selects the corresponding Core constructor.

## 5. Binding and name resolution

Elaboration maintains two ordered environments:

1. a local binder stack, newest binder first;
2. the sequence of successfully elaborated earlier global declarations.

### 5.1 Local binders

The third argument of `pi` and `sigma` is elaborated with the named binder
added to the local stack.

The body of `lam` is elaborated with its named binder added to the local
stack.

These are exactly the binding positions of Core v0.1. No other Surface v0.1
form introduces a local binder.

Local binders may shadow earlier local binders and global declaration names.
Resolution always chooses the nearest local binder first.

If a reference resolves to the local binder at zero-based depth `n` in the
newest-first stack, it elaborates to Core `(var n)`.

For example,

```text
(pi A (universe 0)
  (pi x (ref A)
    (ref A)))
```

elaborates to the Core shape

```text
(pi (universe 0)
  (pi (var 0)
    (var 1)))
```

because the inner `x` is the newest binder and the outer `A` is one binder
away inside the inner codomain.

### 5.2 Global declarations

Surface declaration identifiers must be unique within a source module.

A declaration name is not visible while elaborating its own type or body. It is
inserted into the global name environment only after the complete declaration
has elaborated successfully.

Therefore Surface v0.1 has the same strict forward-only global availability as
Core v0.1:

- a declaration may reference only earlier declarations;
- self-reference is unresolved;
- forward reference is unresolved.

If a reference is not local and matches the earlier global declaration at
zero-based declaration index `i`, it elaborates to Core `(global i)`.

If no local or earlier-global binding matches the identifier, elaboration
fails with an unknown-name diagnostic.

A reserved word in a name position is rejected lexically as a reserved
identifier before name resolution. Thus `(ref pi)` is a reserved-identifier
error, not an unknown-name error.

Name resolution is purely lexical. Surface v0.1 never invokes the type checker
to decide which binding a name denotes.

## 6. Structural elaboration

After resolving `ref`, every Surface v0.1 term constructor elaborates
homomorphically to the identically named Core v0.1 constructor.

Examples:

```text
(universe 2)         -> (universe 2)
(app f a)            -> (app E[f] E[a])
(pair a b)           -> (pair E[a] E[b])
(id A a b)           -> (id E[A] E[a] E[b])
(nat-elim C z s n)   -> (nat-elim E[C] E[z] E[s] E[n])
(ann t A)            -> (ann E[t] E[A])
```

where `E[-]` denotes recursive surface elaboration.

Surface elaboration performs no type inference and introduces no hidden Core
term. In particular:

- `lam` remains a checking-only Core lambda;
- `pair` remains a checking-only Core pair;
- universe levels remain exactly the written natural numbers;
- motives in synthesis positions must contain whatever explicit `ann`
  structure Core v0.1 requires;
- no eta rule, coercion, cumulativity rule, or implicit conversion is added.

A surface source can therefore elaborate successfully and still be rejected by
the Core checker. That separation is intentional.

## 7. Declarations and module compilation

The three Surface v0.1 declaration forms map directly to the three Core v0.1
declaration kinds:

```text
(postulate N A)
  -> Core postulate with display name N and type E[A]

(transparent N A t)
  -> Core transparent declaration with display name N, type E[A], body E[t]

(opaque N A t)
  -> Core opaque declaration with display name N, type E[A], body E[t]
```

A Surface v0.1 source file is one **surface module** and compiles to exactly one
Core v0.1 module.

The `(module IDENT)` name is diagnostic source metadata only in Surface v0.1.
It is not emitted into Core v0.1, is not included in the Core artifact hash or
semantic hash, and has no effect on checking.

Imports and cross-module name resolution are out of scope for Surface v0.1.
Adding them requires an explicit later surface-module contract because import
ordering, artifact identity, and declaration-index composition must be
specified before implementation.

## 8. Determinism and output

For a fixed Surface v0.1 source file and implementation limits, successful
elaboration must determine one Core v0.1 abstract syntax tree.

The public byte-level surface compiler must emit that tree using the existing
canonical Core v0.1 printer. It must not preserve surface comments, whitespace,
binder names, or the surface module name in Core bytes.

Global declaration identifiers do survive as Core display names. Consequently:

- changing only a local binder name does not change emitted Core bytes;
- changing only the surface module name does not change emitted Core bytes;
- consistently renaming a global declaration changes the Core artifact
  identity because display names are serialized;
- that same consistent global rename leaves the existing name-free semantic
  identity unchanged, provided declaration order and resolved references are
  otherwise identical.

A convenience operation may immediately pass the emitted canonical Core bytes
to the existing checker or Foundation Manifest generator. Such a wrapper does
not make surface elaboration trusted.

## 9. Diagnostics and failure boundary

Surface diagnostics are tooling diagnostics and are not additions to the frozen
Core v0.1 failure-class vocabulary.

An implementation should distinguish at least:

- malformed surface syntax;
- unsupported surface version;
- reserved identifier;
- duplicate global declaration name;
- unknown name;
- resource exhaustion.

The exact diagnostic wording, source-span representation, and recovery strategy
are not part of the Surface v0.1 compatibility contract.

Once canonical Core bytes have been produced, Core parsing, checking, manifest
generation, and verification retain their existing frozen failure classes.
A surface tool must not relabel a Core logical rejection as a surface parsing
error.

## 10. Examples

### 10.1 Accepted name resolution

```text
(surface 0 1)
(module Basics)

(postulate A (universe 0))

(transparent identity_map
  (pi x (ref A) (ref A))
  (lam x (ref x)))
```

The global `A` becomes `(global 0)`. Inside `identity_map`, the lambda reference
`x` becomes `(var 0)`. The resulting Core module is then independently
checked.

### 10.2 Local shadowing

```text
(surface 0 1)
(module Shadow)

(postulate A (universe 0))

(transparent choose_inner
  (pi A (universe 0)
    (pi x (ref A) (ref A)))
  (lam A
    (lam x
      (ref x))))
```

Inside the first `pi`, local `A` shadows global `A`. Inside the nested
lambda body, `x` denotes the nearest local binder.

### 10.3 Explicit motive annotation

Surface v0.1 does not synthesize annotations for a lambda used where Core
requires synthesis. The user writes the annotation explicitly:

```text
(ann
  (lam n BODY)
  (pi n nat (universe 0)))
```

The elaborator preserves that `ann` exactly. This satisfies the Core v0.1
requirement that the emitted Core motive carry an explicit annotation; Core
does not require a particular untrusted surface language to synthesize that
annotation rather than requiring it in source.

### 10.4 Rejected self-reference

```text
(surface 0 1)
(module BadSelf)

(transparent loop
  nat
  (ref loop))
```

`loop` is not in the global environment while its body is elaborated, so
`ref loop` is an unknown name.

### 10.5 Rejected forward reference

```text
(surface 0 1)
(module BadForward)

(postulate first (ref later))
(postulate later (universe 0))
```

`later` is unavailable while `first` is elaborated.

### 10.6 Rejected duplicate declaration name

```text
(surface 0 1)
(module Duplicate)

(postulate A (universe 0))
(postulate A (universe 0))
```

The second `A` is rejected by the surface name environment before Core output
is produced.

## 11. Implementation slices

The intended implementation sequence is:

1. define the Surface v0.1 AST and strict source parser, without checker calls;
2. implement deterministic local/global name resolution and translation to the
   existing Core AST;
3. emit canonical Core bytes and add byte-for-byte elaboration fixtures;
4. add an optional compile-and-check wrapper that delegates logical validity to
   the existing Core APIs;
5. only after this base is stable, design imports, richer notation, implicit
   arguments, metavariables, or tactics as separate untrusted layers.

The first implementation should remain dependency-free unless a later reviewed
engineering decision justifies otherwise.

## 12. Foundation impact

Surface v0.1 changes no Core judgment and adds no kernel feature, extension, or
postulate.

Its governing invariant is:

> Surface convenience may choose syntax and names, but only explicit Core terms
> checked by the kernel determine logical validity.
