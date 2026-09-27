# Soroban Contract Interface Reference

> This document is the repository’s OpenAPI-style contract interface catalog. It is generated from the checked-in Rust sources so function names, parameter types, return types, public data types, and event topics remain reviewable alongside the ABI-producing code.

## Interface conventions

| Field | Meaning |
|---|---|
| Function | Soroban contract entry point exposed by `#[contractimpl]`. |
| Parameters | Ordered ABI parameters, including the mandatory `Env` receiver omitted from the public call shape. |
| Returns | Rust return type encoded by the Soroban SDK. `()` means no return value. |
| Events | Topic labels observed in `env.events().publish`; event data is the tuple published by the source. |

## Contract index

| Contract | Functions | Types | Events |
|---|---:|---:|---:|
| `admin-controls` | 15 | 3 | 2 |
| `aggregate-impact-verifier` | 7 | 4 | 0 |
| `atomic-swap` | 5 | 5 | 0 |
| `auth-contract` | 7 | 3 | 0 |
| `carbon-credits` | 16 | 6 | 0 |
| `carbon-dex` | 14 | 5 | 0 |
| `carbon-marketplace` | 57 | 14 | 10 |
| `carbon-price-oracle` | 7 | 3 | 0 |
| `community-offset-pool` | 5 | 4 | 0 |
| `contract-utils` | 6 | 0 | 0 |
| `donation-escrow` | 27 | 11 | 0 |
| `escrow` | 20 | 5 | 0 |
| `escrow-milestone` | 23 | 7 | 3 |
| `farmer-registry` | 30 | 9 | 0 |
| `farmer-wallet` | 29 | 11 | 6 |
| `harvesta-errors` | 0 | 4 | 0 |
| `kyc-attestation` | 14 | 6 | 0 |
| `leaderboard` | 13 | 5 | 0 |
| `location-proof` | 8 | 4 | 0 |
| `naira-payout` | 12 | 5 | 0 |
| `nft-certificate` | 20 | 5 | 6 |
| `nullifier-registry` | 9 | 5 | 0 |
| `planter-blacklist` | 6 | 2 | 0 |
| `planter-registry` | 18 | 6 | 7 |
| `planting-bond` | 6 | 4 | 0 |
| `platform-governance` | 53 | 12 | 4 |
| `public-seal` | 14 | 6 | 0 |
| `species-catalog` | 16 | 9 | 0 |
| `species-registry` | 4 | 2 | 0 |
| `species-voting` | 11 | 4 | 0 |
| `sponsor-receipt` | 18 | 5 | 2 |
| `staking-rewards` | 9 | 3 | 2 |
| `subscription-sponsorship` | 7 | 3 | 0 |
| `transparent-proxy` | 4 | 3 | 1 |
| `treasury` | 10 | 3 | 1 |
| `tree-escrow` | 22 | 7 | 0 |
| `tree-genetics` | 8 | 4 | 0 |
| `tree-registry` | 35 | 8 | 0 |
| `tree-retirement` | 13 | 4 | 2 |
| `tree-token` | 22 | 7 | 4 |
| `tree_registry` | 15 | 5 | 3 |
| `upgrade-timelock` | 9 | 4 | 0 |
| `verifier-staking` | 27 | 7 | 3 |
| `zk-location-verifier` | 12 | 5 | 0 |
| `zk-verifier` | 10 | 6 | 0 |

## Contract interfaces

### `admin-controls`

#### Functions

| Name | ABI signature |
|---|---|
| `is_none` | `fn is_none(&self) -> bool` |
| `initialize` | `fn initialize(env: Env, admin: Address, oracle: Address)` |
| `pause` | `fn pause(env: Env)` |
| `unpause` | `fn unpause(env: Env)` |
| `is_paused` | `fn is_paused(env: Env) -> bool` |
| `assert_not_paused` | `fn assert_not_paused(env: Env)` |
| `update_oracle` | `fn update_oracle(env: Env, new_oracle: Address)` |
| `get_oracle` | `fn get_oracle(env: Env) -> Address` |
| `add_to_whitelist` | `fn add_to_whitelist(env: Env, addr: Address)` |
| `remove_from_whitelist` | `fn remove_from_whitelist(env: Env, addr: Address)` |
| `is_whitelisted` | `fn is_whitelisted(env: Env, addr: Address) -> bool` |
| `assert_whitelisted` | `fn assert_whitelisted(env: Env, addr: Address)` |
| `propose_admin` | `fn propose_admin(env: Env, new_admin: Address)` |
| `accept_admin` | `fn accept_admin(env: Env)` |
| `get_config` | `fn get_config(env: Env) -> AdminConfig` |

#### Public types

| Type | Encoding source |
|---|---|
| `AdminConfig` | `#[contracttype]` or public Rust type in `contracts/admin-controls/src/lib.rs` |
| `AdminControls` | `#[contracttype]` or public Rust type in `contracts/admin-controls/src/lib.rs` |
| `OptAddress` | `#[contracttype]` or public Rust type in `contracts/admin-controls/src/lib.rs` |

#### Event topics

| Topic | Source |
|---|---|
| `WLAdd` | `env.events().publish` in `contracts/admin-controls/src/lib.rs` |
| `WLRemove` | `env.events().publish` in `contracts/admin-controls/src/lib.rs` |

### `aggregate-impact-verifier`

#### Functions

| Name | ABI signature |
|---|---|
| `initialize` | `fn initialize(env: Env, admin: Address)` |
| `submit_aggregate_proof` | `fn submit_aggregate_proof(env: Env, proof_digest: BytesN<32>, stats: AggregateStats)` |
| `revoke_proof` | `fn revoke_proof(env: Env, proof_digest: BytesN<32>)` |
| `get_proof` | `fn get_proof(env: Env, proof_digest: BytesN<32>) -> Option<AggregateProofRecord>` |
| `get_proof_at` | `fn get_proof_at(env: Env, idx: u64) -> Option<BytesN<32>>` |
| `proof_count` | `fn proof_count(env: Env) -> u64` |
| `is_valid_proof` | `fn is_valid_proof(env: Env, proof_digest: BytesN<32>) -> bool` |

#### Public types

| Type | Encoding source |
|---|---|
| `AggregateError` | `#[contracttype]` or public Rust type in `contracts/aggregate-impact-verifier/src/lib.rs` |
| `AggregateImpactVerifier` | `#[contracttype]` or public Rust type in `contracts/aggregate-impact-verifier/src/lib.rs` |
| `AggregateProofRecord` | `#[contracttype]` or public Rust type in `contracts/aggregate-impact-verifier/src/lib.rs` |
| `AggregateStats` | `#[contracttype]` or public Rust type in `contracts/aggregate-impact-verifier/src/lib.rs` |

#### Event topics

| Topic | Source |
|---|---|
| — | No inline `symbol_short!` event topics detected. |

### `atomic-swap`

#### Functions

| Name | ABI signature |
|---|---|
| `initialize` | `fn initialize(env: Env, admin: Address)` |
| `propose_swap` | `fn propose_swap( env: Env, initiator: Address, counterparty: Address, amount: i128, hashlock: BytesN<32>, expiry: u64, dest_chain: String, dest_address: String, ) -> u64` |
| `complete_swap` | `fn complete_swap(env: Env, swap_id: u64, secret: Bytes)` |
| `refund_swap` | `fn refund_swap(env: Env, swap_id: u64)` |
| `get_swap` | `fn get_swap(env: Env, swap_id: u64) -> Swap` |

#### Public types

| Type | Encoding source |
|---|---|
| `AtomicSwapContract` | `#[contracttype]` or public Rust type in `contracts/atomic-swap/src/lib.rs` |
| `DataKey` | `#[contracttype]` or public Rust type in `contracts/atomic-swap/src/lib.rs` |
| `Error` | `#[contracttype]` or public Rust type in `contracts/atomic-swap/src/lib.rs` |
| `Swap` | `#[contracttype]` or public Rust type in `contracts/atomic-swap/src/lib.rs` |
| `SwapStatus` | `#[contracttype]` or public Rust type in `contracts/atomic-swap/src/lib.rs` |

#### Event topics

| Topic | Source |
|---|---|
| — | No inline `symbol_short!` event topics detected. |

### `auth-contract`

#### Functions

| Name | ABI signature |
|---|---|
| `initialize` | `fn initialize(env: Env, admin: Address)` |
| `register_signer` | `fn register_signer(env: Env, admin: Address, signer: Address)` |
| `revoke_signer` | `fn revoke_signer(env: Env, admin: Address, signer: Address)` |
| `is_signer` | `fn is_signer(env: Env, signer: Address) -> bool` |
| `get_signer` | `fn get_signer(env: Env, signer: Address) -> Option<SignerMeta>` |
| `signer_ping` | `fn signer_ping(env: Env, caller: Address, nonce: u64)` |
| `get_admin` | `fn get_admin(env: Env) -> Address` |

#### Public types

| Type | Encoding source |
|---|---|
| `AuthContract` | `#[contracttype]` or public Rust type in `contracts/auth-contract/src/lib.rs` |
| `Error` | `#[contracttype]` or public Rust type in `contracts/auth-contract/src/lib.rs` |
| `SignerMeta` | `#[contracttype]` or public Rust type in `contracts/auth-contract/src/lib.rs` |

#### Event topics

| Topic | Source |
|---|---|
| — | No inline `symbol_short!` event topics detected. |

### `carbon-credits`

#### Functions

| Name | ABI signature |
|---|---|
| `initialize` | `fn initialize(env: Env, admin: Address)` |
| `set_rate` | `fn set_rate(env: Env, slug: Symbol, co2_scaled: i128, maturity_years: u32)` |
| `get_rate` | `fn get_rate(env: Env, slug: Symbol) -> SpeciesRate` |
| `estimate_offset` | `fn estimate_offset(env: Env, slug: Symbol, age_years: u32) -> u64` |
| `record_credit` | `fn record_credit( env: Env, sponsor: Address, slug: Symbol, tree_count: u32, age_years: u32, )` |
| `total_offset_for_sponsor` | `fn total_offset_for_sponsor(env: Env, sponsor: Address) -> u64` |
| `retire_offset` | `fn retire_offset(env: Env, sponsor: Address, amount: u64)` |
| `total_retired_for_sponsor` | `fn total_retired_for_sponsor(env: Env, sponsor: Address) -> u64` |
| `set_regional_baseline` | `fn set_regional_baseline( env: Env, region: Symbol, baseline_co2: i128, reference_year: u32, methodology_digest: BytesN<32>, )` |
| `get_regional_baseline` | `fn get_regional_baseline(env: Env, region: Symbol) -> RegionalBaseline` |
| `set_min_reduction_bps` | `fn set_min_reduction_bps(env: Env, bps: u32)` |
| `get_min_reduction_bps` | `fn get_min_reduction_bps(env: Env) -> u32` |
| `record_project_emissions` | `fn record_project_emissions( env: Env, project_id: Symbol, region: Symbol, project_co2: i128, period_start: u64, period_end: u64, )` |
| `get_project_emissions` | `fn get_project_emissions(env: Env, project_id: Symbol) -> ProjectEmissions` |
| `evaluate_additionality` | `fn evaluate_additionality(env: Env, project_id: Symbol) -> AdditionalityVerdict` |
| `record_verified_credit` | `fn record_verified_credit( env: Env, sponsor: Address, project_id: Symbol, slug: Symbol, tree_count: u32, age_years: u32, )` |

#### Public types

| Type | Encoding source |
|---|---|
| `AdditionalityVerdict` | `#[contracttype]` or public Rust type in `contracts/carbon-credits/src/lib.rs` |
| `CarbonCredits` | `#[contracttype]` or public Rust type in `contracts/carbon-credits/src/lib.rs` |
| `CarbonCreditsError` | `#[contracttype]` or public Rust type in `contracts/carbon-credits/src/lib.rs` |
| `ProjectEmissions` | `#[contracttype]` or public Rust type in `contracts/carbon-credits/src/lib.rs` |
| `RegionalBaseline` | `#[contracttype]` or public Rust type in `contracts/carbon-credits/src/lib.rs` |
| `SpeciesRate` | `#[contracttype]` or public Rust type in `contracts/carbon-credits/src/lib.rs` |

#### Event topics

| Topic | Source |
|---|---|
| — | No inline `symbol_short!` event topics detected. |

### `carbon-dex`

#### Functions

| Name | ABI signature |
|---|---|
| `initialize` | `fn initialize(env: Env, admin: Address)` |
| `deposit` | `fn deposit(env: Env, from: Address, token: Address, amount: i128) -> i128` |
| `withdraw` | `fn withdraw(env: Env, from: Address, token: Address, share_amount: i128) -> i128` |
| `pause` | `fn pause(env: Env)` |
| `unpause` | `fn unpause(env: Env)` |
| `get_pool` | `fn get_pool(env: Env, token: Address) -> Option<PoolState>` |
| `get_position` | `fn get_position(env: Env, token: Address, provider: Address) -> i128` |
| `create_pair` | `fn create_pair(env: Env, token_a: Address, token_b: Address, fee_bps: u32)` |
| `add_liquidity` | `fn add_liquidity( env: Env, from: Address, token_a: Address, token_b: Address, amount_a: i128, amount_b: i128, ) -> i128` |
| `remove_liquidity` | `fn remove_liquidity( env: Env, from: Address, token_a: Address, token_b: Address, shares: i128, ) -> (i128, i128)` |
| `quote_swap` | `fn quote_swap(env: Env, token_a: Address, token_b: Address, amount_in: i128) -> i128` |
| `swap` | `fn swap( env: Env, from: Address, token_a: Address, token_b: Address, amount_in: i128, min_amount_out: i128, ) -> i128` |
| `get_pair` | `fn get_pair(env: Env, token_a: Address, token_b: Address) -> Option<PairState>` |
| `get_pair_position` | `fn get_pair_position( env: Env, token_a: Address, token_b: Address, provider: Address, ) -> i128` |

#### Public types

| Type | Encoding source |
|---|---|
| `CarbonDexContract` | `#[contracttype]` or public Rust type in `contracts/carbon-dex/src/lib.rs` |
| `DataKey` | `#[contracttype]` or public Rust type in `contracts/carbon-dex/src/lib.rs` |
| `Error` | `#[contracttype]` or public Rust type in `contracts/carbon-dex/src/lib.rs` |
| `PairState` | `#[contracttype]` or public Rust type in `contracts/carbon-dex/src/lib.rs` |
| `PoolState` | `#[contracttype]` or public Rust type in `contracts/carbon-dex/src/lib.rs` |

#### Event topics

| Topic | Source |
|---|---|
| — | No inline `symbol_short!` event topics detected. |

### `carbon-marketplace`

#### Functions

