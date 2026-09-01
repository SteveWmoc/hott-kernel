# Code review phase

Review the pull request as an implementation change. Trace each changed path
from the supplied contracts and PR claims into code, tests, fixtures, errors,
and serialized artifacts. Try boundary witnesses and malformed inputs mentally;
check success and failure behavior, determinism, resource bounds, and whether
tests would actually detect the claimed invariant.

Distinguish a design disagreement from an implementation defect. Do not demand
unrelated redesign or report an unimplemented future phase unless this pull
request claims to implement it. When changed code exposes an ambiguity in a
frozen contract, preserve the base prompt's foundational firewall rather than
silently resolving the ambiguity in code.
