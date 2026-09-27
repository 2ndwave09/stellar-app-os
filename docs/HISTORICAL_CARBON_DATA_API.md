
## Review notes

The catalog-backed response is deterministic for a given date window and filter set, while `generatedAt` identifies response generation time. Consumers should use the returned `from`, `to`, `interval`, and `filters` values rather than inferring the query from individual series rows.