| Name | ABI signature |
|---|---|
| `initialize` | `fn initialize( env: Env, admin: Address, tree_token: Address, payment_token: Address, )` |
| `pause` | `fn pause(env: Env)` |
| `unpause` | `fn unpause(env: Env)` |
| `is_paused` | `fn is_paused(env: Env) -> bool` |
| `get_fee_bps` | `fn get_fee_bps(env: Env, amount_in: i128) -> i128` |
| `list` | `fn list( env: Env, seller: Address, planter: Address, amount: i128, price_per_token: i128, payment_token: Address, ) -> u64` |
| `buy` | `fn buy(env: Env, buyer: Address, listing_id: u64, amount: i128)` |
| `cancel` | `fn cancel(env: Env, seller: Address, listing_id: u64)` |
| `get_listing` | `fn get_listing(env: Env, listing_id: u64) -> Listing` |
| `amm_add_liquidity` | `fn amm_add_liquidity( env: Env, provider: Address, tree_amount: i128, payment_amount: i128, ) -> i128` |
| `amm_remove_liquidity` | `fn amm_remove_liquidity( env: Env, provider: Address, lp_shares: i128, ) -> (i128, i128)` |
| `amm_swap_exact_in` | `fn amm_swap_exact_in( env: Env, caller: Address, token_in: Address, amount_in: i128, min_amount_out: i128, ) -> i128` |
| `amm_get_quote` | `fn amm_get_quote(env: Env, token_in: Address, amount_in: i128) -> i128` |
| `amm_pool_info` | `fn amm_pool_info(env: Env) -> AmmPool` |
| `amm_lp_balance` | `fn amm_lp_balance(env: Env, provider: Address) -> i128` |
| `initialize` | `fn initialize(env: Env, admin: Address, tree_token: Address, admin_controls: Address)` |
| `configure_price_oracle` | `fn configure_price_oracle(env: Env, oracle: Address, max_staleness: u64, fallback_price: i128)` |
| `get_min_trade_size` | `fn get_min_trade_size(env: &Env) -> i128` |
| `set_min_trade_size` | `fn set_min_trade_size(env: Env, min_size: i128)` |
| `get_dynamic_price` | `fn get_dynamic_price(env: Env) -> i128` |
| `configure_auction` | `fn configure_auction( env: Env, starting_price: i128, reserve_price: i128, decay_rate: u64, duration: u64, )` |
| `list` | `fn list( env: Env, seller: Address, planter: Address, amount: i128, price_per_token: i128, payment_token: Address, ) -> u64` |
| `buy` | `fn buy(env: Env, buyer: Address, listing_id: u64, amount: i128)` |
| `cancel` | `fn cancel(env: Env, seller: Address, listing_id: u64)` |
| `admin_cancel` | `fn admin_cancel(env: Env, listing_id: u64)` |
| `get_listing` | `fn get_listing(env: Env, listing_id: u64) -> Option<Listing>` |
| `listing_count` | `fn listing_count(env: Env) -> u64` |
| `create_auction` | `fn create_auction(env: Env, seller: Address, planter: Address, amount: i128, payment_token: Address) -> u64` |
| `bid` | `fn bid(env: Env, buyer: Address, auction_id: u64, amount: i128)` |
| `cancel_auction` | `fn cancel_auction(env: Env, seller: Address, auction_id: u64)` |
| `get_auction` | `fn get_auction(env: Env, auction_id: u64) -> Option<DutchAuction>` |
| `get_current_price` | `fn get_current_price(env: Env, auction_id: u64) -> i128` |
| `auction_count` | `fn auction_count(env: Env) -> u64` |
| `place_buy_order` | `fn place_buy_order( env: Env, buyer: Address, payment_token: Address, amount: i128, max_price_per_token: i128, ) -> u64` |
| `place_sell_order` | `fn place_sell_order( env: Env, seller: Address, planter: Address, amount: i128, min_price_per_token: i128, ) -> u64` |
| `cancel_order` | `fn cancel_order(env: Env, caller: Address, order_id: u64)` |
| `get_order` | `fn get_order(env: Env, order_id: u64) -> Option<Order>` |
| `order_count` | `fn order_count(env: Env) -> u64` |
| `set_royalty` | `fn set_royalty(env: Env, basis_points: u32)` |
| `get_royalty` | `fn get_royalty(env: Env) -> u32` |
| `validate_order_trade` | `fn validate_order_trade(env: Env, seller: Address, buyer: Address)` |
| `validate_order_matching` | `fn validate_order_matching(env: Env, buy_owner: Address, sell_owner: Address)` |
| `extend_instance_ttl` | `fn extend_instance_ttl(env: Env, threshold: u32, extend_to: u32)` |
| `bump_instance_ttl` | `fn bump_instance_ttl(env: Env)` |
| `configure_twap` | `fn configure_twap(env: Env, period_seconds: u64, max_observations: u32)` |
| `get_cumulative_observation` | `fn get_cumulative_observation(env: Env) -> Option<CumulativeObservation>` |
| `get_twap` | `fn get_twap(env: Env, observation_count: u32) -> Option<i128>` |
| `get_twap_config` | `fn get_twap_config(env: Env) -> Option<TwapConfig>` |
| `get_total_observations` | `fn get_total_observations(env: Env) -> u64` |
| `configure_treasury_reserve` | `fn configure_treasury_reserve( env: Env, fee_token: Address, usdc_reserve: Address, fee_bps: u32, )` |
| `swap_fees_to_usdc` | `fn swap_fees_to_usdc(env: Env)` |
| `get_treasury_reserve` | `fn get_treasury_reserve(env: Env) -> Option<TreasuryReserveConfig>` |
| `get_accumulated_fees` | `fn get_accumulated_fees(env: Env) -> i128` |
| `initialize` | `fn initialize(env: Env, price: i128, timestamp: u64)` |
| `set_price` | `fn set_price(env: Env, price: i128, timestamp: u64)` |
| `price` | `fn price(env: Env) -> i128` |
| `timestamp` | `fn timestamp(env: Env) -> u64` |

#### Public types

| Type | Encoding source |
|---|---|
| `AmmPool` | `#[contracttype]` or public Rust type in `contracts/carbon-marketplace/src/lib.rs` |
| `AuctionStatus` | `#[contracttype]` or public Rust type in `contracts/carbon-marketplace/src/lib.rs` |
| `CarbonMarketplace` | `#[contracttype]` or public Rust type in `contracts/carbon-marketplace/src/lib.rs` |
| `CumulativeObservation` | `#[contracttype]` or public Rust type in `contracts/carbon-marketplace/src/lib.rs` |
| `DutchAuction` | `#[contracttype]` or public Rust type in `contracts/carbon-marketplace/src/lib.rs` |
| `Listing` | `#[contracttype]` or public Rust type in `contracts/carbon-marketplace/src/lib.rs` |
| `ListingStatus` | `#[contracttype]` or public Rust type in `contracts/carbon-marketplace/src/lib.rs` |
| `LpPosition` | `#[contracttype]` or public Rust type in `contracts/carbon-marketplace/src/lib.rs` |
| `MarketplaceError` | `#[contracttype]` or public Rust type in `contracts/carbon-marketplace/src/lib.rs` |
| `Order` | `#[contracttype]` or public Rust type in `contracts/carbon-marketplace/src/lib.rs` |
| `OrderSide` | `#[contracttype]` or public Rust type in `contracts/carbon-marketplace/src/lib.rs` |
| `OrderStatus` | `#[contracttype]` or public Rust type in `contracts/carbon-marketplace/src/lib.rs` |
| `TreasuryReserveConfig` | `#[contracttype]` or public Rust type in `contracts/carbon-marketplace/src/lib.rs` |
| `TwapConfig` | `#[contracttype]` or public Rust type in `contracts/carbon-marketplace/src/lib.rs` |

#### Event topics

| Topic | Source |
|---|---|
| `adm_cncl` | `env.events().publish` in `contracts/carbon-marketplace/src/lib.rs` |
| `auct_cncl` | `env.events().publish` in `contracts/carbon-marketplace/src/lib.rs` |
| `auct_crtd` | `env.events().publish` in `contracts/carbon-marketplace/src/lib.rs` |
| `bid` | `env.events().publish` in `contracts/carbon-marketplace/src/lib.rs` |
| `cancelled` | `env.events().publish` in `contracts/carbon-marketplace/src/lib.rs` |
| `listed` | `env.events().publish` in `contracts/carbon-marketplace/src/lib.rs` |
| `ord_cncl` | `env.events().publish` in `contracts/carbon-marketplace/src/lib.rs` |
| `paused` | `env.events().publish` in `contracts/carbon-marketplace/src/lib.rs` |
| `sold` | `env.events().publish` in `contracts/carbon-marketplace/src/lib.rs` |
| `unpaused` | `env.events().publish` in `contracts/carbon-marketplace/src/lib.rs` |

### `carbon-price-oracle`

#### Functions

| Name | ABI signature |
|---|---|
| `initialize` | `fn initialize(env: Env, admin: Address, default_price_scaled: i128)` |
| `submit_price` | `fn submit_price(env: Env, source: Address, price_scaled: i128)` |
| `get_price` | `fn get_price(env: Env) -> PriceUpdate` |
| `is_stale` | `fn is_stale(env: Env) -> bool` |
| `calculate_tree_price` | `fn calculate_tree_price(env: Env, base_tree_cost_stroops: i128) -> SponsorPrice` |
| `get_carbon_rate_bps` | `fn get_carbon_rate_bps(env: Env) -> u32` |
| `update_fallback_rate` | `fn update_fallback_rate(env: Env, admin: Address, new_rate_bps: u32)` |

#### Public types

| Type | Encoding source |
|---|---|
| `CarbonPriceOracle` | `#[contracttype]` or public Rust type in `contracts/carbon-price-oracle/src/lib.rs` |
| `PriceUpdate` | `#[contracttype]` or public Rust type in `contracts/carbon-price-oracle/src/lib.rs` |
| `SponsorPrice` | `#[contracttype]` or public Rust type in `contracts/carbon-price-oracle/src/lib.rs` |

#### Event topics

| Topic | Source |
|---|---|
| — | No inline `symbol_short!` event topics detected. |

### `community-offset-pool`

#### Functions

| Name | ABI signature |
|---|---|
| `initialize` | `fn initialize(env: Env, creator: Address, payment_token: Address, credit_token: Address, target_amount: i128)` |
| `contribute` | `fn contribute(env: Env, wallet: Address, amount: i128)` |
| `finalize` | `fn finalize(env: Env, caller: Address, total_credits: i128)` |
| `claim` | `fn claim(env: Env, wallet: Address)` |
| `get_pool` | `fn get_pool(env: Env) -> Pool` |

#### Public types

| Type | Encoding source |
|---|---|
| `CommunityOffsetPool` | `#[contracttype]` or public Rust type in `contracts/community-offset-pool/src/lib.rs` |
| `Member` | `#[contracttype]` or public Rust type in `contracts/community-offset-pool/src/lib.rs` |
| `Pool` | `#[contracttype]` or public Rust type in `contracts/community-offset-pool/src/lib.rs` |
| `PoolError` | `#[contracttype]` or public Rust type in `contracts/community-offset-pool/src/lib.rs` |

#### Event topics

| Topic | Source |
|---|---|
| — | No inline `symbol_short!` event topics detected. |

### `contract-utils`

#### Functions

| Name | ABI signature |
|---|---|
| `add_to_whitelist` | `fn add_to_whitelist(env: &Env, addr: &Address)` |
| `remove_from_whitelist` | `fn remove_from_whitelist(env: &Env, addr: &Address)` |
| `is_whitelisted` | `fn is_whitelisted(env: &Env, addr: &Address) -> bool` |
| `assert_whitelisted` | `fn assert_whitelisted(env: &Env, addr: &Address)` |
| `extend_instance_ttl` | `fn extend_instance_ttl(env: &Env, threshold: u32, extend_to: u32)` |
| `bump_instance_ttl` | `fn bump_instance_ttl(env: &Env)` |

#### Public types

| Type | Encoding source |
|---|---|
| — | No public types detected. |

#### Event topics

| Topic | Source |
|---|---|
| — | No inline `symbol_short!` event topics detected. |

### `donation-escrow`

#### Functions

| Name | ABI signature |
|---|---|
| `initialize` | `fn initialize( env: Env, admin: Address, xlm_token: Address, usdc_token: Address, eurc_token: Address, )` |
| `donate` | `fn donate(env: Env, donor: Address, token: Address, amount: i128, tree_count: u32) -> u64` |
| `advance_batch` | `fn advance_batch(env: Env) -> u32` |
| `release_batch` | `fn release_batch(env: Env, seqs: Vec<u64>, destination: Address)` |
| `refund` | `fn refund(env: Env, seq: u64)` |
| `get_donation` | `fn get_donation(env: Env, seq: u64) -> Option<DonationRecord>` |
| `current_batch` | `fn current_batch(env: Env) -> u32` |
| `setup_recurring` | `fn setup_recurring( env: Env, donor: Address, token: Address, project_id: u64, amount_per_interval: i128, interval_seconds: u64, ) -> u64` |
| `process_recurring` | `fn process_recurring(env: Env, donation_id: u64)` |
| `cancel_recurring` | `fn cancel_recurring(env: Env, donor: Address, donation_id: u64)` |
| `get_recurring` | `fn get_recurring(env: Env, donation_id: u64) -> Option<RecurringDonation>` |
| `register_project` | `fn register_project(env: Env, project_id: u64, project: Address)` |
| `add_accepted_token` | `fn add_accepted_token(env: Env, token_address: Address)` |
| `remove_accepted_token` | `fn remove_accepted_token(env: Env, token_address: Address)` |
| `add_to_whitelist` | `fn add_to_whitelist(env: Env, addr: Address)` |
| `is_whitelisted` | `fn is_whitelisted(env: Env, addr: Address) -> bool` |
| `assert_whitelisted` | `fn assert_whitelisted(env: Env, addr: Address)` |
| `get_accepted_tokens` | `fn get_accepted_tokens(env: Env) -> Vec<AcceptedToken>` |
| `is_accepted_token` | `fn is_accepted_token(env: Env, addr: Address) -> bool` |
| `create_campaign` | `fn create_campaign( env: Env, organiser: Address, token: Address, target_amount: i128, deadline_secs: u64, ) -> u64` |
| `donate_to_campaign` | `fn donate_to_campaign( env: Env, donor: Address, campaign_id: u64, token: Address, amount: i128, ) -> u64` |
| `claim_campaign` | `fn claim_campaign(env: Env, campaign_id: u64, destination: Address)` |
| `refund_campaign_donor` | `fn refund_campaign_donor(env: Env, campaign_id: u64, donation_seq: u64)` |
| `auto_refund_expired` | `fn auto_refund_expired(env: Env, campaign_id: u64, donation_seqs: Vec<u64>)` |
| `get_campaign` | `fn get_campaign(env: Env, campaign_id: u64) -> Option<Campaign>` |
| `get_campaign_donation` | `fn get_campaign_donation( env: Env, campaign_id: u64, donation_seq: u64, ) -> Option<CampaignDonation>` |
| `is_accepted_token` | `fn is_accepted_token(env: Env, addr: Address) -> bool` |

#### Public types

| Type | Encoding source |
|---|---|
| `AcceptedToken` | `#[contracttype]` or public Rust type in `contracts/donation-escrow/src/lib.rs` |
| `AccrualState` | `#[contracttype]` or public Rust type in `contracts/donation-escrow/src/lib.rs` |
| `Campaign` | `#[contracttype]` or public Rust type in `contracts/donation-escrow/src/lib.rs` |
| `CampaignDonation` | `#[contracttype]` or public Rust type in `contracts/donation-escrow/src/lib.rs` |
| `CampaignStatus` | `#[contracttype]` or public Rust type in `contracts/donation-escrow/src/lib.rs` |
| `DonationEscrow` | `#[contracttype]` or public Rust type in `contracts/donation-escrow/src/lib.rs` |
| `DonationEscrowError` | `#[contracttype]` or public Rust type in `contracts/donation-escrow/src/lib.rs` |
| `DonationRecord` | `#[contracttype]` or public Rust type in `contracts/donation-escrow/src/lib.rs` |
| `DonationStatus` | `#[contracttype]` or public Rust type in `contracts/donation-escrow/src/lib.rs` |
| `InterestConfig` | `#[contracttype]` or public Rust type in `contracts/donation-escrow/src/lib.rs` |
| `RecurringDonation` | `#[contracttype]` or public Rust type in `contracts/donation-escrow/src/lib.rs` |

