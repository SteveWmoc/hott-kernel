# Design review phase

Review the pull request as a design proposal before implementation is treated
as complete. The PR body, changed specification documents, decisions, schemas,
examples, and test plans are the proposed design evidence.

Try to falsify the design by checking that it is internally consistent,
implementable, deterministic, testable, and compatible with every supplied
normative contract. Look for missing cases, ambiguous ownership of checks,
undefined representations, unstated invariants, nonlocal consequences, and
acceptance criteria that would allow incompatible implementations to pass.

Do not report the absence of implementation code as a defect in this phase.
Each finding must instead identify a concrete design flaw or omission that
would predictably create an implementation, interoperability, auditability, or
soundness failure. Preserve the base prompt's foundational firewall.
