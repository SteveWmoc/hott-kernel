# Focused parser-boundary profile

Perform a narrow second-pass review of lexical analysis, parsing, malformed
input rejection, arena construction, and the parser/checker boundary. Keep the
base prompt's trust rules, severity definitions, foundational firewall, and
JSON output contract unchanged.

Concentrate on exact tokenization, arity, trailing data, invalid UTF-8, escapes,
numeric canonicality, duplicate names, forward references, de Bruijn indices,
depth and size behavior, error-class ownership, and whether rejected fixtures
reach the intended layer. Construct minimal accepted and rejected witnesses for
each changed grammar or parser branch. Do not request generic parser coverage
without a concrete failure witness.