#### Event topics

| Topic | Source |
|---|---|
| — | No inline `symbol_short!` event topics detected. |

### `escrow`

#### Functions

| Name | ABI signature |
|---|---|
| `initialize` | `fn initialize(env: Env, admin: Address, verifier: Address, admin_controls: Address)` |
| `initialize_registries` | `fn initialize_registries(env: Env, tree_registry: Address, planter_registry: Address)` |
| `set_fee_bps` | `fn set_fee_bps(env: Env, bps: u32)` |
| `set_treasury` | `fn set_treasury(env: Env, treasury: Address)` |
| `get_fee_bps` | `fn get_fee_bps(env: Env) -> u32` |
| `get_treasury` | `fn get_treasury(env: Env) -> Address` |
| `deposit` | `fn deposit( env: Env, sponsor: Address, planter: Address, tree_id: u64, token: Address, amount: i128, )` |
| `deposit_with_insurance` | `fn deposit_with_insurance( env: Env, sponsor: Address, planter: Address, tree_id: u64, token: Address, amount: i128, )` |
| `purchase_insurance` | `fn purchase_insurance(env: Env, sponsor: Address, tree_id: u64)` |
| `report_tree_dead` | `fn report_tree_dead(env: Env, tree_id: u64)` |
| `claim_insurance_refund` | `fn claim_insurance_refund(env: Env, tree_id: u64)` |
| `get_insurance_info` | `fn get_insurance_info(env: Env, tree_id: u64) -> (bool, i128, u64, bool)` |
| `donate_anonymous` | `fn donate_anonymous( env: Env, donor: Address, /// Deposit an anonymous species donation and assign it to an available planter. pub fn donate_anonymous( env: Env, sponsor: Address, amount: i128, token: Address, species: Symbol, region: Symbol, ) -> (u64, Address)` |
| `release` | `fn release(env: Env, tree_id: u64)` |
| `batch_release` | `fn batch_release(env: Env, tree_ids: Vec<u64>)` |
| `refund` | `fn refund(env: Env, tree_id: u64)` |
| `get_escrow` | `fn get_escrow(env: Env, tree_id: u64) -> Option<EscrowRecord>` |
| `get_species_cost` | `fn get_species_cost(env: Env, species: Symbol) -> i128` |
| `set_dead` | `fn set_dead(env: Env, dead: bool)` |
| `is_tree_dead` | `fn is_tree_dead(env: Env, _id: u64) -> bool` |

#### Public types

| Type | Encoding source |
|---|---|
| `Escrow` | `#[contracttype]` or public Rust type in `contracts/escrow/src/lib.rs` |
| `EscrowError` | `#[contracttype]` or public Rust type in `contracts/escrow/src/lib.rs` |
| `EscrowRecord` | `#[contracttype]` or public Rust type in `contracts/escrow/src/lib.rs` |
| `EscrowStatus` | `#[contracttype]` or public Rust type in `contracts/escrow/src/lib.rs` |
| `MockTreeRegistry` | `#[contracttype]` or public Rust type in `contracts/escrow/src/lib.rs` |

#### Event topics

| Topic | Source |
|---|---|
| — | No inline `symbol_short!` event topics detected. |

### `escrow-milestone`

#### Functions

| Name | ABI signature |
|---|---|
| `is_some` | `fn is_some(&self) -> bool` |
| `initialize` | `fn initialize( env: Env, admin: Address, amm: Address, xlm: Address, usdc: Address, usdt: Address, eurc: Address, )` |
| `deposit` | `fn deposit( env: Env, funder: Address, farmer: Address, token: Address, amount: i128, arbiter: Address, )` |
| `sponsor_as_gift` | `fn sponsor_as_gift( env: Env, funder: Address, recipient_wallet: Address, farmer: Address, token: Address, amount: i128, arbiter: Address, )` |
| `release_partial` | `fn release_partial( env: Env, approver: Address, milestone_id: Address, completion_pct: u32, )` |
| `verify_milestone` | `fn verify_milestone(env: Env, farmer: Address, verification_hash: BytesN<32>)` |
| `verify_survival` | `fn verify_survival( env: Env, farmer: Address, survival_verification_hash: BytesN<32>, survival_rate_percent: u32, )` |
| `raise_dispute` | `fn raise_dispute(env: Env, farmer: Address, caller: Address)` |
| `resolve_dispute` | `fn resolve_dispute(env: Env, farmer: Address, arbiter: Address, release_to_seller: bool)` |
| `refund` | `fn refund(env: Env, farmer: Address)` |
| `get_escrow` | `fn get_escrow(env: Env, farmer: Address) -> Option<EscrowState>` |
| `register_verifier` | `fn register_verifier(env: Env, verifier: Address)` |
| `remove_verifier` | `fn remove_verifier(env: Env, verifier: Address)` |
| `get_verifiers` | `fn get_verifiers(env: Env) -> soroban_sdk::Vec<Address>` |
| `approve_milestone` | `fn approve_milestone(env: Env, verifier: Address, farmer: Address, milestone_id: u32)` |
| `get_vote_count` | `fn get_vote_count(env: Env, farmer: Address, milestone_id: u32) -> u32` |
| `add_to_whitelist` | `fn add_to_whitelist(env: Env, addr: Address)` |
| `remove_from_whitelist` | `fn remove_from_whitelist(env: Env, addr: Address)` |
| `is_whitelisted` | `fn is_whitelisted(env: Env, addr: Address) -> bool` |
| `assert_whitelisted` | `fn assert_whitelisted(env: Env, addr: Address)` |
| `deposit` | `fn deposit(env: Env, from: Address, token: Address, amount: i128) -> i128` |
| `withdraw` | `fn withdraw(env: Env, from: Address, token: Address, shares: i128) -> i128` |
| `swap` | `fn swap(env: Env, from: Address, token_in: Address, _token_out: Address, amount_in: i128) -> i128` |

#### Public types

| Type | Encoding source |
|---|---|
| `EscrowMilestone` | `#[contracttype]` or public Rust type in `contracts/escrow-milestone/src/lib.rs` |
| `EscrowMilestoneError` | `#[contracttype]` or public Rust type in `contracts/escrow-milestone/src/lib.rs` |
| `EscrowState` | `#[contracttype]` or public Rust type in `contracts/escrow-milestone/src/lib.rs` |
| `EscrowStatus` | `#[contracttype]` or public Rust type in `contracts/escrow-milestone/src/lib.rs` |
| `MilestoneError` | `#[contracttype]` or public Rust type in `contracts/escrow-milestone/src/lib.rs` |
| `MockAmm` | `#[contracttype]` or public Rust type in `contracts/escrow-milestone/src/lib.rs` |
| `OptProof` | `#[contracttype]` or public Rust type in `contracts/escrow-milestone/src/lib.rs` |

#### Event topics

| Topic | Source |
|---|---|
| `VerfAdded` | `env.events().publish` in `contracts/escrow-milestone/src/lib.rs` |
| `m1release` | `env.events().publish` in `contracts/escrow-milestone/src/lib.rs` |
| `m2release` | `env.events().publish` in `contracts/escrow-milestone/src/lib.rs` |

### `farmer-registry`

#### Functions

| Name | ABI signature |
|---|---|
| `initialize` | `fn initialize(env: Env, admin: Address, admin_controls: Address)` |
| `register_validator` | `fn register_validator(env: Env, admin: Address, validator: Address)` |
| `revoke_validator` | `fn revoke_validator(env: Env, admin: Address, validator: Address)` |
| `is_validator` | `fn is_validator(env: Env, validator: Address) -> bool` |
| `freeze_farmer` | `fn freeze_farmer(env: Env, admin: Address, wallet_address: Address)` |
| `unfreeze_farmer` | `fn unfreeze_farmer(env: Env, admin: Address, wallet_address: Address)` |
| `is_frozen` | `fn is_frozen(env: Env, wallet_address: Address) -> bool` |
| `register_farmer` | `fn register_farmer( env: Env, validator: Address, wallet_address: Address, land_doc_hash: BytesN<32>, doc_preimage: Bytes, region_geohash: String, plot_merkle_root: BytesN<32>, ) -> FarmerProfile` |
| `update_profile` | `fn update_profile( env: Env, validator: Address, wallet_address: Address, new_land_doc_hash: BytesN<32>, new_doc_preimage: Bytes, new_region_geohash: String, new_plot_merkle_root: BytesN<32>, ) -> FarmerProfile` |
| `register_coop` | `fn register_coop( env: Env, validator: Address, multisig_account: Address, signers: Vec<Address>, threshold: u32, ) -> CoopProfile` |
| `get_coop` | `fn get_coop(env: Env, multisig_account: Address) -> Option<CoopProfile>` |
| `is_coop` | `fn is_coop(env: Env, multisig_account: Address) -> bool` |
| `get_farmer` | `fn get_farmer(env: Env, wallet_address: Address) -> Option<PublicFarmerView>` |
| `get_farmer_verified` | `fn get_farmer_verified( env: Env, validator: Address, wallet_address: Address, ) -> FarmerProfile` |
| `get_profile_history` | `fn get_profile_history( env: Env, validator: Address, wallet_address: Address, version: u32, ) -> Option<ProfileHistoryEntry>` |
| `get_version` | `fn get_version(env: Env, wallet_address: Address) -> u32` |
| `is_registered` | `fn is_registered(env: Env, wallet_address: Address) -> bool` |
| `verify_plot_boundary` | `fn verify_plot_boundary( env: Env, wallet_address: Address, leaf: BytesN<32>, proof: Vec<BytesN<32>>, ) -> bool` |
| `set_available` | `fn set_available(env: Env, wallet_address: Address, available: bool)` |
| `is_available` | `fn is_available(env: Env, wallet_address: Address) -> bool` |
| `register_plot` | `fn register_plot( env: Env, farmer: Address, plot_id: BytesN<32>, coordinates: Vec<(i64, i64)>, area_sqm: u64, )` |
| `get_plots_by_farmer` | `fn get_plots_by_farmer(env: Env, farmer_id: Address) -> Vec<FarmPlot>` |
| `verify_land_tenure` | `fn verify_land_tenure( env: Env, validator: Address, farmer: Address, title_id: BytesN<32>, land_title_hash: BytesN<32>, validator_signature: Bytes, ) -> LandTenureVerification` |
| `get_land_tenure` | `fn get_land_tenure(env: Env, title_id: BytesN<32>) -> Option<LandTenureVerification>` |
| `get_farmer_land_tenures` | `fn get_farmer_land_tenures( env: Env, farmer: Address, ) -> Vec<LandTenureVerification>` |
| `upsert_reputation` | `fn upsert_reputation( env: Env, admin: Address, planter: Address, score: u32, completed_jobs: u64, )` |
| `remove_reputation` | `fn remove_reputation(env: Env, admin: Address, planter: Address)` |
| `get_reputation` | `fn get_reputation(env: Env, planter: Address) -> Option<ReputationEntry>` |
| `get_all_reputations` | `fn get_all_reputations(env: Env) -> Vec<ReputationEntry>` |
| `get_reputation_count` | `fn get_reputation_count(env: Env) -> u32` |

#### Public types

| Type | Encoding source |
|---|---|
| `CoopProfile` | `#[contracttype]` or public Rust type in `contracts/farmer-registry/src/lib.rs` |
| `DataKey` | `#[contracttype]` or public Rust type in `contracts/farmer-registry/src/lib.rs` |
| `Error` | `#[contracttype]` or public Rust type in `contracts/farmer-registry/src/lib.rs` |
| `FarmPlot` | `#[contracttype]` or public Rust type in `contracts/farmer-registry/src/lib.rs` |
| `FarmerProfile` | `#[contracttype]` or public Rust type in `contracts/farmer-registry/src/lib.rs` |
| `FarmerRegistry` | `#[contracttype]` or public Rust type in `contracts/farmer-registry/src/lib.rs` |
| `LandTenureVerification` | `#[contracttype]` or public Rust type in `contracts/farmer-registry/src/lib.rs` |
| `ProfileHistoryEntry` | `#[contracttype]` or public Rust type in `contracts/farmer-registry/src/lib.rs` |
| `PublicFarmerView` | `#[contracttype]` or public Rust type in `contracts/farmer-registry/src/lib.rs` |

#### Event topics

| Topic | Source |
|---|---|
| — | No inline `symbol_short!` event topics detected. |

### `farmer-wallet`

#### Functions

| Name | ABI signature |
|---|---|
| `hop_count` | `fn hop_count(&self) -> u32` |
| `initialize` | `fn initialize(env: Env, admin: Address, treasury: Address)` |
| `set_chain_config` | `fn set_chain_config(env: Env, config: ChainConfig)` |
| `get_chain_config` | `fn get_chain_config(env: Env, chain: Chain) -> Option<ChainConfig>` |
| `is_chain_enabled` | `fn is_chain_enabled(env: Env, chain: Chain) -> bool` |
| `add_relayer` | `fn add_relayer(env: Env, relayer: Address)` |
| `remove_relayer` | `fn remove_relayer(env: Env, relayer: Address)` |
| `get_relayers` | `fn get_relayers(env: Env) -> Vec<Address>` |
| `is_relayer` | `fn is_relayer(env: Env, addr: Address) -> bool` |
| `set_treasury` | `fn set_treasury(env: Env, treasury: Address)` |
| `get_treasury` | `fn get_treasury(env: Env) -> Address` |
| `pause` | `fn pause(env: Env)` |
| `unpause` | `fn unpause(env: Env)` |
| `is_paused` | `fn is_paused(env: Env) -> bool` |
| `register_wallet` | `fn register_wallet(env: Env, farmer: Address) -> u64` |
| `set_wallet_active` | `fn set_wallet_active(env: Env, farmer: Address, active: bool)` |
| `get_wallet` | `fn get_wallet(env: Env, farmer: Address) -> Option<WalletProfile>` |
| `set_chain_address` | `fn set_chain_address(env: Env, farmer: Address, chain: Chain, address: String)` |
| `get_chain_address` | `fn get_chain_address(env: Env, farmer: Address, chain: Chain) -> Option<ChainAddress>` |
| `get_wallet_chains` | `fn get_wallet_chains(env: Env, farmer: Address) -> Vec<Chain>` |
| `get_best_route` | `fn get_best_route( env: Env, source_chain: Chain, dest_chain: Chain, amount: i128, ) -> Option<RouteQuote>` |
| `max_transfer` | `fn max_transfer(env: Env, source_chain: Chain, dest_chain: Chain) -> i128` |
| `open_transfer` | `fn open_transfer( env: Env, farmer: Address, dest_chain: Chain, amount: i128, min_amount_out: i128, deadline: u64, ) -> u64` |
| `relay_inbound` | `fn relay_inbound( env: Env, relayer: Address, farmer: Address, source_chain: Chain, source_tx: BytesN<32>, amount: i128, min_amount_out: i128, ) -> u64` |
| `settle_transfer` | `fn settle_transfer( env: Env, relayer: Address, transfer_id: u64, destination_tx: BytesN<32>, ) -> i128` |
| `refund_transfer` | `fn refund_transfer(env: Env, transfer_id: u64)` |
| `get_transfer` | `fn get_transfer(env: Env, transfer_id: u64) -> Option<CreditTransfer>` |
| `get_wallet_transfers` | `fn get_wallet_transfers( env: Env, farmer: Address, cursor: u32, limit: u32, ) -> Vec<CreditTransfer>` |
| `total_transfer_count` | `fn total_transfer_count(env: Env) -> u64` |

