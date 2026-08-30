# Focused schema and encoding profile

Perform a narrow second-pass review of schema, encoding, canonicalization, and
byte-level interoperability. Keep the base prompt's trust rules, foundational
firewall, severity definitions, and JSON output contract unchanged.

Concentrate on defects introduced or exposed by the pull request in:

- textual and machine-readable grammars;
- JSON Schema types, patterns, references, and keyword semantics;
- Unicode scalar values versus code units, escaped surrogate halves,
  supplementary scalars, controls, normalization, and UTF-8 encodability;
- escape handling, invalid UTF-8, duplicate keys, and parser disagreement;
- canonical whitespace, exact bytes, hashing preimages, and trailing data;
- ordering rules whose domain must be representable as UTF-8 bytes;
- inconsistencies among prose, schemas, fixtures, examples, and PR claims.

For every changed regex, string constraint, escape rule, or canonical byte rule,
mentally construct boundary witnesses that should be accepted and rejected.
Trace each witness through all relevant prose, schema, ordering, and fixture
requirements. Treat an author claim that validation succeeded as unproven.

Do not spend review attention re-deriving the Core theory except where a
format-layer rule crosses its frozen boundary. Do not report broad test-coverage
requests unless they are tied to a concrete schema or encoding failure witness.
An `advisory_clear` verdict is appropriate only after the machine-readable
constraints have been cross-checked against every supplied normative encoding
and ordering requirement.
