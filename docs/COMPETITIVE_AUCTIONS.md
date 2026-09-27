# Competitive auctions (issue #1404)

The marketplace now exposes a competitive-auction lifecycle in addition to the existing Dutch auction contract API.

## Lifecycle

1. `POST /api/marketplace/auctions` creates a full-lot auction with a reserve price and closing time.
2. `POST /api/marketplace/auctions/:id/bids` records a full-lot bid. A bid must meet the reserve and exceed the current leader; ties are rejected, so the leading bid is deterministic.
3. `GET /api/marketplace/auctions/:id` returns the public bid history and current leader for price discovery.
4. Once the end time passes, the seller calls `POST /api/marketplace/auctions/:id/finalize`. The highest bid becomes the winner and prior leaders remain refundable.

The service rejects seller self-bids, bids after close, non-increasing prices, invalid quantities, unauthorized finalization, and repeated refund withdrawal. Refunds are represented explicitly as `refundable`/`refunded` bid states so a payment adapter can safely connect the lifecycle to Stellar escrow in a later deployment step.

The repository is intentionally behind an interface (`InMemoryCompetitiveAuctionRepository`) while the existing marketplace contract is undergoing cleanup from historical merged feature branches. Before production settlement, replace the repository with a durable/PostgreSQL or Soroban escrow adapter and make finalization atomically transfer the winning payment and TREE credits. No API response claims that funds have moved on-chain.
