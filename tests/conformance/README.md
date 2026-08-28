# Core v0.1 conformance fixtures

These modules distinguish the typing, conversion, computation, and environment
behavior of `mltt-core/0.1`. They are implementation-neutral contracts for a
checker, not a test harness.

All files are canonical `hott-core/0.1` artifacts. Every module under
`accepted/` must check successfully. Every module under `rejected/` must parse
and pass version and canonical-encoding checks, then produce
`invalid-judgment`.

This directory is deliberately separate from [`tests/format/`](../format/).
Format fixtures are the byte-level ground truth for parsing, canonicalization,
hashing, manifests, and failure-class boundaries. Conformance fixtures isolate
logical behavior. A file may exercise syntax, but its expected result here is
about the selected theory rather than its byte encoding.

## Accepted modules

| Fixture | Required behavior |
| --- | --- |
| `annotation-erasure.core` | `ann` erases during conversion |
| `beta.core` | function beta reduction fires |
| `declaration-kinds.core` | postulate, opaque, transparent, and backward global lookup are valid |
| `dependent-lookup.core` | lookup shifts a dependent stored type by its de Bruijn depth |
| `empty-neutral.core` | empty elimination is typed and remains neutral without a constructor |
| `j-refl.core` | identity elimination computes at reflexivity |
| `mixed-universes.core` | `pi` and `sigma` formation use the maximum level |
| `nat-elim-zero.core` | natural-number elimination computes at zero |
| `nat-elim-succ.core` | natural-number elimination computes at successor |
| `nested-substitution.core` | substitution under a binder avoids capture |
| `pair-projections.core` | both dependent-pair projections compute |
| `transparent-delta.core` | a prior transparent global unfolds during conversion |
| `unit-elim-star.core` | unit elimination computes at `star` |

Every positive computation fixture declares an identity whose endpoints become
equal only when the named reduction fires; a reflexivity body is therefore a
witness of the required conversion rather than merely a term that happens to
contain the constructor.

## Rejected modules

| Fixture | Required rejection |
| --- | --- |
| `bad-body.core` | definition body has the wrong declared type |
| `bare-lambda-synthesis.core` | a lambda cannot synthesize in function position |
| `bare-motive.core` | an unannotated lambda motive cannot synthesize |
| `bare-pair-synthesis.core` | a pair cannot synthesize in projection position |
| `empty-no-computation.core` | empty elimination has no invented computation rule |
| `equality-reflection.core` | an identity proof does not change conversion |
| `false-refl.core` | reflexivity cannot join distinct numeral endpoints |
| `forward-reference.core` | a declaration cannot see a later global |
| `j-motive-path-domain.core` | the second `J` motive domain must be the required identity type |
| `j-path-mismatch.core` | the `J` path endpoint must match its explicit endpoint |
| `j-wrong-branch.core` | the `J` branch must inhabit the motive at reflexivity |
| `no-pi-eta.core` | function eta is not judgmental |
| `no-sigma-eta.core` | pair eta is not judgmental |
| `no-uip.core` | arbitrary identity proofs are not judgmentally equal |
| `opaque-no-delta.core` | an opaque global never unfolds during conversion |
| `out-of-scope-variable.core` | declarations are closed at their boundary |
| `unit-motive-domain.core` | a unary motive domain must match the eliminated type |
| `unit-no-uniqueness.core` | arbitrary unit inhabitants are not judgmentally `star` |
| `universe-noncumulative.core` | a small type is not silently lifted to a larger universe |
