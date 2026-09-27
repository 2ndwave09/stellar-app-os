# Historical carbon data API (issue #1409)

The read-only endpoint `GET /api/v2/historical-carbon-data` returns a market-wide historical view built from the carbon price catalog.

## Query parameters

| Parameter | Required | Description |
| --- | --- | --- |
| `from` | No | Inclusive UTC date in `YYYY-MM-DD` format. Defaults to the beginning of the selected window. |
| `to` | No | Inclusive UTC date in `YYYY-MM-DD` format. Defaults to the current UTC date. |
| `interval` | No | `day`, `week`, or `month`; defaults to `month`. |
| `regions` / `region` | No | Comma-separated catalog region values. Unknown values return `400`. |
| `projectTypes` / `projectType` | No | Comma-separated project-type values. Unknown values return `400`. |

The maximum requested range is 730 days. If neither date is supplied, the endpoint returns the latest 365-day window. `from` must not be later than `to`.

## Response sections

A successful response includes:

- `priceHistory`: interval buckets with volume-weighted average price, listed volume, and series count.
- `projectPerformance`: per-project first, last, average, and percentage price change plus volume.
- `buyerTrends`: aggregate volume trends grouped by region and project type.
- `filters`, `from`, `to`, `interval`, and `generatedAt` for reproducibility.

Buyer trends use `listed_volume_tonnes` as the activity metric. The endpoint does not expose buyer identities or individual transactions, and the response identifies its current source as `catalog`.

## Examples

```text
GET /api/v2/historical-carbon-data?interval=month&regions=africa&from=2026-01-01&to=2026-06-30
GET /api/v2/historical-carbon-data?interval=week&projectType=Reforestation
```

Invalid dates, unsupported intervals, unknown filters, and ranges longer than 730 days return HTTP `400` with an `errors` array. Unexpected processing failures return HTTP `500`. Successful responses are publicly cacheable for five minutes and may be served stale for up to ten additional minutes.

This API is analytical and read-only. It does not modify credit balances, marketplace state, or on-chain records.
