# Independent adversarial review pilot

This repository can ask Z.ai's `GLM-5.3-Flash` model to perform a blind,
advisory review of a pull request. The reviewer is deliberately outside the
trusted boundary: its claims acquire weight only through exact evidence,
reproduction, tests, and human resolution.

The pilot is manual. It does not run on every pull request and it never changes
mergeability or branch protection.

## Security and review boundary

- The workflow runs only when manually dispatched from `main`.
- The initial pilot accepts same-repository pull requests only; fork pull
  requests fail closed.
- It reads pull-request data through the GitHub API. It does not check out or
  execute pull-request code.
- The Z.ai key is sent only to an HTTPS `api.z.ai` chat-completions endpoint.
- Pull-request text is marked as untrusted data in the reviewer prompt, and no
  tools are exposed to the model.
- The model receives the unified diff, bounded complete changed-file contents,
  and the frozen contracts as they existed at the pull request's base commit.
- Existing comments and reviews are excluded to reduce anchoring and preserve
  an independent first pass.
- The result is tied to the exact base and head SHAs. The prompt and input
  packet are SHA-256 identified, and the complete packet and structured report
  are retained as workflow artifacts for 30 days.
- The harness rechecks both SHAs after packet assembly and immediately before
  publication. A concurrent push or base change aborts the run as stale.
- Model output is advisory. CI and the project owner remain the merge gate.

The repository is public, so the selected source and specification content is
already public. Do not copy this pilot to a private repository without making
an explicit decision about sending private source code to the API provider.

## One-time setup

1. Create a key in the [Z.ai API Keys page](https://z.ai/manage-apikey/account).
2. In this repository, open **Settings → Secrets and variables → Actions**.
3. Create a repository secret named `ZAI_API_KEY` containing the key.

The default endpoint is the standard API:

`https://api.z.ai/api/paas/v4/chat/completions`

If the key belongs to a GLM Coding Plan that requires its dedicated endpoint,
create an Actions repository variable named `ZAI_API_URL` with:

`https://api.z.ai/api/coding/paas/v4/chat/completions`

The model defaults to `glm-5.3-flash`. An optional `ZAI_MODEL` repository
variable can change the model while retaining the same validated report
contract.

## Run a review

1. Open **Actions → Adversarial Review → Run workflow**.
2. Select the `main` branch.
3. Enter the pull request number and select `max` reasoning for calibration.
4. Start the workflow.

After local harness tests pass, the review job constructs the packet, calls
Z.ai, validates the returned JSON, and creates or updates one marked comment on
the pull request. Malformed output fails the workflow rather than publishing a
misleading review. If a valid report is too large for a GitHub comment, the
workflow publishes a severity-prioritized bounded view and retains the complete
report in the run artifacts.

## Foundational firewall

Ordinary implementation defects receive `P0` through `P3` findings. If a
proposed resolution would change a frozen Core v0.1 judgment, computation rule,
universe rule, trusted boundary, or serialized meaning, the only permitted
verdict is `foundational_stop`. Such a result requires an explicit design
decision and theory-version analysis; it is never treated as an automatic code
fix.

## Initial calibration

The first runs should target historical pull requests rather than a live merge:

- PR #3, which has a known Unicode-format defect found during its original
  review;
- PR #5, a larger implementation PR that should test false-positive control.

Do not show the model the original review comments before comparing its report
with the known findings. Record whether each result is a true defect, a useful
test proposal, a false positive, or an attempted change to frozen theory.