#### Public types

| Type | Encoding source |
|---|---|
| `Chain` | `#[contracttype]` or public Rust type in `contracts/farmer-wallet/src/lib.rs` |
| `ChainAddress` | `#[contracttype]` or public Rust type in `contracts/farmer-wallet/src/lib.rs` |
| `ChainConfig` | `#[contracttype]` or public Rust type in `contracts/farmer-wallet/src/lib.rs` |
| `CreditTransfer` | `#[contracttype]` or public Rust type in `contracts/farmer-wallet/src/lib.rs` |
| `Direction` | `#[contracttype]` or public Rust type in `contracts/farmer-wallet/src/lib.rs` |
| `FarmerWallet` | `#[contracttype]` or public Rust type in `contracts/farmer-wallet/src/lib.rs` |
| `RouteHop` | `#[contracttype]` or public Rust type in `contracts/farmer-wallet/src/lib.rs` |
| `RouteQuote` | `#[contracttype]` or public Rust type in `contracts/farmer-wallet/src/lib.rs` |
| `TransferStatus` | `#[contracttype]` or public Rust type in `contracts/farmer-wallet/src/lib.rs` |
| `WalletError` | `#[contracttype]` or public Rust type in `contracts/farmer-wallet/src/lib.rs` |
| `WalletProfile` | `#[contracttype]` or public Rust type in `contracts/farmer-wallet/src/lib.rs` |

#### Event topics

| Topic | Source |
|---|---|
| `chainCfg` | `env.events().publish` in `contracts/farmer-wallet/src/lib.rs` |
| `pause` | `env.events().publish` in `contracts/farmer-wallet/src/lib.rs` |
| `relAdd` | `env.events().publish` in `contracts/farmer-wallet/src/lib.rs` |
| `relRm` | `env.events().publish` in `contracts/farmer-wallet/src/lib.rs` |
| `treasury` | `env.events().publish` in `contracts/farmer-wallet/src/lib.rs` |
| `unpause` | `env.events().publish` in `contracts/farmer-wallet/src/lib.rs` |

### `harvesta-errors`

#### Functions

| Name | ABI signature |
|---|---|
| — | No public entry points detected. |

#### Public types

| Type | Encoding source |
|---|---|
| `FarmerError` | `#[contracttype]` or public Rust type in `contracts/harvesta-errors/src/lib.rs` |
| `GovernanceError` | `#[contracttype]` or public Rust type in `contracts/harvesta-errors/src/lib.rs` |
| `HarvestaError` | `#[contracttype]` or public Rust type in `contracts/harvesta-errors/src/lib.rs` |
| `NftError` | `#[contracttype]` or public Rust type in `contracts/harvesta-errors/src/lib.rs` |

#### Event topics

| Topic | Source |
|---|---|
| — | No inline `symbol_short!` event topics detected. |

### `kyc-attestation`

#### Functions

| Name | ABI signature |
|---|---|
| `initialize` | `fn initialize(env: Env, admin: Address)` |
| `register_verifier` | `fn register_verifier(env: Env, admin: Address, verifier: Address)` |
| `batch_revoke_kyc` | `fn batch_revoke_kyc(env: Env, admin: Address, farmers: Vec<Address>)` |
| `verify_kyc` | `fn verify_kyc(env: Env, verifier: Address, farmer: Address, proof: ZkProofInput)` |
| `attest_kyc` | `fn attest_kyc(env: Env, verifier: Address, farmer_id: Address, status: KycStatus)` |
| `get_zk_kyc_status` | `fn get_zk_kyc_status(env: Env, farmer: Address) -> KycStatus` |
| `get_zk_kyc_record` | `fn get_zk_kyc_record(env: Env, farmer: Address) -> Option<ZkKycRecord>` |
| `get_kyc_status` | `fn get_kyc_status(env: Env, farmer_id: Address) -> KycStatus` |
| `get_kyc_history` | `fn get_kyc_history(env: Env, farmer_id: Address) -> Vec<Attestation>` |
| `set_jurisdiction_flags` | `fn set_jurisdiction_flags(env: Env, verifier: Address, user: Address, flags: u32)` |
| `get_jurisdiction_flags` | `fn get_jurisdiction_flags(env: Env, user: Address) -> u32` |
| `has_jurisdiction_compliance` | `fn has_jurisdiction_compliance(env: Env, user: Address, required_flags: u32) -> bool` |
| `extend_instance_ttl` | `fn extend_instance_ttl(env: Env, threshold: u32, extend_to: u32)` |
| `bump_instance_ttl` | `fn bump_instance_ttl(env: Env)` |

#### Public types

| Type | Encoding source |
|---|---|
| `Attestation` | `#[contracttype]` or public Rust type in `contracts/kyc-attestation/src/lib.rs` |
| `Error` | `#[contracttype]` or public Rust type in `contracts/kyc-attestation/src/lib.rs` |
| `KycAttestation` | `#[contracttype]` or public Rust type in `contracts/kyc-attestation/src/lib.rs` |
| `KycStatus` | `#[contracttype]` or public Rust type in `contracts/kyc-attestation/src/lib.rs` |
| `ZkKycRecord` | `#[contracttype]` or public Rust type in `contracts/kyc-attestation/src/lib.rs` |
| `ZkProofInput` | `#[contracttype]` or public Rust type in `contracts/kyc-attestation/src/lib.rs` |

#### Event topics

| Topic | Source |
|---|---|
| — | No inline `symbol_short!` event topics detected. |

### `leaderboard`

#### Functions

| Name | ABI signature |
|---|---|
| `initialize` | `fn initialize(env: Env, admin: Address, month: Month) -> Result<(), Error>` |
| `record_sponsor` | `fn record_sponsor(env: Env, sponsor: Address, amount: i128) -> Result<(), Error>` |
| `record_planter` | `fn record_planter(env: Env, planter: Address, count: i128) -> Result<(), Error>` |
| `reset_month` | `fn reset_month(env: Env, admin: Address, new_month: Month) -> Result<(), Error>` |
| `add_bonus_pool` | `fn add_bonus_pool(env: Env, admin: Address, amount: i128) -> Result<(), Error>` |
| `distribute_sponsor_bonus` | `fn distribute_sponsor_bonus( env: Env, admin: Address, _first: Address, _second: Address, _third: Address, ) -> Result<(), Error>` |
| `distribute_planter_bonus` | `fn distribute_planter_bonus( env: Env, admin: Address, _first: Address, _second: Address, _third: Address, ) -> Result<(), Error>` |
| `get_top_sponsors` | `fn get_top_sponsors(env: Env) -> Result<Vec<LeaderboardEntry>, Error>` |
| `get_top_planters` | `fn get_top_planters(env: Env) -> Result<Vec<LeaderboardEntry>, Error>` |
| `get_sponsor_score` | `fn get_sponsor_score(env: Env, sponsor: Address) -> Result<i128, Error>` |
| `get_planter_score` | `fn get_planter_score(env: Env, planter: Address) -> Result<i128, Error>` |
| `get_current_month` | `fn get_current_month(env: Env) -> Result<Month, Error>` |
| `get_bonus_pool` | `fn get_bonus_pool(env: Env) -> Result<i128, Error>` |

#### Public types

| Type | Encoding source |
|---|---|
| `DataKey` | `#[contracttype]` or public Rust type in `contracts/leaderboard/src/lib.rs` |
| `Error` | `#[contracttype]` or public Rust type in `contracts/leaderboard/src/lib.rs` |
| `Leaderboard` | `#[contracttype]` or public Rust type in `contracts/leaderboard/src/lib.rs` |
| `LeaderboardEntry` | `#[contracttype]` or public Rust type in `contracts/leaderboard/src/lib.rs` |
| `Month` | `#[contracttype]` or public Rust type in `contracts/leaderboard/src/lib.rs` |

#### Event topics

| Topic | Source |
|---|---|
| — | No inline `symbol_short!` event topics detected. |

### `location-proof`

#### Functions

| Name | ABI signature |
|---|---|
| `initialize` | `fn initialize(env: Env, admin: Address)` |
| `register_zone` | `fn register_zone( env: Env, zone_id: u32, vertices: Vec<Vertex>, name: soroban_sdk::String, )` |
| `update_zone` | `fn update_zone( env: Env, zone_id: u32, vertices: Vec<Vertex>, name: soroban_sdk::String, )` |
| `get_zone` | `fn get_zone(env: Env, zone_id: u32) -> Option<AfforestationZone>` |
| `submit_proof_in_zone` | `fn submit_proof_in_zone( env: Env, farmer_id: Address, commitment: BytesN<32>, lat: i32, lon: i32, nonce: u64, zone_id: u32, )` |
| `submit_proof` | `fn submit_proof( env: Env, farmer_id: Address, commitment: BytesN<32>, in_region: bool, nonce: u64, )` |
| `get_proof` | `fn get_proof(env: Env, commitment: BytesN<32>) -> Option<LocationProofEntry>` |
| `is_proven` | `fn is_proven(env: Env, commitment: BytesN<32>) -> bool` |

#### Public types

| Type | Encoding source |
|---|---|
| `AfforestationZone` | `#[contracttype]` or public Rust type in `contracts/location-proof/src/lib.rs` |
| `LocationProof` | `#[contracttype]` or public Rust type in `contracts/location-proof/src/lib.rs` |
| `LocationProofEntry` | `#[contracttype]` or public Rust type in `contracts/location-proof/src/lib.rs` |
| `Vertex` | `#[contracttype]` or public Rust type in `contracts/location-proof/src/lib.rs` |

#### Event topics

| Topic | Source |
|---|---|
| — | No inline `symbol_short!` event topics detected. |

### `naira-payout`

#### Functions

| Name | ABI signature |
|---|---|
| `initialize` | `fn initialize( env: Env, admin: Address, anchor_withdrawal: Address, min_interval_secs: u64, max_daily_payout: i128, )` |
| `initiate_payout` | `fn initiate_payout( env: Env, funder: Address, farmer: Address, token: Address, usdc_amount: i128, expected_ngn_amount: i128, off_ramp_method: OffRampMethod, off_ramp_ref_hash: BytesN<32>, )` |
| `confirm_payout` | `fn confirm_payout(env: Env, farmer: Address, anchor_tx_id: BytesN<32>)` |
| `cancel_payout` | `fn cancel_payout(env: Env, farmer: Address)` |
| `get_payout` | `fn get_payout(env: Env, farmer: Address) -> Option<PayoutRecord>` |
| `update_limits` | `fn update_limits(env: Env, min_interval_secs: u64, max_daily_payout: i128)` |
| `reset_daily_payout` | `fn reset_daily_payout(env: Env)` |
| `reset_address_payout_time` | `fn reset_address_payout_time(env: Env, address: Address)` |
| `add_to_whitelist` | `fn add_to_whitelist(env: Env, addr: Address)` |
| `remove_from_whitelist` | `fn remove_from_whitelist(env: Env, addr: Address)` |
| `is_whitelisted` | `fn is_whitelisted(env: Env, addr: Address) -> bool` |
| `assert_whitelisted` | `fn assert_whitelisted(env: Env, addr: Address)` |

#### Public types

| Type | Encoding source |
|---|---|
| `NairaPayout` | `#[contracttype]` or public Rust type in `contracts/naira-payout/src/lib.rs` |
| `NairaPayoutError` | `#[contracttype]` or public Rust type in `contracts/naira-payout/src/lib.rs` |
| `OffRampMethod` | `#[contracttype]` or public Rust type in `contracts/naira-payout/src/lib.rs` |
| `PayoutRecord` | `#[contracttype]` or public Rust type in `contracts/naira-payout/src/lib.rs` |
| `PayoutStatus` | `#[contracttype]` or public Rust type in `contracts/naira-payout/src/lib.rs` |

#### Event topics

| Topic | Source |
|---|---|
| — | No inline `symbol_short!` event topics detected. |

### `nft-certificate`

#### Functions

| Name | ABI signature |
|---|---|
| `initialize` | `fn initialize(env: Env, admin: Address)` |
| `add_issuer` | `fn add_issuer(env: Env, issuer: Address)` |
| `remove_issuer` | `fn remove_issuer(env: Env, issuer: Address)` |
| `get_issuers` | `fn get_issuers(env: Env) -> Vec<IssuerRecord>` |
| `is_issuer` | `fn is_issuer(env: Env, addr: Address) -> bool` |
| `mint` | `fn mint(env: Env, to: Address, token_id: u64, metadata: CertificateMetadata)` |
| `mint` | `fn mint( env: Env, issuer: Address, to: Address, token_id: u64, metadata: CertificateMetadata, )` |
| `batch_mint` | `fn batch_mint( env: Env, issuer: Address, to: Address, token_ids: Vec<u64>, metadatas: Vec<CertificateMetadata>, )` |
| `merge` | `fn merge( env: Env, owner: Address, token_ids: Vec<u64>, new_token_id: u64, merged_metadata: CertificateMetadata, )` |
| `split` | `fn split( env: Env, owner: Address, original_token_id: u64, new_token_id_1: u64, new_token_id_2: u64, metadata_1: CertificateMetadata, metadata_2: CertificateMetadata, )` |
| `trade` | `fn trade( env: Env, seller: Address, buyer: Address, token_id: u64, payment_token: Address, price: i128, )` |
| `get_token` | `fn get_token(env: Env, token_id: u64) -> Option<Token>` |
| `owner_of` | `fn owner_of(env: Env, token_id: u64) -> Option<Address>` |
| `original_planter_of` | `fn original_planter_of(env: Env, token_id: u64) -> Option<Address>` |
| `total_supply` | `fn total_supply(env: Env) -> u64` |
| `pause` | `fn pause(env: Env)` |
| `unpause` | `fn unpause(env: Env)` |
| `is_paused` | `fn is_paused(env: Env) -> bool` |
| `render_svg` | `fn render_svg(env: Env, token_id: u64) -> String` |
| `token_uri` | `fn token_uri(env: Env, token_id: u64) -> String` |

#### Public types

| Type | Encoding source |
|---|---|
| `IssuerRecord` | `#[contracttype]` or public Rust type in `contracts/nft-certificate/src/lib.rs` |
| `NftCertError` | `#[contracttype]` or public Rust type in `contracts/nft-certificate/src/lib.rs` |
| `NftCertificate` | `#[contracttype]` or public Rust type in `contracts/nft-certificate/src/lib.rs` |
| `Token` | `#[contracttype]` or public Rust type in `contracts/nft-certificate/src/lib.rs` |
| `TokenData` | `#[contracttype]` or public Rust type in `contracts/nft-certificate/src/lib.rs` |

#### Event topics

| Topic | Source |
|---|---|
| `issAdd` | `env.events().publish` in `contracts/nft-certificate/src/lib.rs` |
| `issRm` | `env.events().publish` in `contracts/nft-certificate/src/lib.rs` |
| `merged` | `env.events().publish` in `contracts/nft-certificate/src/lib.rs` |
| `minted` | `env.events().publish` in `contracts/nft-certificate/src/lib.rs` |
| `paused` | `env.events().publish` in `contracts/nft-certificate/src/lib.rs` |
| `unpaused` | `env.events().publish` in `contracts/nft-certificate/src/lib.rs` |

### `nullifier-registry`

#### Functions

