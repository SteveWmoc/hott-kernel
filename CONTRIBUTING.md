# Contributing

The project is currently specifying its foundation. Documentation changes are
therefore implementation work, not preliminary decoration.

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

## Phase 0 discipline

Until Core v0.1 is frozen, please do not add a kernel implementation, tactic
framework, editor plugin, or general surface language. The specification must
lead the software.

## AI assistance

AI-assisted contributions are welcome. Material assistance should be disclosed
in the pull request. Generated work receives the same technical review as any
other contribution.
