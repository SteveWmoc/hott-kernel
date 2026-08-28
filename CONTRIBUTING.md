# Contributing

Core v0.1 is frozen for implementation. Specification changes remain
implementation work, not preliminary decoration.

## Pull requests

Keep pull requests narrow and explain:

- what judgment, rule, boundary, or document changes;
- whether the foundation or trusted computing base is affected;
- which accepted and rejected examples distinguish the change;
- whether any external generator or AI system materially assisted the work.

A foundational change requires a decision record under `docs/decisions/`
before implementation.

## Review standard

A change should be:

- precise enough to admit independent implementation;
- explicit about assumptions and computation behavior;
- accompanied by positive and negative examples;
- free of silent changes to the accepted theory;
- reproducible without trusting the tool that generated it.

## Frozen-core discipline

Core v0.1 is frozen by
[Decision 0012](docs/decisions/0012-freeze-core-v0.1.md). Implementations must
conform to its published theory, formats, and conformance fixtures.

A change to an accepted judgment, conversion, computation rule, universe
policy, or elimination principle requires a new decision record and a new
theory version. Transport, projection, manifest, and feature-vocabulary changes
follow their own version policies. Editorial clarifications, proof work, and
implementation corrections may retain existing versions only when they leave
the specified behavior unchanged.

## AI assistance

AI-assisted contributions are welcome. Material assistance should be disclosed
in the pull request. Generated work receives the same technical review as any
other contribution.