| Name | ABI signature |
|---|---|
| `initialize` | `fn initialize(env: Env, admin: Address)` |
| `compute_commitment` | `fn compute_commitment(env: Env, input: TreeCommitmentInput) -> BytesN<32>` |
| `register` | `fn register(env: Env, input: TreeCommitmentInput, expires_at: Option<u64>) -> BytesN<32>` |
| `is_registered` | `fn is_registered(env: Env, commitment: BytesN<32>) -> bool` |
| `is_registered_batch` | `fn is_registered_batch(env: Env, commitments: Vec<BytesN<32>>) -> Vec<bool>` |
| `register_batch` | `fn register_batch(env: Env, entries: Vec<TreeCommitmentBatchEntry>) -> Vec<BytesN<32>>` |
| `is_expired` | `fn is_expired(env: Env, commitment: BytesN<32>) -> bool` |
| `cleanup_expired` | `fn cleanup_expired(env: Env, nullifiers: Vec<BytesN<32>>)` |
| `get_entry` | `fn get_entry(env: Env, commitment: BytesN<32>) -> Option<NullifierEntry>` |

#### Public types

| Type | Encoding source |
|---|---|
| `NullifierEntry` | `#[contracttype]` or public Rust type in `contracts/nullifier-registry/src/lib.rs` |
| `NullifierError` | `#[contracttype]` or public Rust type in `contracts/nullifier-registry/src/lib.rs` |
| `NullifierRegistry` | `#[contracttype]` or public Rust type in `contracts/nullifier-registry/src/lib.rs` |
| `TreeCommitmentBatchEntry` | `#[contracttype]` or public Rust type in `contracts/nullifier-registry/src/lib.rs` |
| `TreeCommitmentInput` | `#[contracttype]` or public Rust type in `contracts/nullifier-registry/src/lib.rs` |

#### Event topics

| Topic | Source |
|---|---|
| — | No inline `symbol_short!` event topics detected. |

### `planter-blacklist`

#### Functions

| Name | ABI signature |
|---|---|
| `initialize` | `fn initialize(env: Env, admin: Address)` |
| `blacklist` | `fn blacklist(env: Env, admin: Address, planter: Address, reason_hash: BytesN<32>)` |
| `unblacklist` | `fn unblacklist(env: Env, admin: Address, planter: Address)` |
| `is_blacklisted` | `fn is_blacklisted(env: Env, planter: Address) -> bool` |
| `get_entry` | `fn get_entry(env: Env, planter: Address) -> Option<BlacklistEntry>` |
| `get_admin` | `fn get_admin(env: Env) -> Address` |

#### Public types

| Type | Encoding source |
|---|---|
| `BlacklistEntry` | `#[contracttype]` or public Rust type in `contracts/planter-blacklist/src/lib.rs` |
| `PlanterBlacklist` | `#[contracttype]` or public Rust type in `contracts/planter-blacklist/src/lib.rs` |

#### Event topics

| Topic | Source |
|---|---|
| — | No inline `symbol_short!` event topics detected. |

### `planter-registry`

#### Functions

| Name | ABI signature |
|---|---|
| `initialize` | `fn initialize(env: Env, admin: Address, stake_token: Address, min_stake_amount: i128)` |
| `set_escrow` | `fn set_escrow(env: Env, escrow: Address)` |
| `set_region_bounds` | `fn set_region_bounds(env: Env, region: String, bounds: BoundingBox)` |
| `get_region_bounds` | `fn get_region_bounds(env: Env, region: String) -> Option<BoundingBox>` |
| `validate_gps_in_region` | `fn validate_gps_in_region(env: Env, planter: Address, lat: i32, lon: i32) -> bool` |
| `stake_to_apply` | `fn stake_to_apply(env: Env, planter: Address, amount: i128)` |
| `unstake` | `fn unstake(env: Env, planter: Address)` |
| `register_planter` | `fn register_planter( env: Env, wallet: Address, name_hash: BytesN<32>, region: String, ) -> PlanterRecord` |
| `get_planter` | `fn get_planter(env: Env, wallet: Address) -> Option<PlanterRecord>` |
| `increment_score` | `fn increment_score(env: Env, wallet: Address)` |
| `slash_score` | `fn slash_score(env: Env, wallet: Address)` |
| `meets_min_score` | `fn meets_min_score(env: Env, wallet: Address, min_score: u32) -> bool` |
| `get_avail` | `fn get_avail(env: Env, region: String) -> Vec<Address>` |
| `inc_work` | `fn inc_work(env: Env, wallet: Address)` |
| `dec_work` | `fn dec_work(env: Env, wallet: Address)` |
| `set_active` | `fn set_active(env: Env, wallet: Address, active: bool)` |
| `set_capacity` | `fn set_capacity(env: Env, wallet: Address, capacity: u32)` |
| `get_planters_by_region` | `fn get_planters_by_region(env: Env, region: String) -> Vec<Address>` |

#### Public types

| Type | Encoding source |
|---|---|
| `BoundingBox` | `#[contracttype]` or public Rust type in `contracts/planter-registry/src/lib.rs` |
| `Error` | `#[contracttype]` or public Rust type in `contracts/planter-registry/src/lib.rs` |
| `PlanterRecord` | `#[contracttype]` or public Rust type in `contracts/planter-registry/src/lib.rs` |
| `PlanterRegistry` | `#[contracttype]` or public Rust type in `contracts/planter-registry/src/lib.rs` |
| `PlanterStake` | `#[contracttype]` or public Rust type in `contracts/planter-registry/src/lib.rs` |
| `ReputationTier` | `#[contracttype]` or public Rust type in `contracts/planter-registry/src/lib.rs` |

#### Event topics

| Topic | Source |
|---|---|
| `ActiveSet` | `env.events().publish` in `contracts/planter-registry/src/lib.rs` |
| `ScoreInc` | `env.events().publish` in `contracts/planter-registry/src/lib.rs` |
| `ScoreSls` | `env.events().publish` in `contracts/planter-registry/src/lib.rs` |
| `WorkDec` | `env.events().publish` in `contracts/planter-registry/src/lib.rs` |
| `WorkInc` | `env.events().publish` in `contracts/planter-registry/src/lib.rs` |
| `staked` | `env.events().publish` in `contracts/planter-registry/src/lib.rs` |
| `unstaked` | `env.events().publish` in `contracts/planter-registry/src/lib.rs` |

### `planting-bond`

#### Functions

| Name | ABI signature |
|---|---|
| `initialize` | `fn initialize( env: Env, admin: Address, treasury: Address, token: Address, tier_amounts: (i128, i128, i128), )` |
| `accept_job` | `fn accept_job(env: Env, planter: Address, tree_id: u64, tier: u32) -> Bond` |
| `return_bond` | `fn return_bond(env: Env, tree_id: u64)` |
| `slash_bond` | `fn slash_bond(env: Env, tree_id: u64)` |
| `get_bond` | `fn get_bond(env: Env, tree_id: u64) -> Option<Bond>` |
| `tier_amount` | `fn tier_amount(env: &Env, tier: u32) -> i128` |

#### Public types

| Type | Encoding source |
|---|---|
| `Bond` | `#[contracttype]` or public Rust type in `contracts/planting-bond/src/lib.rs` |
| `BondStatus` | `#[contracttype]` or public Rust type in `contracts/planting-bond/src/lib.rs` |
| `Error` | `#[contracttype]` or public Rust type in `contracts/planting-bond/src/lib.rs` |
| `PlantingBond` | `#[contracttype]` or public Rust type in `contracts/planting-bond/src/lib.rs` |

#### Event topics

| Topic | Source |
|---|---|
| — | No inline `symbol_short!` event topics detected. |

### `platform-governance`

#### Functions

| Name | ABI signature |
|---|---|
| `initialize` | `fn initialize( env: Env, admin: Address, staking_contract: Address, admin_controls: Address, platform_fee: u64, min_planting_bond: i128, tree_token: Address, )` |
| `create_proposal` | `fn create_proposal( env: Env, description_hash: String, proposal_type: ProposalType, options: Vec<VoteOption>, voting_period: u64, proposer: Address, )` |
| `vote` | `fn vote(env: Env, proposal_id: u64, option_id: u32, voter: Address)` |
| `queue` | `fn queue(env: Env, proposal_id: u64)` |
| `execute` | `fn execute(env: Env, proposal_id: u64)` |
| `cancel_queued_proposal` | `fn cancel_queued_proposal(env: Env, proposal_id: u64)` |
| `execute` | `fn execute(env: Env, proposal_id: u64)` |
| `lock_tokens` | `fn lock_tokens(env: Env, voter: Address, amount: i128)` |
| `unlock_tokens` | `fn unlock_tokens(env: Env, voter: Address, amount: i128)` |
| `locked_balance` | `fn locked_balance(env: Env, voter: Address) -> Option<TokenLock>` |
| `set_min_lock_seconds` | `fn set_min_lock_seconds(env: Env, seconds: u64)` |
| `register_delegate` | `fn register_delegate(env: Env, delegate: Address, domain: String)` |
| `unregister_delegate` | `fn unregister_delegate(env: Env, delegate: Address)` |
| `delegate_to` | `fn delegate_to(env: Env, delegator: Address, delegate: Address)` |
| `delegate_voting_power` | `fn delegate_voting_power(env: Env, voter: Address, proxy: Address)` |
| `retract_delegation` | `fn retract_delegation(env: Env, delegator: Address)` |
| `get_proposal` | `fn get_proposal(env: Env, proposal_id: u64) -> ProposalRecord` |
| `get_vote` | `fn get_vote(env: Env, proposal_id: u64, voter: Address) -> Option<VoteRecord>` |
| `proposal_count` | `fn proposal_count(env: Env) -> u64` |
| `platform_fee` | `fn platform_fee(env: Env) -> u64` |
| `min_planting_bond` | `fn min_planting_bond(env: Env) -> i128` |
| `verifier_whitelist` | `fn verifier_whitelist(env: Env) -> Vec<Address>` |
| `quorum_percentage` | `fn quorum_percentage(env: Env) -> u64` |
| `timelock_seconds` | `fn timelock_seconds(env: Env) -> u64` |
| `is_paused` | `fn is_paused(env: Env) -> bool` |
| `get_delegation` | `fn get_delegation(env: Env, delegator: Address) -> Option<Address>` |
| `get_delegated_power` | `fn get_delegated_power(env: Env, delegate: Address) -> i128` |
| `get_queued_proposal` | `fn get_queued_proposal(env: Env, proposal_id: u64) -> Option<QueuedProposal>` |
| `get_pending_queue` | `fn get_pending_queue(env: Env) -> Vec<ProposalRecord>` |
| `get_queued_proposals` | `fn get_queued_proposals(env: Env) -> Vec<ProposalRecord>` |
| `get_executable_proposals` | `fn get_executable_proposals(env: Env) -> Vec<ProposalRecord>` |
| `get_proposal_timelock_status` | `fn get_proposal_timelock_status(env: Env, proposal_id: u64) -> (bool, u64, u64)` |
| `update_quorum_percentage` | `fn update_quorum_percentage(env: Env, new_percentage: u64)` |
| `update_timelock` | `fn update_timelock(env: Env, new_timelock: u64)` |
| `set_paused` | `fn set_paused(env: Env, paused: bool)` |
| `set_platform_fee` | `fn set_platform_fee(env: Env, new_fee: u64)` |
| `set_veto_council` | `fn set_veto_council(env: Env, council: Address)` |
| `veto_council` | `fn veto_council(env: Env) -> Option<Address>` |
| `cancel_proposal` | `fn cancel_proposal(env: Env, council: Address, proposal_id: u64)` |
| `set_min_planting_bond` | `fn set_min_planting_bond(env: Env, new_bond: i128)` |
| `add_verifier_to_whitelist` | `fn add_verifier_to_whitelist(env: Env, verifier: Address)` |
| `remove_verifier_from_whitelist` | `fn remove_verifier_from_whitelist(env: Env, verifier: Address)` |
| `create_vesting_schedule` | `fn create_vesting_schedule( env: Env, planter: Address, token: Address, total_amount: i128, start_at: u64, cliff_seconds: u64, vesting_seconds: u64, )` |
| `claim_vested_tokens` | `fn claim_vested_tokens(env: Env, planter: Address) -> i128` |
| `revoke_vesting_schedule` | `fn revoke_vesting_schedule(env: Env, planter: Address)` |
| `get_vesting_schedule` | `fn get_vesting_schedule(env: Env, planter: Address) -> Option<VestingSchedule>` |
| `get_vested_amount` | `fn get_vested_amount(env: Env, planter: Address) -> i128` |
| `get_claimable_amount` | `fn get_claimable_amount(env: Env, planter: Address) -> i128` |
| `adjust_quorum` | `fn adjust_quorum(env: Env, admin: Address)` |
| `participation_30d` | `fn participation_30d(env: Env) -> i128` |
| `participation_rate_bps` | `fn participation_rate_bps(env: Env) -> u64` |
| `participation_window_days` | `fn participation_window_days(_env: Env) -> u32` |
| `isqrt` | `fn isqrt(n: i128) -> i128` |

#### Public types

| Type | Encoding source |
|---|---|
| `DelegateRecord` | `#[contracttype]` or public Rust type in `contracts/platform-governance/src/lib.rs` |
| `GovernanceError` | `#[contracttype]` or public Rust type in `contracts/platform-governance/src/lib.rs` |
| `PlatformGovernance` | `#[contracttype]` or public Rust type in `contracts/platform-governance/src/lib.rs` |
| `ProposalRecord` | `#[contracttype]` or public Rust type in `contracts/platform-governance/src/lib.rs` |
| `ProposalStatus` | `#[contracttype]` or public Rust type in `contracts/platform-governance/src/lib.rs` |
| `ProposalType` | `#[contracttype]` or public Rust type in `contracts/platform-governance/src/lib.rs` |
| `QueuedProposal` | `#[contracttype]` or public Rust type in `contracts/platform-governance/src/lib.rs` |
| `TokenLock` | `#[contracttype]` or public Rust type in `contracts/platform-governance/src/lib.rs` |
| `VestingSchedule` | `#[contracttype]` or public Rust type in `contracts/platform-governance/src/lib.rs` |
| `VoteOption` | `#[contracttype]` or public Rust type in `contracts/platform-governance/src/lib.rs` |
| `VoteRecord` | `#[contracttype]` or public Rust type in `contracts/platform-governance/src/lib.rs` |
| `VoteTally` | `#[contracttype]` or public Rust type in `contracts/platform-governance/src/lib.rs` |

#### Event topics

| Topic | Source |
|---|---|
| `bond_set` | `env.events().publish` in `contracts/platform-governance/src/lib.rs` |
| `fee_set` | `env.events().publish` in `contracts/platform-governance/src/lib.rs` |
| `wl_add` | `env.events().publish` in `contracts/platform-governance/src/lib.rs` |
| `wl_rm` | `env.events().publish` in `contracts/platform-governance/src/lib.rs` |

### `public-seal`

#### Functions

