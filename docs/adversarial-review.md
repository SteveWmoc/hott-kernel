# Independent adversarial review pilot

This repository can ask Z.ai's `GLM-5.3-Flash` model, hosted by Fireworks AI,
to perform a blind, advisory review of a pull request. The reviewer is
deliberately outside the trusted boundary: its claims acquire weight only
through exact evidence, reproduction, tests, and human resolution.

The pilot is manual. It does not run on every pull request and it never changes
mergeability or branch protection.

Each paid dispatch selects two independent dimensions:

- a `design` or `code` review phase; and
- an automatically routed or explicitly selected risk profile.

This permits an independent challenge before implementation and a separate
exact-SHA check after implementation without pretending that either model pass
is a merge gate.

## Security and review boundary

- The workflow runs only when manually dispatched from `main`.
- The initial pilot accepts same-repository pull requests only; fork pull
  requests fail closed.
- It reads pull-request data through the GitHub API. It does not check out or
  execute pull-request code.
- The Fireworks key is sent only to the exact HTTPS endpoint
  `https://api.fireworks.ai/inference/v1/chat/completions`.
- Pull-request text is marked as untrusted data in the reviewer prompt, and no
  tools are exposed to the model.
- Review profiles are checked-in prompt supplements selected from a fixed
  allowlist. A workflow input cannot supply prompt text or a prompt path.
- The `auto` profile resolves from checked-in path rules after packet assembly.
  Route order is an explicit risk priority; all matching profiles and the one
  selected profile are recorded in the report metadata.
- The model receives the unified diff, bounded complete changed-file contents,
  and the frozen contracts as they existed at the pull request's base commit.
- Existing comments and reviews are excluded to reduce anchoring and preserve
  an independent first pass.
- The result is tied to the exact base and head SHAs. The prompt and input
  packet are SHA-256 identified, and the complete packet and structured report
  are retained as workflow artifacts for 30 days.
- An optional historical head must be a full 40-character SHA returned by
  GitHub as a commit of the selected pull request. The harness constructs that
  replay from the recorded PR base and refuses an unrelated or diverged commit.
- The harness rechecks the recorded base and current head after packet assembly
  and immediately before publication. A concurrent push or base change aborts
  the run as stale.
- Model output is advisory. CI and the project owner remain the merge gate.

The repository is public, so the selected source and specification content is
already public. Do not copy this pilot to a private repository without making
an explicit decision about sending private source code to the API provider.

## One-time setup

