# Foundation manifest v0.1

**Status:** Normative Phase 0 specification.

A foundation manifest is a deterministic audit report for one complete Core
artifact. It identifies the artifact and theory and contains one audit record
for every declaration. It is not a proof object and cannot affect whether the
kernel accepts a term.

The machine-readable schema is
[`foundation-manifest-0.1.schema.json`](../schemas/foundation-manifest-0.1.schema.json).

## 1. Schema identity

The schema identifier is:

```text
hott-foundation-manifest/0.1
```

The feature vocabulary identifier for Core v0.1 is:

```text
mltt-core-features/0.1
```

The manifest-schema version, feature-vocabulary version, core transport
version, semantic-projection version, and theory version are independent.

## 2. One manifest per artifact

A manifest covers exactly one canonical `hott-core` artifact. Its declaration
records occur in the same order as the artifact declarations. Record index `0`
describes the first declaration, and each record's `index` must equal its array
position.

Core v0.1 has no imports, so every dependency is an index into the same
artifact. Future imported environments require a later manifest schema.

## 3. Top-level shape

The top-level fields are:

```json
{
  "schema": "hott-foundation-manifest/0.1",
  "artifact": {
    "format": "hott-core/0.1",
    "sha256": "d15f02d6d077829b1133995f8f81b3d8e404bb7bef976368eb86fd7a98d22834",
    "semantic_projection": "hott-semantic/0.1",
    "semantic_sha256": "af4c8b526f9a1fe4a03efe2cfe60b0744f0a7f6bcca227f762d3e1c6d9de5deb"
  },
  "kernel": {
    "theory": "mltt-core",
    "version": "0.1"
  },
  "audit": {
    "feature_vocabulary": "mltt-core-features/0.1",
    "declarations": []
  },
  "asserted_provenance": []
}
```

This is the complete deterministic manifest content for the canonical empty
module fixture. An exact example containing a declaration record is checked in
at
[`identity-u0.manifest.json`](../tests/format/manifests/identity-u0.manifest.json).

## 4. Deterministic and asserted portions

These top-level fields are deterministic and recomputable from the checked
artifact and selected specifications:

- `schema`;
- `artifact`;
- `kernel`;
- `audit`.

`asserted_provenance` contains claims about how candidate terms were produced.
The kernel cannot reconstruct those historical facts. A verifier validates
their shape but does not certify their truth and excludes them when comparing
recomputed audit data.

An empty provenance array means that no provenance claim accompanies the
artifact. It does not mean “manually authored.”

## 5. Declaration record

Each declaration record has this shape:

```json
{
  "index": 0,
  "display_name": "id-U0",
  "kind": "transparent",
  "direct": {
    "kernel_features": ["pi", "universe"],
    "extensions": [],
    "postulates": [],
    "declarations": []
  },
  "transitive": {
    "kernel_features": ["pi", "universe"],
    "extensions": [],
    "postulates": [],
    "declarations": []
  }
}
```

`kind` is exactly one of `postulate`, `transparent`, or `opaque`. It is not
inferred from the display name.

## 6. Core feature vocabulary

Core v0.1 defines these kernel-feature identifiers:

- `empty`;
- `identity`;
- `natural-numbers`;
- `pi`;
- `sigma`;
- `unit`;
- `universe`.

They name families of primitive syntax and rules rather than individual
operational checker events. The theory version identifies the complete ambient
rule set; a feature list explains which families occur structurally in a
declaration and its dependencies.

## 7. Constructor mapping

Direct feature extraction recursively scans a declaration's type and, when it
has one, its body. Each encountered constructor contributes as follows:

| Core constructor tags | Feature |
| --- | --- |
| `universe` | `universe` |
| `pi`, `lam`, `app` | `pi` |
| `sigma`, `pair`, `fst`, `snd` | `sigma` |
| `id`, `refl`, `j` | `identity` |
| `empty`, `empty-elim` | `empty` |
| `unit`, `star`, `unit-elim` | `unit` |
| `nat`, `zero`, `succ`, `nat-elim` | `natural-numbers` |
| `var`, `global`, `ann` | none |

“None” means that the tag contributes no feature by itself. Child terms are
still scanned. A `global` additionally contributes its referenced declaration
index, and `ann` contributes features found in both its term and type.

## 8. Direct dependencies

For declaration $d$, the `direct` object is computed as follows:

1. scan its declared type;
2. scan its body if its kind is `transparent` or `opaque`;
3. collect mapped kernel features;
4. collect every referenced earlier global in `declarations`;
5. place each directly referenced postulate index in `postulates` as well;
6. record directly named extension packages in `extensions`.