| Name | ABI signature |
|---|---|
| `initialize` | `fn initialize( env: Env, admin: Address, signers: soroban_sdk::Vec<Address>, threshold: u32, approval_window_secs: u64, )` |
| `replace_policy` | `fn replace_policy( env: Env, signers: soroban_sdk::Vec<Address>, threshold: u32, approval_window_secs: u64, )` |
| `revoke_signer` | `fn revoke_signer(env: Env, signer: Address)` |
| `unrevoke_signer` | `fn unrevoke_signer(env: Env, signer: Address)` |
| `propose` | `fn propose(env: Env, evidence_hash: BytesN<32>) -> u32` |
| `approve` | `fn approve(env: Env, request_id: u32)` |
| `cancel` | `fn cancel(env: Env, request_id: u32)` |
| `current_policy_version` | `fn current_policy_version(env: Env) -> u32` |
| `get_policy` | `fn get_policy(env: Env, version: u32) -> SealPolicy` |
| `get_request` | `fn get_request(env: Env, request_id: u32) -> SealRequestSnapshot` |
| `has_approved` | `fn has_approved(env: Env, request_id: u32, signer: Address) -> bool` |
| `is_signer_revoked` | `fn is_signer_revoked(env: Env, signer: Address) -> bool` |
| `open_request_count` | `fn open_request_count(env: Env) -> u32` |
| `get_admin` | `fn get_admin(env: Env) -> Address` |

#### Public types

| Type | Encoding source |
|---|---|
| `PolicyState` | `#[contracttype]` or public Rust type in `contracts/public-seal/src/lib.rs` |
| `PublicSeal` | `#[contracttype]` or public Rust type in `contracts/public-seal/src/lib.rs` |
| `RequestState` | `#[contracttype]` or public Rust type in `contracts/public-seal/src/lib.rs` |
| `SealPolicy` | `#[contracttype]` or public Rust type in `contracts/public-seal/src/lib.rs` |
| `SealRequest` | `#[contracttype]` or public Rust type in `contracts/public-seal/src/lib.rs` |
| `SealRequestSnapshot` | `#[contracttype]` or public Rust type in `contracts/public-seal/src/lib.rs` |

#### Event topics

| Topic | Source |
|---|---|
| — | No inline `symbol_short!` event topics detected. |

### `species-catalog`

#### Functions

| Name | ABI signature |
|---|---|
| `initialize` | `fn initialize(env: Env, admin: Address)` |
| `register_species` | `fn register_species(env: Env, args: RegisterSpeciesArgs) -> u32` |
| `update_species` | `fn update_species(env: Env, id: u32, args: UpdateSpeciesArgs)` |
| `remove_species` | `fn remove_species(env: Env, id: u32)` |
| `get_species` | `fn get_species(env: Env, id: u32) -> SpeciesCatalogEntry` |
| `get_species_by_slug` | `fn get_species_by_slug(env: Env, slug: Symbol) -> SpeciesCatalogEntry` |
| `search_by_common_name` | `fn search_by_common_name( env: Env, prefix: soroban_sdk::String, limit: u32, offset: u32, ) -> Vec<SpeciesCatalogEntry>` |
| `search_by_scientific_name` | `fn search_by_scientific_name( env: Env, prefix: soroban_sdk::String, limit: u32, offset: u32, ) -> Vec<SpeciesCatalogEntry>` |
| `search_by_genus` | `fn search_by_genus( env: Env, genus: soroban_sdk::String, limit: u32, offset: u32, ) -> Vec<SpeciesCatalogEntry>` |
| `search_by_family` | `fn search_by_family( env: Env, family: soroban_sdk::String, limit: u32, offset: u32, ) -> Vec<SpeciesCatalogEntry>` |
| `search_by_conservation_status` | `fn search_by_conservation_status( env: Env, status: ConservationStatus, limit: u32, offset: u32, ) -> Vec<SpeciesCatalogEntry>` |
| `search_by_leaf_type` | `fn search_by_leaf_type( env: Env, leaf_type: LeafType, limit: u32, offset: u32, ) -> Vec<SpeciesCatalogEntry>` |
| `search_by_region` | `fn search_by_region( env: Env, region: soroban_sdk::String, limit: u32, offset: u32, ) -> Vec<SpeciesCatalogEntry>` |
| `search_species` | `fn search_species(env: Env, filter: SearchFilter) -> SpeciesSearchResponse` |
| `list_all_species` | `fn list_all_species(env: Env, limit: u32, offset: u32) -> Vec<SpeciesCatalogEntry>` |
| `species_count` | `fn species_count(env: Env) -> u32` |

#### Public types

| Type | Encoding source |
|---|---|
| `ConservationStatus` | `#[contracttype]` or public Rust type in `contracts/species-catalog/src/lib.rs` |
| `LeafType` | `#[contracttype]` or public Rust type in `contracts/species-catalog/src/lib.rs` |
| `RegisterSpeciesArgs` | `#[contracttype]` or public Rust type in `contracts/species-catalog/src/lib.rs` |
| `SearchFilter` | `#[contracttype]` or public Rust type in `contracts/species-catalog/src/lib.rs` |
| `SpeciesCatalog` | `#[contracttype]` or public Rust type in `contracts/species-catalog/src/lib.rs` |
| `SpeciesCatalogEntry` | `#[contracttype]` or public Rust type in `contracts/species-catalog/src/lib.rs` |
| `SpeciesCatalogError` | `#[contracttype]` or public Rust type in `contracts/species-catalog/src/lib.rs` |
| `SpeciesSearchResponse` | `#[contracttype]` or public Rust type in `contracts/species-catalog/src/lib.rs` |
| `UpdateSpeciesArgs` | `#[contracttype]` or public Rust type in `contracts/species-catalog/src/lib.rs` |

#### Event topics

| Topic | Source |
|---|---|
| — | No inline `symbol_short!` event topics detected. |

### `species-registry`

#### Functions

| Name | ABI signature |
|---|---|
| `initialize` | `fn initialize(env: Env, admin: Address)` |
| `register_species` | `fn register_species(env: Env, slug: Symbol, co2_scaled: i128, maturity_years: u32)` |
| `get_species` | `fn get_species(env: Env, slug: Symbol) -> SpeciesRecord` |
| `get_co2_rate` | `fn get_co2_rate(env: Env, slug: Symbol) -> i128` |

#### Public types

| Type | Encoding source |
|---|---|
| `SpeciesRecord` | `#[contracttype]` or public Rust type in `contracts/species-registry/src/lib.rs` |
| `SpeciesRegistry` | `#[contracttype]` or public Rust type in `contracts/species-registry/src/lib.rs` |

#### Event topics

| Topic | Source |
|---|---|
| — | No inline `symbol_short!` event topics detected. |

### `species-voting`

#### Functions

| Name | ABI signature |
|---|---|
| `initialize` | `fn initialize( env: Env, admin: Address, tree_token: Address, species_registry: Address, voting_threshold: i128, voting_period: u64, )` |
| `propose_species` | `fn propose_species( env: Env, proposer: Address, slug: Symbol, name: String, co2_scaled: i128, maturity_years: u32, )` |
| `vote` | `fn vote(env: Env, voter: Address, proposal_id: u64, vote_for: bool)` |
| `execute_proposal` | `fn execute_proposal(env: Env, proposal_id: u64)` |
| `get_proposal` | `fn get_proposal(env: Env, proposal_id: u64) -> ProposalRecord` |
| `get_vote` | `fn get_vote(env: Env, proposal_id: u64, voter: Address) -> Option<VoteRecord>` |
| `proposal_count` | `fn proposal_count(env: Env) -> u64` |
| `voting_threshold` | `fn voting_threshold(env: Env) -> i128` |
| `voting_period` | `fn voting_period(env: Env) -> u64` |
| `update_voting_threshold` | `fn update_voting_threshold(env: Env, new_threshold: i128)` |
| `update_voting_period` | `fn update_voting_period(env: Env, new_period: u64)` |

#### Public types

| Type | Encoding source |
|---|---|
| `ProposalRecord` | `#[contracttype]` or public Rust type in `contracts/species-voting/src/lib.rs` |
| `ProposalStatus` | `#[contracttype]` or public Rust type in `contracts/species-voting/src/lib.rs` |
| `SpeciesVoting` | `#[contracttype]` or public Rust type in `contracts/species-voting/src/lib.rs` |
| `VoteRecord` | `#[contracttype]` or public Rust type in `contracts/species-voting/src/lib.rs` |

#### Event topics

| Topic | Source |
|---|---|
| — | No inline `symbol_short!` event topics detected. |

### `sponsor-receipt`

#### Functions

| Name | ABI signature |
|---|---|
| `is_none` | `fn is_none(&self) -> bool` |
| `initialize` | `fn initialize(env: Env, admin: Address)` |
| `mint_receipt` | `fn mint_receipt( env: Env, sponsor: Address, tree_id: u64, species: Symbol, region: String, co2_estimate_scaled: i128, planter: OptAddress, ) -> u64` |
| `get_receipt` | `fn get_receipt(env: Env, receipt_id: u64) -> Option<SponsorReceipt>` |
| `get_receipts_by_sponsor` | `fn get_receipts_by_sponsor(env: Env, sponsor: Address) -> Vec<u64>` |
| `owner_of` | `fn owner_of(env: Env, receipt_id: u64) -> Option<Address>` |
| `receipt_for_tree` | `fn receipt_for_tree(env: Env, sponsor: Address, tree_id: u64) -> u64` |
| `total_receipts` | `fn total_receipts(env: Env) -> u64` |
| `transfer` | `fn transfer(env: Env, from: Address, to: Address, receipt_id: u64)` |
| `trade` | `fn trade( env: Env, seller: Address, buyer: Address, receipt_id: u64, payment_token: Address, price: i128, )` |
| `attempt_transfer` | `fn attempt_transfer( env: Env, _from: Address, _to: Address, _receipt_id: u64, )` |
| `revoke_receipt` | `fn revoke_receipt(env: Env, receipt_id: u64)` |
| `pause` | `fn pause(env: Env)` |
| `unpause` | `fn unpause(env: Env)` |
| `is_paused` | `fn is_paused(env: Env) -> bool` |
| `get_admin` | `fn get_admin(env: Env) -> Address` |
| `propose_admin` | `fn propose_admin(env: Env, new_admin: Address)` |
| `accept_admin` | `fn accept_admin(env: Env)` |

#### Public types

| Type | Encoding source |
|---|---|
| `DataKey` | `#[contracttype]` or public Rust type in `contracts/sponsor-receipt/src/lib.rs` |
| `Error` | `#[contracttype]` or public Rust type in `contracts/sponsor-receipt/src/lib.rs` |
| `OptAddress` | `#[contracttype]` or public Rust type in `contracts/sponsor-receipt/src/lib.rs` |
| `SponsorReceipt` | `#[contracttype]` or public Rust type in `contracts/sponsor-receipt/src/lib.rs` |
| `SponsorReceiptContract` | `#[contracttype]` or public Rust type in `contracts/sponsor-receipt/src/lib.rs` |

#### Event topics

| Topic | Source |
|---|---|
| `AdminProp` | `env.events().publish` in `contracts/sponsor-receipt/src/lib.rs` |
| `AdminXfer` | `env.events().publish` in `contracts/sponsor-receipt/src/lib.rs` |

### `staking-rewards`

#### Functions

| Name | ABI signature |
|---|---|
| `initialize` | `fn initialize(env: Env, admin: Address, xlm_token: Address, default_apy_bps: u32)` |
| `stake` | `fn stake(env: Env, sponsor: Address, amount: i128, lock_months: u32) -> u64` |
| `calculate_rewards` | `fn calculate_rewards(env: Env, stake_id: u64) -> i128` |
| `claim` | `fn claim(env: Env, sponsor: Address, stake_id: u64) -> i128` |
| `unstake` | `fn unstake(env: Env, sponsor: Address, stake_id: u64) -> i128` |
| `fund_rewards` | `fn fund_rewards(env: Env, admin: Address, amount: i128)` |
| `update_apy` | `fn update_apy(env: Env, admin: Address, new_apy_bps: u32)` |
| `get_stake` | `fn get_stake(env: Env, stake_id: u64) -> Option<StakeRecord>` |
| `get_pending_rewards` | `fn get_pending_rewards(env: Env, stake_id: u64) -> i128` |

#### Public types

| Type | Encoding source |
|---|---|
| `StakeRecord` | `#[contracttype]` or public Rust type in `contracts/staking-rewards/src/lib.rs` |
| `StakeStatus` | `#[contracttype]` or public Rust type in `contracts/staking-rewards/src/lib.rs` |
| `StakingRewards` | `#[contracttype]` or public Rust type in `contracts/staking-rewards/src/lib.rs` |

#### Event topics

| Topic | Source |
|---|---|
| `apy` | `env.events().publish` in `contracts/staking-rewards/src/lib.rs` |
| `fund` | `env.events().publish` in `contracts/staking-rewards/src/lib.rs` |

### `subscription-sponsorship`

#### Functions

| Name | ABI signature |
|---|---|
| `initialize` | `fn initialize(env: Env, admin: Address, xlm_token: Address, usdc_token: Address)` |
| `setup` | `fn setup( env: Env, sponsor: Address, farmer: Address, token: Address, amount_per_cycle: i128, trees_per_cycle: u32, interval_seconds: u64, ) -> u64` |
| `process` | `fn process(env: Env, subscription_id: u64)` |
| `cancel` | `fn cancel(env: Env, sponsor: Address, subscription_id: u64)` |
| `get_subscription` | `fn get_subscription(env: Env, subscription_id: u64) -> Option<SubscriptionRecord>` |
| `get_sponsor_subscriptions` | `fn get_sponsor_subscriptions(env: Env, sponsor: Address) -> Vec<u64>` |
| `update_tokens` | `fn update_tokens(env: Env, xlm_token: Address, usdc_token: Address)` |

#### Public types

| Type | Encoding source |
|---|---|
| `SubscriptionRecord` | `#[contracttype]` or public Rust type in `contracts/subscription-sponsorship/src/lib.rs` |
| `SubscriptionSponsorship` | `#[contracttype]` or public Rust type in `contracts/subscription-sponsorship/src/lib.rs` |
| `SubscriptionStatus` | `#[contracttype]` or public Rust type in `contracts/subscription-sponsorship/src/lib.rs` |

#### Event topics

| Topic | Source |
|---|---|
| — | No inline `symbol_short!` event topics detected. |

### `transparent-proxy`

#### Functions

| Name | ABI signature |
|---|---|
| `initialize` | `fn initialize(env: Env, admin: Address, implementation: BytesN<32>)` |
| `upgrade` | `fn upgrade(env: Env, new_implementation: BytesN<32>)` |
| `get_config` | `fn get_config(env: Env) -> ProxyConfig` |
| `get_admin` | `fn get_admin(env: Env) -> Address` |

#### Public types

| Type | Encoding source |
|---|---|
| `ProxyConfig` | `#[contracttype]` or public Rust type in `contracts/transparent-proxy/src/lib.rs` |
| `ProxyError` | `#[contracttype]` or public Rust type in `contracts/transparent-proxy/src/lib.rs` |
| `TransparentProxy` | `#[contracttype]` or public Rust type in `contracts/transparent-proxy/src/lib.rs` |

#### Event topics

| Topic | Source |
|---|---|
| `Init` | `env.events().publish` in `contracts/transparent-proxy/src/lib.rs` |

### `treasury`

#### Functions

| Name | ABI signature |
|---|---|
| `initialize` | `fn initialize(env: Env, signers: Vec<Address>, token: Address)` |
| `deposit` | `fn deposit(env: Env, from: Address, amount: i128)` |
| `propose` | `fn propose(env: Env, signer: Address, to: Address, amount: i128) -> u32` |
| `approve` | `fn approve(env: Env, signer: Address, proposal_id: u32)` |
| `cancel` | `fn cancel(env: Env, signer: Address, proposal_id: u32)` |
| `get_proposal` | `fn get_proposal(env: Env, proposal_id: u32) -> WithdrawProposal` |
| `approval_count` | `fn approval_count(env: Env, proposal_id: u32) -> u32` |
| `is_emergency` | `fn is_emergency(env: Env, proposal_id: u32) -> bool` |
| `balance` | `fn balance(env: Env) -> i128` |
| `get_signers` | `fn get_signers(env: Env) -> Vec<Address>` |

