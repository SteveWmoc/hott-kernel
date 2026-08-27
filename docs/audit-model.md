# Foundation audit model

## Purpose

A declaration can be free of named postulates while still relying on powerful
kernel rules. The audit model therefore reports several different forms of
dependency rather than calling all of them “axioms.” The terms below have the
precise meanings given in the [glossary](glossary.md).

## Dependency classes

### Kernel rules

Primitive judgments and computation principles supplied by the selected core
theory, for example:

- `universe`;
- `pi`;
- `sigma`;
- `identity`;
- `empty`;
- `unit`;
- `natural-numbers`.

These entries report rules that the checker itself recognizes. They are not
named constants in the checked declaration environment.

### Extensions

Optional rule packages not present in the selected core, for example:

- `propositional-truncation`;
- `hit.circle`;
- `cubical.composition`;
- `cubical.glue`.

An extension may add formation, introduction, elimination, or judgmental
computation rules. Its manifest entry must identify which rules it adds and
the exact extension version.

“Univalence” alone is not an adequate extension identifier. If univalence is
provided computationally by cubical primitives, the manifest reports those
primitive extensions and their rules. If it is merely assumed as an opaque
term, it is a postulate instead.

### Postulates

Opaque constants supplied without checked bodies, for example:

- excluded middle;
- a choice principle;
- an axiomatic form of function extensionality;
- axiomatic univalence, such as `postulate.univalence`.

A principle implemented by a kernel extension and the same mathematical
principle assumed as an opaque term are intentionally reported differently.

### Declaration dependencies

Previously checked global declarations referenced by a term. Audit results
include their transitive foundational dependencies. Transparent and opaque
checked definitions remain declarations, while an opaque declaration without
a body is a postulate.

### Generation provenance

Tools or agents involved in producing a candidate term, such as:

- a tactic;
- a decision procedure;
- an SMT solver;
- an AI system;
- manual construction.

Provenance is not part of logical validity, but it is relevant to
reproducibility and metalinguistic analysis.

## Proposed manifest shape

The eventual serialized schema may resemble:

```json
{
  "declaration": "Path.inverse",
  "artifact": {
    "format": "hott-core/0.1",
    "sha256": "..."
  },
  "kernel": {
    "theory": "mltt-core",
    "version": "0.1"
  },
  "rules": ["universe", "pi", "identity"],
  "extensions": [],
  "postulates": [],
  "declarations": ["Path"],
  "generated_by": [
    {
      "kind": "surface-elaborator",
      "version": "0.1"
    },
    {
      "kind": "ai-assistant",
      "name": "example-only"
    }
  ]
}
```

The exact JSON schema is not yet frozen. The canonical term artifact described
in [the core-format sketch](core-format.md) is authoritative; the manifest is
recomputed metadata tied to that artifact's hash. The frozen schema will label
the exact artifact hash separately from the name-free semantic hash; a verifier
recomputes both.

## Transitivity

If declaration $d$ references declaration $e$, the foundation manifest for
$d$ contains the union of:

- features used directly by $d$;
- features in the manifest for $e$;
- corresponding dependencies for every other referenced declaration.

The union operation is deterministic: identifiers are deduplicated and
canonically ordered. Cycles are forbidden in checked declaration environments
unless a future, explicitly specified inductive or recursive mechanism permits
them.

## Reproducibility

A manifest is an explanatory certificate, not a proof of successful checking.
A verifier should be able to:

1. load the identified kernel theory version;
2. reconstruct the referenced environment;
3. parse and check the fully explicit term;
4. recompute the artifact hash and manifest;
5. compare the deterministic result.

A malformed or dishonest manifest cannot cause an invalid term to be accepted.

## Metalinguistic assumptions

The logical validity of an accepted term does not depend on how it was found.
Nevertheless, proof search may employ classical reasoning, external
computation, or unverified code without leaving a corresponding constant in the
term.

Generation provenance is therefore recorded separately from object-theoretic
dependencies. The project will not claim that these two analyses answer the
same question.
