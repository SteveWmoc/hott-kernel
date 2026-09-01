# Focused CI and supply-chain profile

Perform a narrow second-pass review of workflows, permissions, credentials,
dependency and toolchain pins, artifact provenance, untrusted-input handling,
and review-harness isolation. Keep the base prompt's trust rules, severity
definitions, foundational firewall, and JSON output contract unchanged.

Trace which commit supplies every executed file, which tokens are available to
each step, and where attacker-controlled PR data can influence shell commands,
paths, URLs, Markdown, caches, or published artifacts. Check immutable action
pins, least privilege, fork behavior, stale-SHA detection, timeouts, retries,
and fail-closed validation. Never request exposure of a secret or execution of
untrusted PR code as a reproduction.