#### Public types

| Type | Encoding source |
|---|---|
| `ProposalStatus` | `#[contracttype]` or public Rust type in `contracts/treasury/src/lib.rs` |
| `Treasury` | `#[contracttype]` or public Rust type in `contracts/treasury/src/lib.rs` |
| `WithdrawProposal` | `#[contracttype]` or public Rust type in `contracts/treasury/src/lib.rs` |

#### Event topics

| Topic | Source |
|---|---|
| `proposed` | `env.events().publish` in `contracts/treasury/src/lib.rs` |

### `tree-escrow`

#### Functions

| Name | ABI signature |
|---|---|
| `initialize` | `fn initialize(env: Env, admin: Address, tree_token: Address)` |
| `create_milestone_stream` | `fn create_milestone_stream( env: Env, funder: Address, farmer: Address, token: Address, total_amount: i128, total_milestones: u32, milestone_interval_secs: u64, ) -> u64` |
| `approve_milestone_greenlight` | `fn approve_milestone_greenlight(env: Env, verifier: Address, stream_id: u64)` |
| `release_stream_payment` | `fn release_stream_payment(env: Env, stream_id: u64) -> i128` |
| `get_milestone_stream` | `fn get_milestone_stream(env: Env, stream_id: u64) -> Option<MilestoneStream>` |
| `extend_instance_ttl` | `fn extend_instance_ttl(env: Env, threshold: u32, extend_to: u32)` |
| `bump_instance_ttl` | `fn bump_instance_ttl(env: Env)` |
| `deposit` | `fn deposit( env: Env, donor: Address, farmer: Address, token: Address, amount: i128, tree_count: i128, area_hectares: i128, )` |
| `sponsor_as_gift` | `fn sponsor_as_gift( env: Env, donor: Address, recipient_wallet: Address, farmer: Address, token: Address, amount: i128, tree_count: i128, area_hectares: i128, )` |
| `deposit_with_insurance` | `fn deposit_with_insurance( env: Env, donor: Address, farmer: Address, token: Address, amount: i128, tree_count: i128, )` |
| `configure_wholesale_approvers` | `fn configure_wholesale_approvers(env: Env, approvers: Vec<Address>, threshold: u32)` |
| `approve_wholesale_order` | `fn approve_wholesale_order(env: Env, approver: Address, order_id: u64)` |
| `cancel_wholesale_order` | `fn cancel_wholesale_order(env: Env, caller: Address, order_id: u64)` |
| `get_wholesale_order` | `fn get_wholesale_order(env: Env, order_id: u64) -> Option<WholesaleOrder>` |
| `get_wholesale_approvers` | `fn get_wholesale_approvers(env: Env) -> Option<(Vec<Address>, u32)>` |
| `verify_planting` | `fn verify_planting( env: Env, farmer: Address, proof_hash: BytesN<32>, verified_tree_count: i128, )` |
| `verify_survival` | `fn verify_survival( env: Env, farmer: Address, proof_hash: BytesN<32>, survival_rate_percent: u32, )` |
| `refund` | `fn refund(env: Env, farmer: Address)` |
| `report_dead_tree` | `fn report_dead_tree(env: Env, farmer: Address)` |
| `claim_insurance_refund` | `fn claim_insurance_refund(env: Env, farmer: Address)` |
| `get_insurance_info` | `fn get_insurance_info(env: Env, farmer: Address) -> (bool, i128, u64, bool)` |
| `get_record` | `fn get_record(env: Env, farmer: Address) -> Option<EscrowRecord>` |

#### Public types

| Type | Encoding source |
|---|---|
| `BatchSlot` | `#[contracttype]` or public Rust type in `contracts/tree-escrow/src/lib.rs` |
| `EscrowRecord` | `#[contracttype]` or public Rust type in `contracts/tree-escrow/src/lib.rs` |
| `EscrowStatus` | `#[contracttype]` or public Rust type in `contracts/tree-escrow/src/lib.rs` |
| `MilestoneStream` | `#[contracttype]` or public Rust type in `contracts/tree-escrow/src/lib.rs` |
| `OrderStatus` | `#[contracttype]` or public Rust type in `contracts/tree-escrow/src/lib.rs` |
| `TreeEscrow` | `#[contracttype]` or public Rust type in `contracts/tree-escrow/src/lib.rs` |
| `WholesaleOrder` | `#[contracttype]` or public Rust type in `contracts/tree-escrow/src/lib.rs` |

#### Event topics

| Topic | Source |
|---|---|
| — | No inline `symbol_short!` event topics detected. |

### `tree-genetics`

#### Functions

| Name | ABI signature |
|---|---|
| `initialize` | `fn initialize(env: Env, admin: Address)` |
| `register_genetics` | `fn register_genetics(env: Env, species: Symbol, markers: Vec<GeneticMarker>)` |
| `record_planting` | `fn record_planting( env: Env, region: Symbol, species: Symbol, tree_count: u32, ) -> u64` |
| `get_genetics` | `fn get_genetics(env: Env, species: Symbol) -> Option<SpeciesGenetics>` |
| `get_diversity_score` | `fn get_diversity_score(env: Env, species: Symbol) -> u32` |
| `calculate_biodiversity` | `fn calculate_biodiversity(env: Env, region: Symbol) -> u32` |
| `is_monoculture` | `fn is_monoculture(env: Env, region: Symbol) -> bool` |
| `list_region_species` | `fn list_region_species(env: Env, region: Symbol) -> Vec<Symbol>` |

#### Public types

| Type | Encoding source |
|---|---|
| `GeneticMarker` | `#[contracttype]` or public Rust type in `contracts/tree-genetics/src/lib.rs` |
| `PlantingRecord` | `#[contracttype]` or public Rust type in `contracts/tree-genetics/src/lib.rs` |
| `SpeciesGenetics` | `#[contracttype]` or public Rust type in `contracts/tree-genetics/src/lib.rs` |
| `TreeGenetics` | `#[contracttype]` or public Rust type in `contracts/tree-genetics/src/lib.rs` |

#### Event topics

| Topic | Source |
|---|---|
| — | No inline `symbol_short!` event topics detected. |

### `tree-registry`

#### Functions

| Name | ABI signature |
|---|---|
| `initialize` | `fn initialize(env: Env, admin: Address, escrow: Address)` |
| `mint_tree` | `fn mint_tree( env: Env, sponsor: Address, species: soroban_sdk::String, region: soroban_sdk::String, planter: Address, ) -> u64` |
| `batch_mint_tree` | `fn batch_mint_tree( env: Env, sponsor: Address, species: Vec<soroban_sdk::String>, region: Vec<soroban_sdk::String>, planter: Vec<Address>, ) -> Vec<u64>` |
| `add_verifier` | `fn add_verifier(env: Env, verifier: Address)` |
| `set_current_month` | `fn set_current_month(env: Env, month: u32)` |
| `get_current_month` | `fn get_current_month(env: Env) -> Option<u32>` |
| `set_planting_season` | `fn set_planting_season( env: Env, region: soroban_sdk::String, start_month: u32, end_month: u32, )` |
| `get_planting_season` | `fn get_planting_season(env: Env, region: soroban_sdk::String) -> Option<SeasonWindow>` |
| `remove_verifier` | `fn remove_verifier(env: Env, verifier: Address)` |
| `get_verifiers` | `fn get_verifiers(env: Env) -> Vec<Address>` |
| `get_planter_score` | `fn get_planter_score(env: Env, planter: Address) -> u64` |
| `verify_tree` | `fn verify_tree( env: Env, verifier: Address, tree_id: u64, approved: bool, notes_hash: Option<soroban_sdk::String>, )` |
| `update_tree_health` | `fn update_tree_health(env: Env, verifier: Address, tree_id: u64, health: TreeHealth)` |
| `batch_update_survival` | `fn batch_update_survival( env: Env, verifier: Address, tree_ids: Vec<u64>, health_states: Vec<TreeHealth>, )` |
| `get_tree` | `fn get_tree(env: Env, id: u64) -> Option<TreeRecord>` |
| `is_tree_dead` | `fn is_tree_dead(env: Env, id: u64) -> bool` |
| `list_by_sponsor` | `fn list_by_sponsor(env: Env, sponsor: Address) -> Vec<TreeRecord>` |
| `claim_milestone` | `fn claim_milestone(env: Env, sponsor: Address, tree_id: u64, milestone_years: u64) -> i128` |
| `tree_count` | `fn tree_count(env: Env) -> u64` |
| `register_species` | `fn register_species(env: Env, slug: Symbol, co2_scaled: i128, maturity_years: u32)` |
| `update_species` | `fn update_species(env: Env, slug: Symbol, co2_scaled: i128, maturity_years: u32)` |
| `get_species_info` | `fn get_species_info(env: Env, slug: Symbol) -> Option<SpeciesInfo>` |
| `unregister_species` | `fn unregister_species(env: Env, slug: Symbol)` |
| `get_distinct_species` | `fn get_distinct_species(env: Env) -> Vec<soroban_sdk::String>` |
| `get_tree_ids_by_species` | `fn get_tree_ids_by_species(env: Env, species: soroban_sdk::String) -> Vec<u64>` |
| `get_species_count` | `fn get_species_count(env: Env, species: soroban_sdk::String) -> u64` |
| `get_tree_ids_by_species_and_region` | `fn get_tree_ids_by_species_and_region( env: Env, species: soroban_sdk::String, region: soroban_sdk::String, ) -> Vec<u64>` |
| `get_tree_ids_by_species_and_status` | `fn get_tree_ids_by_species_and_status( env: Env, species: soroban_sdk::String, status: TreeStatus, ) -> Vec<u64>` |
| `get_species_in_region` | `fn get_species_in_region( env: Env, region: soroban_sdk::String, ) -> Vec<soroban_sdk::String>` |
| `pause` | `fn pause(env: Env)` |
| `unpause` | `fn unpause(env: Env)` |
| `is_paused` | `fn is_paused(env: Env) -> bool` |
| `get_status_history` | `fn get_status_history(env: Env, tree_id: u64) -> Vec<(TreeStatus, u64)>` |
| `get_planter_metrics` | `fn get_planter_metrics(env: Env, wallet: Address) -> PlanterMetrics` |
| `set_planter_bond` | `fn set_planter_bond(env: Env, wallet: Address, amount: i128)` |

#### Public types

| Type | Encoding source |
|---|---|
| `PlanterMetrics` | `#[contracttype]` or public Rust type in `contracts/tree-registry/src/lib.rs` |
| `SeasonWindow` | `#[contracttype]` or public Rust type in `contracts/tree-registry/src/lib.rs` |
| `SpeciesInfo` | `#[contracttype]` or public Rust type in `contracts/tree-registry/src/lib.rs` |
| `TreeHealth` | `#[contracttype]` or public Rust type in `contracts/tree-registry/src/lib.rs` |
| `TreeRecord` | `#[contracttype]` or public Rust type in `contracts/tree-registry/src/lib.rs` |
| `TreeRegistry` | `#[contracttype]` or public Rust type in `contracts/tree-registry/src/lib.rs` |
| `TreeRegistryError` | `#[contracttype]` or public Rust type in `contracts/tree-registry/src/lib.rs` |
| `TreeStatus` | `#[contracttype]` or public Rust type in `contracts/tree-registry/src/lib.rs` |

#### Event topics

| Topic | Source |
|---|---|
| — | No inline `symbol_short!` event topics detected. |

### `tree-retirement`

#### Functions

| Name | ABI signature |
|---|---|
| `initialize` | `fn initialize(env: Env, admin: Address)` |
| `add_issuer` | `fn add_issuer(env: Env, issuer: Address)` |
| `remove_issuer` | `fn remove_issuer(env: Env, issuer: Address)` |
| `get_issuers` | `fn get_issuers(env: Env) -> Vec<IssuerRecord>` |
| `is_issuer` | `fn is_issuer(env: Env, addr: Address) -> bool` |
| `pause` | `fn pause(env: Env)` |
| `unpause` | `fn unpause(env: Env)` |
| `is_paused` | `fn is_paused(env: Env) -> bool` |
| `retire_tree` | `fn retire_tree( env: Env, issuer: Address, sponsor: Address, tree_id: u64, tree_count: i128, co2_offset_kg: i128, reason: String, ) -> u64` |
| `get_retirement_certificate` | `fn get_retirement_certificate(env: Env, retirement_id: u64) -> Option<RetirementCertificate>` |
| `is_tree_retired` | `fn is_tree_retired(env: Env, tree_id: u64) -> bool` |
| `get_retirement_for_tree` | `fn get_retirement_for_tree(env: Env, tree_id: u64) -> Option<RetirementCertificate>` |
| `retirement_count` | `fn retirement_count(env: Env) -> u64` |

#### Public types

| Type | Encoding source |
|---|---|
| `IssuerRecord` | `#[contracttype]` or public Rust type in `contracts/tree-retirement/src/lib.rs` |
| `RetirementCertificate` | `#[contracttype]` or public Rust type in `contracts/tree-retirement/src/lib.rs` |
| `RetirementError` | `#[contracttype]` or public Rust type in `contracts/tree-retirement/src/lib.rs` |
| `TreeRetirement` | `#[contracttype]` or public Rust type in `contracts/tree-retirement/src/lib.rs` |

#### Event topics

| Topic | Source |
|---|---|
| `issAdd` | `env.events().publish` in `contracts/tree-retirement/src/lib.rs` |
| `issRm` | `env.events().publish` in `contracts/tree-retirement/src/lib.rs` |

### `tree-token`

#### Functions

| Name | ABI signature |
|---|---|
| `initialize` | `fn initialize(env: Env, admin: Address, tree_token: Address)` |
| `clawback` | `fn clawback(env: Env, admin: Address, from: Address, amount: i128)` |
| `burn` | `fn burn(env: Env, burner: Address, amount: i128, esg_reference: soroban_sdk::String)` |
| `stake` | `fn stake(env: Env, user: Address, amount: i128)` |
| `unstake` | `fn unstake(env: Env, user: Address, amount: i128)` |
| `claim_reward` | `fn claim_reward(env: Env, user: Address)` |
| `deposit_reward` | `fn deposit_reward(env: Env, admin: Address, amount: i128)` |
| `transfer_meta` | `fn transfer_meta( env: Env, relayer: Address, payload: MetaTransferPayload, signature: BytesN<64>, )` |
| `get_nonce` | `fn get_nonce(env: Env, account: Address) -> u64` |
| `permit` | `fn permit(env: Env, payload: PermitPayload, signature: BytesN<64>)` |
| `transfer_from` | `fn transfer_from(env: Env, spender: Address, owner: Address, to: Address, amount: i128)` |
| `get_permit_nonce` | `fn get_permit_nonce(env: Env, owner: Address) -> u64` |
| `get_allowance` | `fn get_allowance(env: Env, owner: Address, spender: Address) -> Option<Allowance>` |
| `get_burn_record` | `fn get_burn_record(env: Env, idx: u32) -> Option<BurnRecord>` |
| `burn_count` | `fn burn_count(env: Env) -> u32` |
| `pause` | `fn pause(env: Env)` |
| `unpause` | `fn unpause(env: Env)` |
| `is_paused` | `fn is_paused(env: Env) -> bool` |
| `add_to_whitelist` | `fn add_to_whitelist(env: Env, addr: Address)` |
| `remove_from_whitelist` | `fn remove_from_whitelist(env: Env, addr: Address)` |
| `is_whitelisted` | `fn is_whitelisted(env: Env, addr: Address) -> bool` |
| `assert_whitelisted` | `fn assert_whitelisted(env: Env, addr: Address)` |

