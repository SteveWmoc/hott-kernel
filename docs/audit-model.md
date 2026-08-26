# Foundation audit model

## Purpose

A declaration can be free of named postulates while still relying on powerful
kernel rules. The audit model therefore reports several different forms of
dependency rather than calling all of them “axioms.”

## Dependency classes

### Kernel rules

Primitive judgments and computation principles supplied by the selected core
theory, for example:

- <code>universe</code>
- <code>pi</code>
- <code>sigma</code>
- <code>identity</code>
- <code>empty</code>
- <code>unit</code>
- <code>natural-numbers</code>

### Extensions

Optional rule packages not present in the selected core, for example:

- <code>univalence</code>
- <code>propositional-truncation</code>
- <code>hit.circle</code>
- <code>cubical.composition</code>
- <code>cubical.glue</code>

An extension may add formation, introduction, elimination, or judgmental
computation rules. Its manifest entry must identify which.

### Postulates

Opaque constants supplied without checked bodies, for example:

- excluded middle;
- a choice principle;
- an axiomatic form of function extensionality;
- axiomatic univalence.

A principle implemented later by a kernel extension and the same principle
assumed as an opaque term are intentionally reported differently.

### Declaration dependencies

Previously checked global declarations referenced by a term. Audit results
include their transitive foundational dependencies.

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

~~~json
{
  "declaration": "Path.inverse",
  "kernel": {
    "theory": "mltt",
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
    }
  ]
}
~~~

The exact encoding is not yet frozen.

## Transitivity

If declaration $d$ references declaration $e$, the foundation manifest for
$d$ contains the union of:

- features used directly by $d$;
- features in the manifest for $e$;
- corresponding dependencies for every other referenced declaration.

Cycles are forbidden in checked declaration environments unless a future,
explicitly specified inductive or recursive mechanism permits them.

## Reproducibility

A manifest is an explanatory certificate, not a proof of successful checking.
A verifier should be able to:

1. load the identified kernel theory version;
2. reconstruct the referenced environment;
3. check the fully explicit term;
4. recompute the manifest;
5. compare the deterministic result.

## Metalinguistic assumptions

The logical validity of an accepted term does not depend on how it was found.
Nevertheless, a proof search may employ classical reasoning, external
computation, or unverified code without leaving a corresponding constant in the
term.

Generation provenance is therefore recorded separately from object-theoretic
dependencies. The project will not claim that these two analyses answer the
same question.
