Query rules APIs

Query rules allow you to configure per-query rules that are applied at query time to queries that match the specific rule. Query rules are organized into rulesets, collections of query rules that are matched against incoming queries. Query rules are applied using the rule query.

If a query matches one or more rules in the ruleset, the query is re-written to apply the rules before searching. This allows pinning documents for only queries that match a specific term.