#### Public types

| Type | Encoding source |
|---|---|
| `Allowance` | `#[contracttype]` or public Rust type in `contracts/tree-token/src/lib.rs` |
| `BridgeLockRecord` | `#[contracttype]` or public Rust type in `contracts/tree-token/src/lib.rs` |
| `BurnRecord` | `#[contracttype]` or public Rust type in `contracts/tree-token/src/lib.rs` |
| `Config` | `#[contracttype]` or public Rust type in `contracts/tree-token/src/lib.rs` |
| `PermitPayload` | `#[contracttype]` or public Rust type in `contracts/tree-token/src/lib.rs` |
| `TreeToken` | `#[contracttype]` or public Rust type in `contracts/tree-token/src/lib.rs` |
| `TreeTokenError` | `#[contracttype]` or public Rust type in `contracts/tree-token/src/lib.rs` |

#### Event topics

| Topic | Source |
|---|---|
| `rew_clm` | `env.events().publish` in `contracts/tree-token/src/lib.rs` |
| `rew_dep` | `env.events().publish` in `contracts/tree-token/src/lib.rs` |
| `staked` | `env.events().publish` in `contracts/tree-token/src/lib.rs` |
| `unstaked` | `env.events().publish` in `contracts/tree-token/src/lib.rs` |

### `tree_registry`

#### Functions

| Name | ABI signature |
|---|---|
| `initialize` | `fn initialize(env: Env)` |
| `mint_anon` | `fn mint_anon( env: Env, species: Symbol, region: Symbol, planter: Address, ) -> u64` |
| `mint_sponsored` | `fn mint_sponsored( env: Env, sponsor: Address, planter: Address, species: Symbol, region: Symbol, ) -> u64` |
| `nft_owner` | `fn nft_owner(env: Env, tree_id: u64) -> Result<Address, Error>` |
| `transfer_nft` | `fn transfer_nft( env: Env, tree_id: u64, from: Address, to: Address, ) -> Result<(), Error>` |
| `trade_nft` | `fn trade_nft( env: Env, tree_id: u64, seller: Address, buyer: Address, payment_token: Address, price: i128, ) -> Result<(), Error>` |
| `get_sponsor_trees` | `fn get_sponsor_trees(env: Env, sponsor: Address) -> Vec<Tree>` |
| `get_tree` | `fn get_tree(env: Env, tree_id: u64) -> Option<Tree>` |
| `get_planter_trees` | `fn get_planter_trees(env: Env, planter: Address) -> Vec<Tree>` |
| `update_status` | `fn update_status(env: Env, tree_id: u64, new_status: TreeStatus) -> Result<(), Error>` |
| `upload_proof` | `fn upload_proof(env: Env, tree_id: u64, photo_cid: Symbol, gps_lat: i64, gps_lon: i64) -> Result<(), Error>` |
| `get_total_trees` | `fn get_total_trees(env: Env) -> u64` |
| `get_total_anonymous_trees` | `fn get_total_anonymous_trees(env: Env) -> u64` |
| `get_next_tree_id` | `fn get_next_tree_id(env: Env) -> u64` |
| `set_escrow_address` | `fn set_escrow_address(env: Env, escrow: Address)` |

#### Public types

| Type | Encoding source |
|---|---|
| `DataKey` | `#[contracttype]` or public Rust type in `contracts/tree_registry/src/lib.rs` |
| `Error` | `#[contracttype]` or public Rust type in `contracts/tree_registry/src/lib.rs` |
| `Tree` | `#[contracttype]` or public Rust type in `contracts/tree_registry/src/lib.rs` |
| `TreeRegistryContract` | `#[contracttype]` or public Rust type in `contracts/tree_registry/src/lib.rs` |
| `TreeStatus` | `#[contracttype]` or public Rust type in `contracts/tree_registry/src/lib.rs` |

#### Event topics

| Topic | Source |
|---|---|
| `anon_minted` | `env.events().publish` in `contracts/tree_registry/src/lib.rs` |
| `proof_upd` | `env.events().publish` in `contracts/tree_registry/src/lib.rs` |
| `status_upd` | `env.events().publish` in `contracts/tree_registry/src/lib.rs` |

### `upgrade-timelock`

#### Functions

| Name | ABI signature |
|---|---|
| `initialize` | `fn initialize(env: Env, admin: Address)` |
| `approve_upgrade` | `fn approve_upgrade(env: Env, target: Address, wasm_hash: BytesN<32>)` |
| `activate_upgrade` | `fn activate_upgrade(env: Env)` |
| `cancel_upgrade` | `fn cancel_upgrade(env: Env)` |
| `set_minimum_delay` | `fn set_minimum_delay(env: Env, delay_seconds: u64)` |
| `minimum_delay` | `fn minimum_delay(env: Env) -> u64` |
| `get_pending_upgrade` | `fn get_pending_upgrade(env: Env) -> UpgradeState` |
| `upgrade` | `fn upgrade(env: Env, wasm_hash: BytesN<32>)` |
| `wasm_hash` | `fn wasm_hash(env: Env) -> BytesN<32>` |

#### Public types

| Type | Encoding source |
|---|---|
| `PendingUpgrade` | `#[contracttype]` or public Rust type in `contracts/upgrade-timelock/src/lib.rs` |
| `UpgradeState` | `#[contracttype]` or public Rust type in `contracts/upgrade-timelock/src/lib.rs` |
| `UpgradeTimelock` | `#[contracttype]` or public Rust type in `contracts/upgrade-timelock/src/lib.rs` |
| `UpgradeTimelockError` | `#[contracttype]` or public Rust type in `contracts/upgrade-timelock/src/lib.rs` |

#### Event topics

| Topic | Source |
|---|---|
| — | No inline `symbol_short!` event topics detected. |

### `verifier-staking`

#### Functions

| Name | ABI signature |
|---|---|
| `initialize` | `fn initialize( env: Env, admin: Address, stake_token: Address, min_stake_amount: i128, governance_contract: Address, replanting_buffer_pool: Address, sla_penalty_amount: i128, )` |
| `register` | `fn register(env: Env, verifier: Address)` |
| `stake` | `fn stake(env: Env, verifier: Address, amount: i128)` |
| `slash` | `fn slash(env: Env, verifier: Address, slash_amount: i128)` |
| `slash_invalid_approval` | `fn slash_invalid_approval( env: Env, verifier: Address, reason: InvalidApprovalKind, ) -> u64` |
| `appeal_slash` | `fn appeal_slash(env: Env, verifier: Address, slash_id: u64, _reason: soroban_sdk::String)` |
| `execute_slash` | `fn execute_slash(env: Env, slash_id: u64)` |
| `get_offense_count` | `fn get_offense_count(env: Env, verifier: Address) -> u32` |
| `slash_escalated` | `fn slash_escalated(env: Env, verifier: Address, base_slash_amount: i128) -> i128` |
| `unstake` | `fn unstake(env: Env, verifier: Address)` |
| `withdraw` | `fn withdraw(env: Env, verifier: Address)` |
| `is_eligible` | `fn is_eligible(env: Env, verifier: Address) -> bool` |
| `is_registered` | `fn is_registered(env: Env, verifier: Address) -> bool` |
| `get_stake` | `fn get_stake(env: Env, verifier: Address) -> Option<VerifierStake>` |
| `get_slash_request` | `fn get_slash_request(env: Env, slash_id: u64) -> Option<SlashRequest>` |
| `get_unbondings` | `fn get_unbondings(env: Env, verifier: Address) -> Vec<Unbonding>` |
| `get_slashed_to_buffer_pool` | `fn get_slashed_to_buffer_pool(env: Env, verifier: Address) -> i128` |
| `get_min_stake` | `fn get_min_stake(env: Env) -> i128` |
| `get_governance_contract` | `fn get_governance_contract(env: Env) -> Address` |
| `get_replanting_buffer_pool` | `fn get_replanting_buffer_pool(env: Env) -> Address` |
| `get_sla_penalty_amount` | `fn get_sla_penalty_amount(env: Env) -> i128` |
| `assign_plot` | `fn assign_plot(env: Env, verifier: Address, plot_id: u64)` |
| `complete_audit` | `fn complete_audit(env: Env, verifier: Address, plot_id: u64)` |
| `penalize_sla` | `fn penalize_sla(env: Env, verifier: Address, plot_id: u64)` |
| `delegate` | `fn delegate(env: Env, delegator: Address, verifier: Address, amount: i128)` |
| `undelegate` | `fn undelegate(env: Env, delegator: Address, verifier: Address)` |
| `get_delegation` | `fn get_delegation(env: Env, verifier: Address, delegator: Address) -> i128` |

#### Public types

| Type | Encoding source |
|---|---|
| `InvalidApprovalKind` | `#[contracttype]` or public Rust type in `contracts/verifier-staking/src/lib.rs` |
| `SlashRequest` | `#[contracttype]` or public Rust type in `contracts/verifier-staking/src/lib.rs` |
| `SlashStatus` | `#[contracttype]` or public Rust type in `contracts/verifier-staking/src/lib.rs` |
| `Unbonding` | `#[contracttype]` or public Rust type in `contracts/verifier-staking/src/lib.rs` |
| `VerifierStake` | `#[contracttype]` or public Rust type in `contracts/verifier-staking/src/lib.rs` |
| `VerifierStaking` | `#[contracttype]` or public Rust type in `contracts/verifier-staking/src/lib.rs` |
| `VerifierStakingError` | `#[contracttype]` or public Rust type in `contracts/verifier-staking/src/lib.rs` |

#### Event topics

| Topic | Source |
|---|---|
| `assigned` | `env.events().publish` in `contracts/verifier-staking/src/lib.rs` |
| `completed` | `env.events().publish` in `contracts/verifier-staking/src/lib.rs` |
| `sla_brch` | `env.events().publish` in `contracts/verifier-staking/src/lib.rs` |

### `zk-location-verifier`

#### Functions

| Name | ABI signature |
|---|---|
| `initialize` | `fn initialize(env: Env, admin: Address)` |
| `initialize_with_cache_ttl` | `fn initialize_with_cache_ttl(env: Env, admin: Address, cache_ttl_ledgers: u32)` |
| `get_proof_cache_ttl` | `fn get_proof_cache_ttl(env: Env) -> u32` |
| `submit_commitment` | `fn submit_commitment( env: Env, farmer: Address, commitment: BytesN<32>, region_index: u32, )` |
| `approve_location` | `fn approve_location(env: Env, commitment: BytesN<32>, proof_digest: BytesN<32>)` |
| `reject_location` | `fn reject_location(env: Env, commitment: BytesN<32>)` |
| `get_verification` | `fn get_verification(env: Env, commitment: BytesN<32>) -> Option<LocationVerification>` |
| `get_proof_digest` | `fn get_proof_digest(env: Env, commitment: BytesN<32>) -> Option<BytesN<32>>` |
| `is_approved` | `fn is_approved(env: Env, commitment: BytesN<32>) -> bool` |
| `batch_submit_commitments` | `fn batch_submit_commitments( env: Env, farmers: Vec<Address>, commitments: Vec<BytesN<32>>, region_indices: Vec<u32>, )` |
| `batch_approve_locations` | `fn batch_approve_locations( env: Env, commitments: Vec<BytesN<32>>, proof_digests: Vec<BytesN<32>>, )` |
| `batch_reject_locations` | `fn batch_reject_locations(env: Env, commitments: Vec<BytesN<32>>)` |

#### Public types

| Type | Encoding source |
|---|---|
| `CachedProofResult` | `#[contracttype]` or public Rust type in `contracts/zk-location-verifier/src/lib.rs` |
| `LocationVerification` | `#[contracttype]` or public Rust type in `contracts/zk-location-verifier/src/lib.rs` |
| `VerificationStatus` | `#[contracttype]` or public Rust type in `contracts/zk-location-verifier/src/lib.rs` |
| `ZkLocationError` | `#[contracttype]` or public Rust type in `contracts/zk-location-verifier/src/lib.rs` |
| `ZkLocationVerifier` | `#[contracttype]` or public Rust type in `contracts/zk-location-verifier/src/lib.rs` |

#### Event topics

| Topic | Source |
|---|---|
| — | No inline `symbol_short!` event topics detected. |

### `zk-verifier`

#### Functions

| Name | ABI signature |
|---|---|
| `initialize` | `fn initialize(env: Env, admin: Address)` |
| `verify_location_proof` | `fn verify_location_proof( env: Env, submitter: Address, proof: ZkProof, inputs: ProofInputs, )` |
| `batch_verify_location_proofs` | `fn batch_verify_location_proofs( env: Env, submitter: Address, proofs: Vec<ZkProof>, public_inputs: Vec<ProofInputs>, ) -> Result<Vec<bool>, ZkError>` |
| `verify_proof` | `fn verify_proof( env: Env, submitter: Address, proof: ZkProof, inputs: ProofInputs, )` |
| `batch_verify` | `fn batch_verify( env: Env, submitter: Address, proofs: Vec<ZkProof>, public_inputs: Vec<ProofInputs>, ) -> Result<Vec<bool>, ZkError>` |
| `is_nullifier_spent` | `fn is_nullifier_spent(env: Env, nullifier_hash: BytesN<32>) -> bool` |
| `get_verification_key_hash` | `fn get_verification_key_hash(env: Env) -> BytesN<32>` |
| `verify_proof_compressed` | `fn verify_proof_compressed( env: Env, submitter: Address, proof: ZkProof, compressed: CompressedProofInputs, )` |
| `batch_verify_compressed` | `fn batch_verify_compressed( env: Env, submitter: Address, proofs: Vec<ZkProof>, compressed_inputs: Vec<CompressedProofInputs>, ) -> Result<Vec<bool>, ZkError>` |
| `compress_inputs` | `fn compress_inputs( env: Env, commitment: BytesN<32>, nullifier_hash: BytesN<32>, proof_timestamp: u64, ) -> CompressedProofInputs` |

#### Public types

| Type | Encoding source |
|---|---|
| `CompressedProofInputs` | `#[contracttype]` or public Rust type in `contracts/zk-verifier/src/lib.rs` |
| `NullifierEntry` | `#[contracttype]` or public Rust type in `contracts/zk-verifier/src/lib.rs` |
| `ProofInputs` | `#[contracttype]` or public Rust type in `contracts/zk-verifier/src/lib.rs` |
| `ZkError` | `#[contracttype]` or public Rust type in `contracts/zk-verifier/src/lib.rs` |
| `ZkProof` | `#[contracttype]` or public Rust type in `contracts/zk-verifier/src/lib.rs` |
| `ZkVerifier` | `#[contracttype]` or public Rust type in `contracts/zk-verifier/src/lib.rs` |

#### Event topics

| Topic | Source |
|---|---|
| — | No inline `symbol_short!` event topics detected. |

## Escrow release security notes

The `escrow` contract exposes `release(tree_id)` and `batch_release(tree_ids)`. Both require authorization from the configured verifier before any state transition. `batch_release` validates a non-empty list of at most 64 IDs and reverts atomically if any item is missing, already settled, or otherwise invalid. This reduces timing and ordering exposure for relayers without granting any new release authority to observers of the transaction pool.

## Regeneration

Run `python3 scripts/generate-contract-interface-docs.py` after changing a contract entry point, public type, or inline event topic.