1. Create or reuse a key in the
   [Fireworks API Keys page](https://app.fireworks.ai/settings/users/api-keys).
2. In this repository, open **Settings → Secrets and variables → Actions**.
3. Create a repository secret named `FIREWORKS_API_KEY` containing the key.

The endpoint and model are pinned in the reviewed harness:

- endpoint: `https://api.fireworks.ai/inference/v1/chat/completions`
- model: `accounts/fireworks/models/glm-5p3-flash`

Changing either value requires a reviewed code change. This keeps the external
processor and model identity visible in the repository rather than allowing an
unreviewed Actions variable to redirect source packets.

## Run a review

1. Open **Actions → Adversarial Review → Run workflow**.
2. Select the `main` branch.
3. Enter the pull request number and choose a review phase:
   - `design` challenges the proposal, contracts, invariants, and acceptance
     criteria without treating absent implementation as a defect;
   - `code` traces the finished implementation, tests, fixtures, and failure
     behavior against the supplied contracts.
4. Select `auto` for a checked-in path-routed focus, `broad` for an unrestricted
   pass, or one explicit focused profile. Select `max` reasoning for calibration.
   Leave **review head sha** empty to review the current PR head. To replay an
   earlier state, enter the exact full SHA of a commit in that PR's history.
5. Start the workflow.

After local harness tests pass, the review job constructs the packet, calls
Fireworks, validates the returned JSON, and creates or updates one marked
comment on the pull request. Malformed output fails the workflow rather than
publishing a misleading review. If a valid report is too large for a GitHub
comment, the workflow publishes a severity-prioritized bounded view and retains
the complete report in the run artifacts.

A historical replay is visibly labelled with both the reviewed SHA and the
current or final PR head. It uses a SHA-specific comment marker, so it neither
overwrites nor masquerades as the ordinary current-head review.

Design and code comments have separate identities. Repeating the same phase and
resolved profile updates that review, while a different phase or focus remains
alongside it. A duplicate in-progress dispatch for the same pull request,
phase, and requested profile is cancelled; unrelated review passes are not.

## Path-routed focused profiles

The `auto` selector examines only the changed paths already included in the
review packet and chooses the first matching route in this checked-in priority
order. Renames are matched using both their previous and destination paths so a
move cannot erase the risk classification:

| Priority | Profile | Typical paths and risk |
|---|---|---|
| 1 | `foundational-consistency` | Charter, frozen Core rules, decisions, metatheory, audit and failure-class contracts |
| 2 | `kernel-soundness` | Checker, conversion, substitution, normalization, syntax, and conformance tests |
| 3 | `schema-encoding` | Schemas, manifests, canonical printing and bytes, hashes, and format contracts |
| 4 | `parser-boundary` | Parser, malformed input, lexical rules, error ownership, and rejection fixtures |
| 5 | `ci-supply-chain` | Workflows, scripts, dependency and toolchain pins, permissions, and credential boundaries |

If no route matches, `auto` falls back to `broad`. If several routes match, the
first one is selected and every match is retained in the audit metadata. An
explicit profile bypasses routing, which keeps historical calibration and
controlled comparisons possible. If the changed-file list reaches its reviewed
bound, the collector probes for one additional file: equality is complete,
while actual overflow makes `auto` fail closed rather than select from an
incomplete path set. The operator must then raise the bound in a reviewed
change or select an explicit profile.

Every focused profile appends a checked-in prompt to the same base reviewer
contract and selected phase prompt. The `schema-encoding` profile, for example,
concentrates on textual grammars, JSON Schema, Unicode and UTF-8 boundaries,
regex witnesses, canonical bytes, ordering, and discrepancies among prose,
schemas, fixtures, and author claims.

Profile selection does not change packet assembly. Replaying the same PR head
with the same repository state therefore holds the model, reasoning effort,
and packet constant while changing only the prompt. Focused comments use a
profile-specific marker and remain alongside the broad result.

The harness consumes Fireworks' server-sent event stream while discarding
private reasoning chunks and retaining only final assistant content. This keeps
long reasoning requests from leaving an idle HTTP connection. A retryable HTTP
or network failure before any streamed output is retried once after a short
delay; an interrupted partial stream is never retried automatically.

The completion allowance is pinned to `131072`, GLM-5.3-Flash's documented
maximum. Reasoning tokens and final answer tokens share that allowance; the
full limit gives `max` reasoning room to terminate and emit the required JSON.
The strict report validator and bounded GitHub renderer still constrain what
can be published.

## Usage and cost audit

The streaming request asks Fireworks to include its token-usage object. Because
earlier stream events can contain provisional counters, the harness treats the
last usage event before the stream terminator as Fireworks' authoritative final
totals. A successful review fails closed if that final record is missing,
malformed, or internally inconsistent. The harness records:

- prompt, cached-prompt, uncached-prompt, completion, and total token counts;
- the pricing snapshot used for estimation;
- a six-decimal estimated USD cost;
- provider, model, reasoning effort, phase, requested and resolved profiles,
  prompt and packet hashes, exact SHAs, workflow run, and UTC timestamp.

The complete record is retained as `review-usage.json` and inside
`review-result.json`; a compact version appears in the PR comment. The pinned
estimate uses the model-page rates current when this harness was reviewed:
$0.15/M input, $0.03/M cached input, and $0.50/M output tokens. Fireworks billing
remains authoritative, so a pricing change requires a reviewed update to the
constants and documentation rather than silently changing historical reports.

The workflow remains manually dispatched and has a 45-minute job timeout. It
does not maintain cross-run account state or attempt to enforce a monthly
provider budget; the operator must track and enforce the monthly limit
separately from this per-run audit record.

## Foundational firewall

Ordinary implementation defects receive `P0` through `P3` findings. If a
proposed resolution would change a frozen Core v0.1 judgment, computation rule,
universe rule, trusted boundary, or serialized meaning, the only permitted
verdict is `foundational_stop`. Such a result requires an explicit design
decision and theory-version analysis; it is never treated as an automatic code
fix.

A concrete interoperability divergence or undefined canonical byte sequence is
at least `P2`, never a nonblocking `P3`. Before returning, the reviewer must
cross-check its summary against its verdict and findings. These prompt rules
encode lessons from the first historical calibration, while the strict schema
still enforces the relationships that are mechanically decidable.

## Calibration evidence

The first blind historical replay reviewed PR #3 at
`590150e8fc6504b92873f0ad68f070c92e120138`, which contained a known Unicode
surrogate defect. The broad pass missed it. With the packet, model, reasoning
effort, and historical SHA held constant, the `schema-encoding` pass found the
exact `a\uD800b` witness and the adjacent failure-class gap. It nevertheless
downgraded the interoperability defect to `P3` and produced a summary that did
not cleanly acknowledge its own finding.

That result established the operating policy encoded here: broad review is a
search for surprises, focused review protects attention on the changed risk
area, mechanically checkable invariants belong in deterministic tests, and all
model output remains advisory. PR #5 and later clean revisions remain useful
false-positive controls.

Continue to grade historical or seeded cases without exposing their known
answers in the packet. Record detection, false positives, severity accuracy,
foundational-firewall handling, runtime, reported tokens, and estimated and
billed cost whenever the model, prompt, routes, or pricing snapshot changes.