Core v0.1 has no extension syntax, so every conforming Core v0.1 manifest has
an empty direct `extensions` array. The field is retained so the dependency
classes remain structurally distinct and versionable.

A postulate's own record scans its type and has no body. It does not list itself
as a dependency.

## 9. Transitive closure

The `transitive` object includes the direct dependencies. For each dependency
class, it is the union of the direct set with the transitive sets of every
directly referenced declaration.

If $D_d$ is the set of direct declaration dependencies of $d$, then each
feature-like component satisfies

$$
T(d)=R(d)\cup\bigcup_{e\in D_d}T(e),
$$

where $R(d)$ is the direct component. For declaration indices themselves,
$T(d)$ additionally contains every $e\in D_d$.

Because every global reference points backward, records can be computed in one
forward pass and cycles cannot occur.

## 10. Transparency and opacity

The extraction procedure never performs reduction or unfolds a global.

When an opaque or postulate global is referenced, its already computed
transitive feature sets are unioned into the referencing record. The opaque
body is not traversed again, and a postulate has no body to traverse. The same
cached-union rule is used for transparent globals; manifest extraction is
therefore independent of a checker's unfolding and normalization strategy.

An opaque declaration's own direct record includes features and dependencies
from its checked body. Opacity controls conversion after acceptance; it does
not erase what was required to validate the declaration itself.

## 11. Canonical ordering

Every set-valued JSON array is duplicate-free and ordered as follows:

- `kernel_features` and `extensions`: ascending UTF-8 byte order;
- `postulates` and `declarations`: ascending numerical order;
- declaration audit records: ascending `index` order;
- provenance records: ascending declaration index;
- each `generated_by` array: ascending tuple
  `(kind, name, version-or-empty, details-or-empty)` by UTF-8 byte order.

`postulates` is always a subset of `declarations`. Every dependency index is
strictly smaller than the record's declaration index.

These constraints are normative even where JSON Schema cannot express them.

At most one provenance record may identify a given declaration, and every
provenance declaration index must identify an audit record in the same
manifest.

## 12. Extensions

An extension identifier names a separately versioned rule package, not a
mathematical slogan. Future schemas or theory versions must define how selected
extension packages enter direct structural extraction.

For example, an opaque constant named “univalence” is a postulate dependency,
whereas computational cubical primitives would be extension dependencies.
Core v0.1 already permits explicit postulate declarations, but it has no
extension-package selection mechanism; adding one requires a later theory and
manifest decision.

## 13. Postulates

The `postulates` arrays contain declaration indices whose kind is `postulate`.
They do not contain opaque definitions, because opaque definitions have checked
bodies. A postulate reached through another declaration appears in the
transitive array even when it is not referenced directly.

## 14. Asserted provenance

Each provenance record identifies a declaration and contains a nonempty
`generated_by` array. A generator entry has:

- required `kind` and `name` strings;
- optional `version`;
- optional free-text `details`.

Provenance may identify manual construction, an elaborator, a tactic, an
external solver, an AI system, or another producer. It is excluded from both
artifact and semantic hashes because it is not part of the core artifact.

Provenance does not change logical validity. False or incomplete provenance is
an audit defect, not a proof of an invalid judgment.

## 15. JSON representation

Manifests are well-formed UTF-8 JSON and conform to the checked-in JSON Schema.
The following additional rules apply:

- no byte-order mark;
- no duplicate object keys;
- every decoded string is a sequence of Unicode scalar values; lone surrogates
  are forbidden even when written as JSON escapes such as `\uD800`;
- decoded `display_name` values exactly match the core artifact;
- decoded numeric values are nonnegative integers; their JSON spelling is not
  canonical;
- the arrays described as sets obey Section 11.

JSON object-key order and insignificant whitespace are not semantically
significant. Manifest bytes are not hashed in schema v0.1. Independent
verifiers compare the parsed deterministic fields, not incidental JSON
pretty-printing.

## 16. Verification procedure

A verifier:

1. parses the core artifact and selects supported versions;
2. checks canonical encoding;
3. recomputes artifact and semantic hashes;
4. checks every declaration in order;
5. recomputes every direct and transitive audit record;
6. validates the manifest schema and ordering constraints;
7. compares the recomputed deterministic fields;
8. reports asserted provenance separately.

A hash or manifest mismatch does not alter the kernel result. It reports an
integrity or audit failure under the vocabulary in
[`failure-classes.md`](failure-classes.md).

## 17. Exact fixtures

Canonical artifacts, semantic projections, expected hashes, malformed inputs,
and a complete manifest live under [`tests/format/`](../tests/format/). Those
bytes are normative examples of this specification.
