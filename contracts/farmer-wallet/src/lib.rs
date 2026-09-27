#![no_std]

//! Farmer Wallet — multi-chain credit routing (v2).
//!
//! Closes #1434. Builds on the hashlock/timelock settlement of the v1
//! `atomic-swap` contract (#1380) and turns it into a routable, multi-chain
//! farmer wallet.
//!
//! # Problem
//!
//! Carbon credits are cheapest to move on some chains and cheapest to hold on
//! others. A farmer who only ever operates on Stellar overpays bridge fees and
//! relayer costs and cannot capture better execution prices on Polygon or
//! Ethereum. Farmers also need a single identity that is recognisable on
//! every chain they settle on.
//!
//! # Design
//!
//! This contract is the **Stellar-side coordination ledger** for a farmer's
//! multi-chain wallet. It is deliberately *not* a bridge: it never touches
//! non-Stellar state. Instead it
//!
//! 1. holds one wallet profile per farmer, with an explicitly registered
//!    destination address on every EVM chain the farmer settles on;
//! 2. prices a transfer across the enabled chains and picks the cheapest
//!    route — direct, or two-hop through the remaining chain — which is the
//!    "optimal pricing/fees" requirement;
//! 3. escrows the credits on Stellar and records the transfer so a relayer can
//!    settle the destination-chain leg; and
//! 4. refunds the farmer when the destination leg does not settle before the
//!    farmer's deadline.
//!
//! ## Chains
//!
//! `Stellar` is the home chain and the implicit source of every outbound
//! transfer: the farmer's Stellar `Address` *is* their Stellar address, so it
//! cannot be registered or overridden. `Polygon` and `Ethereum` carry
//! externally-owned `0x…` addresses that must be registered up front, which
//! removes a whole class of misdirected-funds incidents.
//!
//! ## Pricing
//!
//! Each hop `a -> b` is priced as
//!
//! ```text
//! fee(amount) = fixed_fee(a) + amount * fee_bps(a) / 10_000
//! ```
//!
//! and the hop output feeds the next hop, so a two-hop route pays the
//! intermediate chain's fee on the already-reduced amount. A route is viable
//! only when both chains are enabled, the amount sits inside the leaving
//! chain's `[min_amount, max_amount]` band, and the arriving chain has enough
//! unencumbered liquidity to absorb the output. The router scores every
//! candidate by final `amount_out`, breaking ties on settlement time and then
//! hop count, so the result is deterministic.
//!
//! Quotes cannot go stale: `open_transfer` re-prices on chain and enforces the
//! caller's `min_amount_out` slippage bound plus a `deadline`, and the fee that
//! is actually charged is the one stored on the transfer at open time — later
//! admin fee changes cannot reprice a live transfer.
//!
//! ## Trust model
//!
//! `admin` owns chain configuration, the relayer set and the treasury.
//! Relayers are the bridge operators: they attest the destination-chain leg
//! (`settle_transfer`) and the source-chain burn (`relay_inbound`). A relayer
//! can therefore move escrowed credits, so the relayer set is expected to be
//! operated as a multisig, `pause` is the emergency stop, and every relayer
//! action emits an on-chain record plus a unique transaction-hash replay
//! guard. Liquidity providers pre-fund the vault; nothing here mints.

use harvesta_errors::HarvestaError;
use soroban_sdk::{
    contract, contracterror, contractimpl, contracttype, panic_with_error, symbol_short, token,
    Address, BytesN, Env, String, Vec,
};

// ── Error codes ───────────────────────────────────────────────────────────────

/// Farmer-wallet specific error codes.
#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum WalletError {
    /// No chain configuration exists for the referenced chain.
    ChainNotConfigured = 500,
    /// The chain is configured but administratively disabled.
    ChainNotEnabled = 501,
    /// `fee_bps` exceeds the protocol ceiling.
    FeeBpsOutOfRange = 502,
    /// `max_amount` is below `min_amount`, above the protocol cap, or
    /// `min_amount` is negative.
    InvalidAmountBand = 503,
    /// `fixed_fee` or `liquidity` was negative.
    NegativeAmount = 504,
    /// The amount is below the leaving chain's `min_amount`.
    AmountBelowMinimum = 505,
    /// The amount is above the leaving chain's `max_amount`.
    AmountAboveMaximum = 506,
    /// No enabled, liquid route exists for the requested pair and amount.
    NoRouteAvailable = 507,
    /// `deadline` is in the past, or further out than the protocol allows.
    InvalidDeadline = 508,
    /// The re-priced route delivers less than the caller's `min_amount_out`.
    SlippageExceeded = 509,
    /// `min_amount_out` is zero or above `amount`.
    InvalidMinAmountOut = 510,
    /// The farmer has no wallet profile.
    WalletNotRegistered = 511,
    /// The farmer already has a wallet profile.
    WalletAlreadyRegistered = 512,
    /// The farmer's wallet is frozen.
    WalletInactive = 513,
    /// No destination address is registered on the requested chain.
    AddressNotRegistered = 514,
    /// The submitted `0x…` address is not a valid 20-byte hex address.
    InvalidChainAddress = 515,
    /// The Stellar address is implicit and cannot be registered or replaced.
    AddressIsImplicit = 516,
    /// The source and destination chains are the same.
    SameChain = 517,
    /// The transfer does not exist.
    TransferNotFound = 518,
    /// The transfer is not in the `Pending` state.
    TransferNotPending = 519,
    /// The transfer's deadline has passed; the farmer should refund instead.
    TransferExpired = 520,
    /// The transfer has not reached its deadline yet.
    TransferNotExpired = 521,
    /// The supplied transaction hash was already consumed by another transfer.
    ReplayDetected = 522,
    /// The vault does not hold enough credits to honour the payout or fee.
    InsufficientLiquidity = 523,
    /// The caller is not in the authorized relayer set.
    NotRelayer = 524,
    /// Address is already in the relayer set.
    RelayerAlreadyExists = 525,
    /// Address is not in the relayer set.
    RelayerNotFound = 526,
    /// Cannot remove the last relayer — settlement would become impossible.
    CannotRemoveLastRelayer = 527,
    /// The relayer set is empty, so settlement is impossible.
    RelayersNotSet = 528,
    /// The farmer still has unsettled transfers.
    WalletHasPendingTransfers = 529,
    /// The chain still has unsettled transfers routed through it.
    ChainHasPendingTransfers = 530,
    /// The treasury address is not usable.
    InvalidTreasury = 531,
    /// The page cursor lies past the end of the wallet's transfer history.
    InvalidCursor = 532,
    /// `limit` is zero or above the maximum page size.
    InvalidPageSize = 533,
}

// ── Constants ─────────────────────────────────────────────────────────────────

/// Basis-point denominator.
const BPS_DENOM: i128 = 10_000;

/// Protocol fee ceiling: 10% of the transferred amount.
const MAX_FEE_BPS: u32 = 1_000;

/// Hard cap on a single transfer, independent of chain configuration. Bounds
/// the intermediate `amount * fee_bps` product so it cannot overflow.
const HARD_MAX_AMOUNT: i128 = 1_000_000_000_000_000;

/// Longest refund window a farmer may request.
const MAX_DEADLINE_SECONDS: u64 = 3_600;

/// Downward probes `max_transfer` may spend confirming its analytic ceiling.
/// Bounded so the call stays cheap; each probe is the same work as a quote.
const MAX_TRANSFER_PROBES: u32 = 6;

/// Largest page returned by `get_wallet_transfers`.
const MAX_PAGE_SIZE: u32 = 50;

/// Length of a `0x`-prefixed, 20-byte hex EVM address.
const EVM_ADDRESS_LEN: u32 = 42;

// ── Types ─────────────────────────────────────────────────────────────────────

/// A blockchain the farmer wallet can route credits across.
#[contracttype]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Chain {
    /// Home chain. Every outbound transfer originates here.
    Stellar,
    /// Polygon PoS.
    Polygon,
    /// Ethereum mainnet.
    Ethereum,
}

/// A farmer's address on a given chain.
///
/// Stellar is implicit — the farmer's own `Address` — so only EVM chains ever
/// carry a `String`.
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ChainAddress {
    /// The farmer's Stellar account.
    Native(Address),
    /// A `0x`-prefixed, 20-byte hex address on an EVM chain.
    Evm(String),
}

/// Per-chain routing and settlement configuration.
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ChainConfig {
    pub chain: Chain,
    /// Disabled chains are not routable, and cannot be disabled while
    /// transfers through them are still in flight.
    pub enabled: bool,
    /// Variable fee charged on the amount leaving this chain, in basis points.
    pub fee_bps: u32,
    /// Fixed relayer cost charged on top of the basis-point fee.
    pub fixed_fee: i128,
    /// Credit liquidity this chain advertises to the router.
    pub liquidity: i128,
    /// Settled (wrapped) credit token contract for this chain. On Stellar this
    /// is the SAC-wrapped credit token the wallet escrows against.
    pub asset: Address,
    /// Smallest routable amount leaving this chain.
    pub min_amount: i128,
    /// Largest routable amount leaving this chain.
    pub max_amount: i128,
    /// Advertised settlement time, used only as a route tie-break.
    pub settlement_seconds: u64,
    pub updated_at: u64,
}

/// One `a -> b` leg of a route.
#[contracttype]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct RouteHop {
    pub from: Chain,
    pub to: Chain,
}

/// A priced route.
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RouteQuote {
    /// Hops in order. A single hop is a direct transfer.
    pub hops: Vec<RouteHop>,
    /// Gross amount leaving the source chain.
    pub amount_in: i128,
    /// Net amount delivered on the destination chain.
    pub amount_out: i128,
    /// Sum of every hop's fee, denominated in the source asset.
    pub total_fee: i128,
    /// Sum of every hop's advertised settlement time.
    pub estimated_seconds: u64,
    pub quoted_at: u64,
}

impl RouteQuote {
    /// Number of hops in the route.
    #[must_use]
    pub fn hop_count(&self) -> u32 {
        self.hops.len()
    }
}

/// Which side of the bridge a transfer was opened from.
#[contracttype]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Direction {
    /// Stellar to EVM. Credits are pulled from the farmer and escrowed.
    Outbound,
    /// EVM to Stellar. Credits are paid out of the vault.
    Inbound,
}

/// Lifecycle of a cross-chain credit transfer.
#[contracttype]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum TransferStatus {
    /// Escrowed on Stellar, awaiting destination-chain settlement.
    Pending,
    /// Destination leg attested and the fee collected.
    Settled,
    /// Deadline passed and the gross amount returned to the farmer.
    Refunded,
}

/// An on-chain record of one cross-chain credit movement.
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CreditTransfer {
    pub id: u64,
    pub farmer: Address,
    pub direction: Direction,
    pub source_chain: Chain,
    pub dest_chain: Chain,
    /// Registered destination address for `dest_chain`.
    pub destination: ChainAddress,
    /// Gross amount leaving the source chain, in source-chain units.
    pub amount: i128,
    /// Fee collected for the route, in source-chain units. Locked in at open
    /// time, so later admin fee changes cannot reprice a live transfer.
    pub fee: i128,
    /// Net amount delivered on the destination chain, in destination units.
    pub net_amount: i128,
    /// Hops in the route that produced the fee.
    pub hops: u32,
    pub status: TransferStatus,
    pub opened_at: u64,
    /// Farmer-chosen deadline. A `Pending` transfer can be refunded after it.
    pub expires_at: u64,
    pub settled_at: u64,
    /// Relayer that attested settlement, or the contract itself at open time.
    pub settled_by: Address,
    /// Source- or destination-chain transaction hash, depending on direction.
    pub chain_tx: BytesN<32>,
}

/// Per-farmer wallet profile.
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WalletProfile {
    pub farmer: Address,
    /// Monotonic wallet identifier, assigned at registration.
    pub wallet_id: u64,
    pub created_at: u64,
    /// A frozen wallet keeps its addresses and history but cannot move credits.
    pub active: bool,
    /// Lifetime number of transfers opened from this wallet.
    pub transfer_count: u64,
    pub updated_at: u64,
}

// ── Storage keys ──────────────────────────────────────────────────────────────

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
enum DataKey {
    Admin,
    Treasury,
    Paused,
    Relayers,
    ChainConfig(Chain),
    /// Destination-chain liquidity reserved by transfers still in flight.
    Pending(Chain),
    WalletSeq,
    Wallet(Address),
    /// Chains the farmer has explicitly registered an address on.
    RegisteredChains(Address),
    ChainAddress(Address, Chain),
    /// Ordered transfer IDs, for pagination.
    TransferIds(Address),
    TransferSeq,
    Transfer(u64),
    /// Replay guard for source/destination transaction hashes.
    ConsumedTx(BytesN<32>),
}

// ── Contract ──────────────────────────────────────────────────────────────────

#[contract]
pub struct FarmerWallet;

#[contractimpl]
impl FarmerWallet {
    // ── Initialization ────────────────────────────────────────────────────────

    /// One-time initialisation.
    ///
    /// `admin` manages chain configuration, the relayer set and the treasury.
    /// `treasury` collects routing fees and cannot be the contract itself.
    ///
    /// # Errors
    /// - `AlreadyInitialized` if the contract is already set up.
    /// - `InvalidTreasury` if `treasury` is the contract address.
    pub fn initialize(env: Env, admin: Address, treasury: Address) {
        if env.storage().instance().has(&DataKey::Admin) {
            panic_with_error!(&env, HarvestaError::AlreadyInitialized);
        }
        if treasury == env.current_contract_address() {
            panic_with_error!(&env, WalletError::InvalidTreasury);
        }
        admin.require_auth();
        env.storage().instance().set(&DataKey::Admin, &admin);
        env.storage().instance().set(&DataKey::Treasury, &treasury);
        env.storage().instance().set(&DataKey::Paused, &false);
        env.storage().instance().set(&DataKey::WalletSeq, &0u64);
        env.storage().instance().set(&DataKey::TransferSeq, &0u64);
        let empty: Vec<Address> = Vec::new(&env);
        env.storage().instance().set(&DataKey::Relayers, &empty);
    }

    // ── Chain configuration ───────────────────────────────────────────────────

    /// Create or replace a chain's routing configuration. Admin only.
    ///
    /// # Errors
    /// - `FeeBpsOutOfRange` if `fee_bps` exceeds the 10% ceiling.
    /// - `InvalidAmountBand` if `min_amount` is negative, if `max_amount`
    ///   exceeds the protocol cap, or if `max_amount < min_amount`.
    /// - `NegativeAmount` if `fixed_fee` or `liquidity` is negative.
    /// - `ChainHasPendingTransfers` if the chain is being disabled while
    ///   transfers routed through it are still unsettled.
    pub fn set_chain_config(env: Env, config: ChainConfig) {
        Self::require_admin(&env);

        if config.fee_bps > MAX_FEE_BPS {
            panic_with_error!(&env, WalletError::FeeBpsOutOfRange);
        }
        if config.fixed_fee < 0 || config.liquidity < 0 {
            panic_with_error!(&env, WalletError::NegativeAmount);
        }
        if config.min_amount < 0
            || config.max_amount > HARD_MAX_AMOUNT
            || config.max_amount < config.min_amount
        {
            panic_with_error!(&env, WalletError::InvalidAmountBand);
        }
        if !config.enabled && Self::pending(&env, &config.chain) > 0 {
            panic_with_error!(&env, WalletError::ChainHasPendingTransfers);
        }

        let chain = config.chain;
        let stored = ChainConfig {
            updated_at: env.ledger().timestamp(),
            ..config
        };
        env.storage()
            .instance()
            .set(&DataKey::ChainConfig(chain), &stored);
        env.events().publish((symbol_short!("chainCfg"),), chain);
    }

    /// Returns a chain's routing configuration, or `None` when unconfigured.
    #[must_use]
    pub fn get_chain_config(env: Env, chain: Chain) -> Option<ChainConfig> {
        env.storage().instance().get(&DataKey::ChainConfig(chain))
    }

    /// Returns `true` if `chain` is configured and enabled.
    #[must_use]
    pub fn is_chain_enabled(env: Env, chain: Chain) -> bool {
        Self::is_enabled(&env, &chain)
    }

    // ── Relayer set ───────────────────────────────────────────────────────────

    /// Add a bridge operator. Admin only. Mirrors the issuer-set pattern used
    /// by `nft-certificate` and `tree-retirement`.
    ///
    /// # Errors
    /// - `RelayerAlreadyExists` if the address is already a relayer.
    pub fn add_relayer(env: Env, relayer: Address) {
        Self::require_admin(&env);
        let mut relayers = Self::relayers(&env);
        if relayers.iter().any(|a| a == relayer) {
            panic_with_error!(&env, WalletError::RelayerAlreadyExists);
        }
        relayers.push_back(relayer.clone());
        env.storage().instance().set(&DataKey::Relayers, &relayers);
        env.events().publish((symbol_short!("relAdd"),), relayer);
    }

    /// Remove a bridge operator. Admin only. The last relayer cannot be
    /// removed, because settlement would become impossible.
    ///
    /// # Errors
    /// - `RelayerNotFound` if the address is not a relayer.
    /// - `CannotRemoveLastRelayer` if it is the only one left.
    pub fn remove_relayer(env: Env, relayer: Address) {
        Self::require_admin(&env);
        let relayers = Self::relayers(&env);
        if !relayers.iter().any(|a| a == relayer) {
            panic_with_error!(&env, WalletError::RelayerNotFound);
        }
        if relayers.len() <= 1 {
            panic_with_error!(&env, WalletError::CannotRemoveLastRelayer);
        }
        let mut updated: Vec<Address> = Vec::new(&env);
        for addr in relayers.iter() {
            if addr != relayer {
                updated.push_back(addr);
            }
        }
        env.storage().instance().set(&DataKey::Relayers, &updated);
        env.events().publish((symbol_short!("relRm"),), relayer);
    }

    /// Returns every authorized relayer.
    #[must_use]
    pub fn get_relayers(env: Env) -> Vec<Address> {
        Self::relayers(&env)
    }

    /// Returns `true` if `addr` is an authorized relayer.
    #[must_use]
    pub fn is_relayer(env: Env, addr: Address) -> bool {
        Self::relayers(&env).iter().any(|a| a == addr)
    }

    // ── Treasury and pause ────────────────────────────────────────────────────

    /// Point the fee treasury at a new address. Admin only.
    ///
    /// # Errors
    /// - `InvalidTreasury` if `treasury` is the contract address.
    pub fn set_treasury(env: Env, treasury: Address) {
        Self::require_admin(&env);
        if treasury == env.current_contract_address() {
            panic_with_error!(&env, WalletError::InvalidTreasury);
        }
        env.storage().instance().set(&DataKey::Treasury, &treasury);
        env.events().publish((symbol_short!("treasury"),), treasury);
    }

    /// Returns the current fee treasury.
    #[must_use]
    pub fn get_treasury(env: Env) -> Address {
        Self::treasury(&env)
    }

    /// Admin-only emergency stop. Blocks wallet and transfer state changes.
    pub fn pause(env: Env) {
        Self::require_admin(&env);
        env.storage().instance().set(&DataKey::Paused, &true);
        env.events().publish((symbol_short!("pause"),), ());
    }

    /// Admin-only resume.
    pub fn unpause(env: Env) {
        Self::require_admin(&env);
        env.storage().instance().set(&DataKey::Paused, &false);
        env.events().publish((symbol_short!("unpause"),), ());
    }

    #[must_use]
    pub fn is_paused(env: Env) -> bool {
        env.storage()
            .instance()
            .get(&DataKey::Paused)
            .unwrap_or(false)
    }

    // ── Wallet registration ───────────────────────────────────────────────────

    /// Register a wallet for `farmer` and return its wallet ID.
    ///
    /// The farmer's Stellar address is the implicit Stellar chain address, so
    /// this also seeds the registered-chain list with `Stellar`.
    ///
    /// # Errors
    /// - `WalletAlreadyRegistered` if the farmer already has a wallet.
    pub fn register_wallet(env: Env, farmer: Address) -> u64 {
        Self::assert_not_paused(&env);
        farmer.require_auth();
        if env
            .storage()
            .persistent()
            .has(&DataKey::Wallet(farmer.clone()))
        {
            panic_with_error!(&env, WalletError::WalletAlreadyRegistered);
        }

        let wallet_id: u64 = env
            .storage()
            .instance()
            .get(&DataKey::WalletSeq)
            .unwrap_or(0)
            + 1;
        let now = env.ledger().timestamp();
        let profile = WalletProfile {
            farmer: farmer.clone(),
            wallet_id,
            created_at: now,
            active: true,
            transfer_count: 0,
            updated_at: now,
        };
        env.storage()
            .persistent()
            .set(&DataKey::Wallet(farmer.clone()), &profile);
        env.storage()
            .instance()
            .set(&DataKey::WalletSeq, &wallet_id);

        let mut chains: Vec<Chain> = Vec::new(&env);
        chains.push_back(Chain::Stellar);
        env.storage()
            .persistent()
            .set(&DataKey::RegisteredChains(farmer.clone()), &chains);
        let ids: Vec<u64> = Vec::new(&env);
        env.storage()
            .persistent()
            .set(&DataKey::TransferIds(farmer.clone()), &ids);

        env.events()
            .publish((symbol_short!("wallet"), farmer), wallet_id);
        wallet_id
    }

    /// Activate or freeze a wallet. Farmer only.
    ///
    /// # Errors
    /// - `WalletNotRegistered` if the farmer has no wallet.
    /// - `WalletHasPendingTransfers` if freezing while transfers are unsettled.
    pub fn set_wallet_active(env: Env, farmer: Address, active: bool) {
        Self::assert_not_paused(&env);
        farmer.require_auth();
        let mut profile = Self::require_wallet(&env, &farmer);
        if !active && Self::open_transfer_count(&env, &farmer) > 0 {
            panic_with_error!(&env, WalletError::WalletHasPendingTransfers);
        }
        profile.active = active;
        profile.updated_at = env.ledger().timestamp();
        env.storage()
            .persistent()
            .set(&DataKey::Wallet(farmer), &profile);
    }

    /// Returns the farmer's wallet profile, or `None`.
    #[must_use]
    pub fn get_wallet(env: Env, farmer: Address) -> Option<WalletProfile> {
        env.storage().persistent().get(&DataKey::Wallet(farmer))
    }

    // ── Chain addresses ───────────────────────────────────────────────────────

    /// Register or replace the farmer's `0x…` address on an EVM chain.
    ///
    /// Registering up front is what keeps a transfer from being sent to a
    /// mistyped or attacker-supplied destination. `Stellar` is rejected: the
    /// farmer's Stellar `Address` is already their Stellar address.
    ///
    /// # Errors
    /// - `AddressIsImplicit` for `Stellar`.
    /// - `InvalidChainAddress` if `address` is not `0x` plus 40 hex digits.
    /// - `ChainNotConfigured` if the chain is absent or disabled.
    /// - `WalletHasPendingTransfers` while transfers are unsettled.
    /// - `WalletNotRegistered` if the farmer has no wallet.
    pub fn set_chain_address(env: Env, farmer: Address, chain: Chain, address: String) {
        Self::assert_not_paused(&env);
        farmer.require_auth();
        let _ = Self::require_wallet(&env, &farmer);
        if chain == Chain::Stellar {
            panic_with_error!(&env, WalletError::AddressIsImplicit);
        }
        if !Self::is_enabled(&env, &chain) {
            panic_with_error!(&env, WalletError::ChainNotConfigured);
        }
        if !Self::is_valid_evm_address(&address) {
            panic_with_error!(&env, WalletError::InvalidChainAddress);
        }
        if Self::open_transfer_count(&env, &farmer) > 0 {
            panic_with_error!(&env, WalletError::WalletHasPendingTransfers);
        }

        let key = DataKey::ChainAddress(farmer.clone(), chain);
        let already_registered = env.storage().persistent().has(&key);
        env.storage()
            .persistent()
            .set(&key, &ChainAddress::Evm(address.clone()));

        if !already_registered {
            let mut chains: Vec<Chain> = env
                .storage()
                .persistent()
                .get(&DataKey::RegisteredChains(farmer.clone()))
                .unwrap_or_else(|| Vec::new(&env));
            chains.push_back(chain);
            env.storage()
                .persistent()
                .set(&DataKey::RegisteredChains(farmer.clone()), &chains);
        }

        env.events()
            .publish((symbol_short!("addrSet"), farmer), (chain, address));
    }

    /// Returns the farmer's address on `chain`, or `None` when unregistered.
    /// `Stellar` resolves to the farmer's own account once a wallet exists.
    #[must_use]
    pub fn get_chain_address(env: Env, farmer: Address, chain: Chain) -> Option<ChainAddress> {
        if chain == Chain::Stellar {
            return env
                .storage()
                .persistent()
                .get(&DataKey::Wallet(farmer))
                .map(|profile: WalletProfile| ChainAddress::Native(profile.farmer));
        }
        env.storage()
            .persistent()
            .get(&DataKey::ChainAddress(farmer, chain))
    }

    /// Returns the chains the farmer has registered. Always includes
    /// `Stellar` once a wallet exists.
    #[must_use]
    pub fn get_wallet_chains(env: Env, farmer: Address) -> Vec<Chain> {
        env.storage()
            .persistent()
            .get(&DataKey::RegisteredChains(farmer))
            .unwrap_or_else(|| Vec::new(&env))
    }

    // ── Routing ───────────────────────────────────────────────────────────────

    /// Price the cheapest available route for `amount` between two chains.
    ///
    /// Candidates are the direct hop and every two-hop route through the
    /// remaining chain. Returns `None` when no candidate is viable — that is a
    /// normal outcome rather than an error, so callers can fall back.
    #[must_use]
    pub fn get_best_route(
        env: Env,
        source_chain: Chain,
        dest_chain: Chain,
        amount: i128,
    ) -> Option<RouteQuote> {
        if source_chain == dest_chain || amount <= 0 {
            return None;
        }
        Self::best_quote(&env, source_chain, dest_chain, amount)
    }

    /// Largest amount currently routable between two chains, or `0` when no
    /// route exists.
    #[must_use]
    /// Largest amount that can currently leave `source_chain` for
    /// `dest_chain`, or `0` when no amount is routable.
    ///
    /// The answer is advisory: settlement and relayer activity move available
    /// liquidity between calls, so a farmer should still quote before opening a
    /// transfer and pass `min_amount_out` as the real slippage guard.
    pub fn max_transfer(env: Env, source_chain: Chain, dest_chain: Chain) -> i128 {
        if source_chain == dest_chain {
            return 0;
        }
        let (Some(src), Some(dst)) = (
            Self::chain_config(&env, &source_chain),
            Self::chain_config(&env, &dest_chain),
        ) else {
            return 0;
        };
        if !src.enabled || !dst.enabled {
            return 0;
        }
        let band = src.max_amount.min(dst.max_amount).min(HARD_MAX_AMOUNT);
        let available = Self::available(&env, &dest_chain);
        if band <= 0 || available <= 0 {
            return 0;
        }

        // Gross-amount ceiling implied by the source chain's fee and the
        // destination's available liquidity. A hop delivering `x` keeps
        // `x - floor(x * bps / 10_000) - fixed_fee`, and `floor(u) >= u - 1`,
        // so requiring `x * (10_000 - bps) / 10_000 <= available - 1 + fixed_fee`
        // is sufficient for the net to fit. `MAX_FEE_BPS` keeps the divisor
        // positive, and `band` caps the product regardless of the inputs.
        let room = available.saturating_add(src.fixed_fee).saturating_sub(1);
        let divisor = BPS_DENOM - i128::from(src.fee_bps);
        let cap = band.min(room.saturating_mul(BPS_DENOM) / divisor);
        if cap <= 0 {
            return 0;
        }
        if Self::best_quote(&env, source_chain, dest_chain, cap).is_some() {
            return cap;
        }

        // The ceiling ignores the amount bands, can overshoot by a unit or two
        // through rounding, and a two-hop detour may only open up well below it.
        // Walk down a short ladder before giving up.
        let mut probe = cap;
        for _ in 0..MAX_TRANSFER_PROBES {
            probe = probe * 3 / 4;
            if probe <= 0 {
                break;
            }
            if Self::best_quote(&env, source_chain, dest_chain, probe).is_some() {
                return probe;
            }
        }
        0
    }

    // ── Outbound transfers (Stellar to EVM) ───────────────────────────────────

    /// Open a cross-chain transfer out of the farmer's Stellar balance.
    ///
    /// The gross `amount` is pulled from the farmer and escrowed, the route is
    /// priced on chain, and the resulting fee is locked in for settlement.
    ///
    /// `min_amount_out` is a slippage bound: if the re-priced route delivers
    /// less, the transfer reverts instead of executing at a worse rate.
    /// `deadline` bounds the refund window and cannot exceed one hour.
    ///
    /// # Errors
    /// - `AddressNotRegistered` if the farmer has no address on `dest_chain`.
    /// - `SameChain` if `dest_chain` is `Stellar`.
    /// - `AmountMustBePositive` on a non-positive `amount`.
    /// - `AmountBelowMinimum` / `AmountAboveMaximum` on an out-of-band amount.
    /// - `InvalidMinAmountOut` if `min_amount_out` is zero or above `amount`.
    /// - `InvalidDeadline` if `deadline` is in the past or too far out.
    /// - `NoRouteAvailable` if nothing is currently routable.
    /// - `SlippageExceeded` if the route delivers under `min_amount_out`.
    pub fn open_transfer(
        env: Env,
        farmer: Address,
        dest_chain: Chain,
        amount: i128,
        min_amount_out: i128,
        deadline: u64,
    ) -> u64 {
        Self::assert_not_paused(&env);
        farmer.require_auth();
        let profile = Self::require_wallet(&env, &farmer);
        if !profile.active {
            panic_with_error!(&env, WalletError::WalletInactive);
        }
        if dest_chain == Chain::Stellar {
            panic_with_error!(&env, WalletError::SameChain);
        }
        if amount <= 0 {
            panic_with_error!(&env, HarvestaError::AmountMustBePositive);
        }
        if min_amount_out <= 0 || min_amount_out > amount {
            panic_with_error!(&env, WalletError::InvalidMinAmountOut);
        }
        Self::check_deadline(&env, deadline);

        // The first hop leaves Stellar, so Stellar's band gates the amount.
        let stellar = Self::require_config(&env, &Chain::Stellar);
        if amount < stellar.min_amount {
            panic_with_error!(&env, WalletError::AmountBelowMinimum);
        }
        if amount > stellar.max_amount {
            panic_with_error!(&env, WalletError::AmountAboveMaximum);
        }

        let destination = Self::require_dest_address(&env, &farmer, &dest_chain);
        let quote = Self::require_quote(&env, Chain::Stellar, dest_chain, amount);
        if quote.amount_out < min_amount_out {
            panic_with_error!(&env, WalletError::SlippageExceeded);
        }

        // Escrow the gross amount: the fee moves to the treasury on settlement
        // and the net stays behind as destination-chain backing.
        token::Client::new(&env, &stellar.asset).transfer(
            &farmer,
            env.current_contract_address(),
            &amount,
        );

        let id = Self::store_transfer(
            &env,
            &farmer,
            Direction::Outbound,
            Chain::Stellar,
            dest_chain,
            destination,
            amount,
            &quote,
            deadline,
            env.current_contract_address(),
            &BytesN::from_array(&env, &[0u8; 32]),
        );
        env.events().publish(
            (symbol_short!("xferOpen"), farmer),
            (id, dest_chain, amount, quote.amount_out),
        );
        id
    }

    // ── Inbound transfers (EVM to Stellar) ────────────────────────────────────

    /// Release credits into the farmer's Stellar wallet for a burn that
    /// already happened on `source_chain`. Relayer only.
    ///
    /// The relayer supplies the source-chain transaction hash, which the
    /// contract uses as a replay guard. The fee is collected on the source
    /// chain by the relayer, so the vault pays only `amount_out` here — there
    /// is no refund window and the transfer is `Settled` from the moment it
    /// opens.
    ///
    /// # Errors
    /// - `NotRelayer` if the caller is not in the relayer set.
    /// - `SameChain` if `source_chain` is `Stellar`.
    /// - `ReplayDetected` if `source_tx` was already consumed.
    /// - `NoRouteAvailable` if the vault holds too little Stellar credit to
    ///   settle the leg.
    /// - `SlippageExceeded` if the route delivers under `min_amount_out`.
    pub fn relay_inbound(
        env: Env,
        relayer: Address,
        farmer: Address,
        source_chain: Chain,
        source_tx: BytesN<32>,
        amount: i128,
        min_amount_out: i128,
    ) -> u64 {
        Self::assert_not_paused(&env);
        Self::require_relayer(&env, &relayer);
        let profile = Self::require_wallet(&env, &farmer);
        if !profile.active {
            panic_with_error!(&env, WalletError::WalletInactive);
        }
        if source_chain == Chain::Stellar {
            panic_with_error!(&env, WalletError::SameChain);
        }
        if amount <= 0 {
            panic_with_error!(&env, HarvestaError::AmountMustBePositive);
        }
        if min_amount_out <= 0 || min_amount_out > amount {
            panic_with_error!(&env, WalletError::InvalidMinAmountOut);
        }
        Self::consume_tx(&env, &source_tx);

        let quote = Self::require_quote(&env, source_chain, Chain::Stellar, amount);
        if quote.amount_out < min_amount_out {
            panic_with_error!(&env, WalletError::SlippageExceeded);
        }

        let stellar = Self::require_config(&env, &Chain::Stellar);
        let vault = env.current_contract_address();
        // `require_quote` already proved the destination chain has `available`
        // liquidity, and `available` is capped by the balance actually held, so
        // this cannot normally fire. It stays as a last line of defence so a
        // payout can never be attempted from an empty vault.
        let balance = token::Client::new(&env, &stellar.asset).balance(&vault);
        if balance < quote.amount_out {
            panic_with_error!(&env, WalletError::InsufficientLiquidity);
        }
        token::Client::new(&env, &stellar.asset).transfer(&vault, &farmer, &quote.amount_out);

        let id = Self::store_transfer(
            &env,
            &farmer,
            Direction::Inbound,
            source_chain,
            Chain::Stellar,
            ChainAddress::Native(farmer.clone()),
            amount,
            &quote,
            0,
            relayer.clone(),
            &source_tx,
        );
        env.events().publish(
            (symbol_short!("xferOpen"), farmer),
            (id, source_chain, quote.amount_out, relayer),
        );
        id
    }

    // ── Settlement and refunds ────────────────────────────────────────────────

    /// Attest that the destination-chain leg of an outbound transfer
    /// completed, releasing its fee to the treasury. Relayer only.
    ///
    /// The escrowed net amount stays in the vault as backing for the credits
    /// minted on the destination chain. `destination_tx` is consumed as a
    /// replay guard.
    ///
    /// # Errors
    /// - `NotRelayer` if the caller is not in the relayer set.
    /// - `TransferNotFound` / `TransferNotPending` on a bad state.
    /// - `TransferExpired` once the refund window has closed.
    /// - `ReplayDetected` if `destination_tx` was already consumed.
    /// - `InsufficientLiquidity` if the vault cannot cover the fee.
    pub fn settle_transfer(
        env: Env,
        relayer: Address,
        transfer_id: u64,
        destination_tx: BytesN<32>,
    ) -> i128 {
        Self::assert_not_paused(&env);
        Self::require_relayer(&env, &relayer);
        let mut transfer = Self::require_transfer(&env, transfer_id);
        if transfer.status != TransferStatus::Pending {
            panic_with_error!(&env, WalletError::TransferNotPending);
        }
        if env.ledger().timestamp() >= transfer.expires_at {
            panic_with_error!(&env, WalletError::TransferExpired);
        }
        Self::consume_tx(&env, &destination_tx);

        let stellar = Self::require_config(&env, &Chain::Stellar);
        let vault = env.current_contract_address();
        if transfer.fee > 0 {
            let balance = token::Client::new(&env, &stellar.asset).balance(&vault);
            if balance < transfer.fee {
                panic_with_error!(&env, WalletError::InsufficientLiquidity);
            }
            token::Client::new(&env, &stellar.asset).transfer(
                &vault,
                Self::treasury(&env),
                &transfer.fee,
            );
        }

        transfer.status = TransferStatus::Settled;
        transfer.settled_at = env.ledger().timestamp();
        transfer.settled_by = relayer.clone();
        transfer.chain_tx = destination_tx;
        env.storage()
            .persistent()
            .set(&DataKey::Transfer(transfer_id), &transfer);
        Self::release_pending(&env, &transfer.dest_chain, transfer.net_amount);

        env.events().publish(
            (symbol_short!("xferSetl"), transfer.farmer),
            (transfer_id, transfer.fee, relayer),
        );
        transfer.fee
    }

    /// Return an unsettled outbound transfer to the farmer once its deadline
    /// has passed. Farmer only, and only for their own transfer.
    ///
    /// # Errors
    /// - `TransferNotFound` if the transfer does not exist.
    /// - `Unauthorized` if the caller is not the transfer's farmer.
    /// - `TransferNotPending` if the transfer already settled or was refunded.
    /// - `TransferNotExpired` if the deadline has not passed.
    pub fn refund_transfer(env: Env, transfer_id: u64) {
        Self::assert_not_paused(&env);
        let mut transfer = Self::require_transfer(&env, transfer_id);
        transfer.farmer.require_auth();
        if transfer.status != TransferStatus::Pending {
            panic_with_error!(&env, WalletError::TransferNotPending);
        }
        if env.ledger().timestamp() < transfer.expires_at {
            panic_with_error!(&env, WalletError::TransferNotExpired);
        }

        let stellar = Self::require_config(&env, &Chain::Stellar);
        let vault = env.current_contract_address();
        token::Client::new(&env, &stellar.asset).transfer(
            &vault,
            &transfer.farmer,
            &transfer.amount,
        );

        transfer.status = TransferStatus::Refunded;
        transfer.settled_at = env.ledger().timestamp();
        transfer.settled_by = transfer.farmer.clone();
        env.storage()
            .persistent()
            .set(&DataKey::Transfer(transfer_id), &transfer);
        Self::release_pending(&env, &transfer.dest_chain, transfer.net_amount);

        env.events().publish(
            (symbol_short!("xferRfnd"), transfer.farmer),
            (transfer_id, transfer.amount),
        );
    }

    // ── Transfer queries ──────────────────────────────────────────────────────

    /// Returns a transfer record, or `None`.
    #[must_use]
    pub fn get_transfer(env: Env, transfer_id: u64) -> Option<CreditTransfer> {
        env.storage()
            .persistent()
            .get(&DataKey::Transfer(transfer_id))
    }

    /// Returns a page of a farmer's transfers, oldest first.
    ///
    /// # Errors
    /// - `InvalidPageSize` if `limit` is zero or above `MAX_PAGE_SIZE`.
    /// - `InvalidCursor` if `cursor` is past the end of the history.
    pub fn get_wallet_transfers(
        env: Env,
        farmer: Address,
        cursor: u32,
        limit: u32,
    ) -> Vec<CreditTransfer> {
        if limit == 0 || limit > MAX_PAGE_SIZE {
            panic_with_error!(&env, WalletError::InvalidPageSize);
        }
        let ids: Vec<u64> = env
            .storage()
            .persistent()
            .get(&DataKey::TransferIds(farmer.clone()))
            .unwrap_or_else(|| Vec::new(&env));
        if cursor > ids.len() {
            panic_with_error!(&env, WalletError::InvalidCursor);
        }
        let mut page: Vec<CreditTransfer> = Vec::new(&env);
        let mut i = cursor;
        while i < ids.len() && page.len() < limit {
            if let Some(id) = ids.get(i) {
                if let Some(transfer) = env.storage().persistent().get(&DataKey::Transfer(id)) {
                    page.push_back(transfer);
                }
            }
            i += 1;
        }
        page
    }

    /// Total number of transfers opened across all wallets.
    #[must_use]
    pub fn total_transfer_count(env: Env) -> u64 {
        env.storage()
            .instance()
            .get(&DataKey::TransferSeq)
            .unwrap_or(0)
    }

    // ── Internal: authorization ───────────────────────────────────────────────

    fn require_admin(env: &Env) {
        let admin: Address = env
            .storage()
            .instance()
            .get(&DataKey::Admin)
            .unwrap_or_else(|| panic_with_error!(env, HarvestaError::NotInitialized));
        admin.require_auth();
    }

    fn assert_not_paused(env: &Env) {
        let paused: bool = env
            .storage()
            .instance()
            .get(&DataKey::Paused)
            .unwrap_or(false);
        if paused {
            panic_with_error!(env, HarvestaError::ContractPaused);
        }
    }

    /// Authorizes `relayer` and checks membership in the relayer set.
    fn require_relayer(env: &Env, relayer: &Address) {
        relayer.require_auth();
        let relayers = Self::relayers(env);
        if relayers.is_empty() {
            panic_with_error!(env, WalletError::RelayersNotSet);
        }
        if !relayers.iter().any(|a| a == *relayer) {
            panic_with_error!(env, WalletError::NotRelayer);
        }
    }

    // ── Internal: lookups ─────────────────────────────────────────────────────

    fn relayers(env: &Env) -> Vec<Address> {
        env.storage()
            .instance()
            .get(&DataKey::Relayers)
            .unwrap_or_else(|| Vec::new(env))
    }

    fn treasury(env: &Env) -> Address {
        env.storage()
            .instance()
            .get(&DataKey::Treasury)
            .unwrap_or_else(|| panic_with_error!(env, HarvestaError::NotInitialized))
    }

    fn chain_config(env: &Env, chain: &Chain) -> Option<ChainConfig> {
        env.storage().instance().get(&DataKey::ChainConfig(*chain))
    }

    /// Chain configuration, panicking when absent or administratively off.
    fn require_config(env: &Env, chain: &Chain) -> ChainConfig {
        match Self::routable_config(env, chain) {
            Some(config) => config,
            None => match Self::chain_config(env, chain) {
                Some(_) => panic_with_error!(env, WalletError::ChainNotEnabled),
                None => panic_with_error!(env, WalletError::ChainNotConfigured),
            },
        }
    }

    /// Configuration for a chain only when it can actually route.
    fn routable_config(env: &Env, chain: &Chain) -> Option<ChainConfig> {
        Self::chain_config(env, chain).filter(|config| config.enabled)
    }

    fn is_enabled(env: &Env, chain: &Chain) -> bool {
        Self::routable_config(env, chain).is_some()
    }

    fn require_wallet(env: &Env, farmer: &Address) -> WalletProfile {
        env.storage()
            .persistent()
            .get(&DataKey::Wallet(farmer.clone()))
            .unwrap_or_else(|| panic_with_error!(env, WalletError::WalletNotRegistered))
    }

    fn require_transfer(env: &Env, transfer_id: u64) -> CreditTransfer {
        env.storage()
            .persistent()
            .get(&DataKey::Transfer(transfer_id))
            .unwrap_or_else(|| panic_with_error!(env, WalletError::TransferNotFound))
    }

    /// Registered `0x…` address for `chain`.
    fn require_dest_address(env: &Env, farmer: &Address, chain: &Chain) -> ChainAddress {
        match env
            .storage()
            .persistent()
            .get(&DataKey::ChainAddress(farmer.clone(), *chain))
        {
            Some(ChainAddress::Evm(address)) => ChainAddress::Evm(address),
            _ => panic_with_error!(env, WalletError::AddressNotRegistered),
        }
    }

    /// The best route for the pair, or `NoRouteAvailable`.
    fn require_quote(env: &Env, source: Chain, dest: Chain, amount: i128) -> RouteQuote {
        Self::best_quote(env, source, dest, amount)
            .unwrap_or_else(|| panic_with_error!(env, WalletError::NoRouteAvailable))
    }

    fn check_deadline(env: &Env, deadline: u64) {
        let now = env.ledger().timestamp();
        if deadline < now || deadline > now.saturating_add(MAX_DEADLINE_SECONDS) {
            panic_with_error!(env, WalletError::InvalidDeadline);
        }
    }

    /// Marks a source/destination chain transaction hash as consumed.
    fn consume_tx(env: &Env, chain_tx: &BytesN<32>) {
        if env
            .storage()
            .persistent()
            .has(&DataKey::ConsumedTx(chain_tx.clone()))
        {
            panic_with_error!(env, WalletError::ReplayDetected);
        }
        env.storage()
            .persistent()
            .set(&DataKey::ConsumedTx(chain_tx.clone()), &true);
    }

    /// Destination-chain liquidity not already reserved by in-flight
    /// transfers, so a quote can never oversubscribe a chain.
    fn available(env: &Env, chain: &Chain) -> i128 {
        let config = match Self::chain_config(env, chain) {
            Some(config) => config,
            None => return 0,
        };
        let held = token::Client::new(env, &config.asset).balance(&env.current_contract_address());
        config
            .liquidity
            .min(held)
            .saturating_sub(Self::pending(env, chain))
    }

    /// Destination-chain liquidity reserved by pending transfers.
    fn pending(env: &Env, chain: &Chain) -> i128 {
        env.storage()
            .instance()
            .get(&DataKey::Pending(*chain))
            .unwrap_or(0)
    }

    fn reserve_pending(env: &Env, chain: &Chain, amount: i128) {
        let current = Self::pending(env, chain);
        env.storage()
            .instance()
            .set(&DataKey::Pending(*chain), &current.saturating_add(amount));
    }

    fn release_pending(env: &Env, chain: &Chain, amount: i128) {
        let current = Self::pending(env, chain);
        env.storage()
            .instance()
            .set(&DataKey::Pending(*chain), &current.saturating_sub(amount));
    }

    /// Number of the farmer's transfers still in `Pending`.
    fn open_transfer_count(env: &Env, farmer: &Address) -> u64 {
        let ids: Vec<u64> = env
            .storage()
            .persistent()
            .get(&DataKey::TransferIds(farmer.clone()))
            .unwrap_or_else(|| Vec::new(env));
        let mut count = 0u64;
        for id in ids.iter() {
            let pending: bool = env
                .storage()
                .persistent()
                .get(&DataKey::Transfer(id))
                .is_some_and(|transfer: CreditTransfer| transfer.status == TransferStatus::Pending);
            if pending {
                count += 1;
            }
        }
        count
    }

    // ── Internal: routing ─────────────────────────────────────────────────────

    /// Price a specific hop sequence, or `None` when it is not viable.
    fn score(env: &Env, hops: &Vec<RouteHop>, amount_in: i128) -> Option<RouteQuote> {
        if amount_in <= 0 || hops.is_empty() {
            return None;
        }
        let mut amount = amount_in;
        let mut total_fee: i128 = 0;
        let mut seconds: u64 = 0;
        for i in 0..hops.len() {
            let hop = hops.get(i)?;
            let from_config = Self::routable_config(env, &hop.from)?;
            let to_config = Self::routable_config(env, &hop.to)?;
            if amount < from_config.min_amount || amount > from_config.max_amount {
                return None;
            }
            // `amount <= HARD_MAX_AMOUNT` and `fee_bps <= MAX_FEE_BPS`, so the
            // basis-point product below cannot overflow.
            let bps_fee = (amount * i128::from(from_config.fee_bps)) / BPS_DENOM;
            let fee = bps_fee.saturating_add(from_config.fixed_fee);
            if fee >= amount {
                return None;
            }
            let next = amount - fee;
            if Self::available(env, &hop.to) < next {
                return None;
            }
            total_fee = total_fee.saturating_add(fee);
            seconds = seconds.saturating_add(to_config.settlement_seconds);
            amount = next;
        }
        if amount <= 0 {
            return None;
        }
        Some(RouteQuote {
            hops: hops.clone(),
            amount_in,
            amount_out: amount,
            total_fee,
            estimated_seconds: seconds,
            quoted_at: env.ledger().timestamp(),
        })
    }

    /// Score the direct hop and every two-hop alternative, returning the best.
    fn best_quote(env: &Env, source: Chain, dest: Chain, amount_in: i128) -> Option<RouteQuote> {
        if source == dest {
            return None;
        }
        let mut best: Option<RouteQuote> = None;

        let mut direct: Vec<RouteHop> = Vec::new(env);
        direct.push_back(RouteHop {
            from: source,
            to: dest,
        });
        if let Some(quote) = Self::score(env, &direct, amount_in) {
            best = Some(quote);
        }

        for mid in [Chain::Stellar, Chain::Polygon, Chain::Ethereum] {
            if mid == source || mid == dest {
                continue;
            }
            let mut hops: Vec<RouteHop> = Vec::new(env);
            hops.push_back(RouteHop {
                from: source,
                to: mid,
            });
            hops.push_back(RouteHop {
                from: mid,
                to: dest,
            });
            if let Some(quote) = Self::score(env, &hops, amount_in) {
                best = match best {
                    None => Some(quote),
                    Some(current) => {
                        if Self::is_better(&quote, &current) {
                            Some(quote)
                        } else {
                            Some(current)
                        }
                    }
                };
            }
        }
        best
    }

    /// Deterministic route preference: most delivered, then fastest, then
    /// fewest hops.
    fn is_better(candidate: &RouteQuote, current: &RouteQuote) -> bool {
        if candidate.amount_out != current.amount_out {
            return candidate.amount_out > current.amount_out;
        }
        if candidate.estimated_seconds != current.estimated_seconds {
            return candidate.estimated_seconds < current.estimated_seconds;
        }
        candidate.hops.len() < current.hops.len()
    }

    /// Persist a transfer, reserve destination liquidity for outbound legs,
    /// and update the farmer's profile and history. Returns the new ID.
    #[allow(clippy::too_many_arguments)]
    fn store_transfer(
        env: &Env,
        farmer: &Address,
        direction: Direction,
        source_chain: Chain,
        dest_chain: Chain,
        destination: ChainAddress,
        amount: i128,
        quote: &RouteQuote,
        expires_at: u64,
        settled_by: Address,
        chain_tx: &BytesN<32>,
    ) -> u64 {
        let now = env.ledger().timestamp();
        let id: u64 = env
            .storage()
            .instance()
            .get(&DataKey::TransferSeq)
            .unwrap_or(0)
            + 1;
        let inbound = direction == Direction::Inbound;
        let transfer = CreditTransfer {
            id,
            farmer: farmer.clone(),
            direction,
            source_chain,
            dest_chain,
            destination,
            amount,
            fee: quote.total_fee,
            net_amount: quote.amount_out,
            hops: quote.hop_count(),
            // An inbound leg is already attested by the relayer's `source_tx`,
            // so it settles immediately and has no refund window.
            status: if inbound {
                TransferStatus::Settled
            } else {
                TransferStatus::Pending
            },
            opened_at: now,
            expires_at,
            settled_at: if inbound { now } else { 0 },
            settled_by,
            chain_tx: chain_tx.clone(),
        };
        env.storage()
            .persistent()
            .set(&DataKey::Transfer(id), &transfer);
        env.storage().instance().set(&DataKey::TransferSeq, &id);
        if !inbound {
            Self::reserve_pending(env, &dest_chain, quote.amount_out);
        }

        let mut ids: Vec<u64> = env
            .storage()
            .persistent()
            .get(&DataKey::TransferIds(farmer.clone()))
            .unwrap_or_else(|| Vec::new(env));
        ids.push_back(id);
        env.storage()
            .persistent()
            .set(&DataKey::TransferIds(farmer.clone()), &ids);

        let existing: Option<WalletProfile> = env
            .storage()
            .persistent()
            .get(&DataKey::Wallet(farmer.clone()));
        if let Some(mut profile) = existing {
            profile.transfer_count += 1;
            profile.updated_at = now;
            env.storage()
                .persistent()
                .set(&DataKey::Wallet(farmer.clone()), &profile);
        }
        id
    }

    /// `0x` plus exactly 40 hex digits.
    fn is_valid_evm_address(address: &String) -> bool {
        if address.len() != EVM_ADDRESS_LEN {
            return false;
        }
        let mut raw = [0u8; EVM_ADDRESS_LEN as usize];
        address.copy_into_slice(&mut raw);
        if raw[0] != b'0' || (raw[1] != b'x' && raw[1] != b'X') {
            return false;
        }
        raw[2..].iter().all(|byte| Self::is_hex(*byte))
    }

    fn is_hex(byte: u8) -> bool {
        byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte) || (b'A'..=b'F').contains(&byte)
    }
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use soroban_sdk::testutils::{Address as _, Ledger};

    /// Basis points configured for the Stellar leg in the default setup.
    const STELLAR_BPS: u32 = 10;
    const STELLAR_FIXED: i128 = 1_000;
    /// Cheap Polygon lane — the natural routing intermediate.
    const POLYGON_BPS: u32 = 5;
    const POLYGON_FIXED: i128 = 100;
    /// Expensive Ethereum lane (10% ceiling) — the max-fee case.
    const ETHEREUM_BPS: u32 = 1_000;
    const ETHEREUM_FIXED: i128 = 0;

    /// Credit liquidity each chain advertises and holds in the vault.
    const CHAIN_LIQUIDITY: i128 = 1_000_000_000;
    /// Minimum routable amount leaving any chain.
    const CHAIN_MIN: i128 = 1_000;
    /// Credits minted to the farmer in setup.
    const FARMER_BALANCE: i128 = 10_000_000_000;
    /// Ledger timestamp used throughout.
    const NOW: u64 = 1_000;

    struct Ctx {
        env: Env,
        admin: Address,
        treasury: Address,
        farmer: Address,
        relayer: Address,
        /// SAC-wrapped credit token on Stellar — the escrowed asset.
        stellar_token: Address,
        polygon_token: Address,
        ethereum_token: Address,
        /// Contract address, i.e. the escrow vault.
        vault: Address,
        client: FarmerWalletClient<'static>,
    }

    impl Ctx {
        fn balance(&self, token: &Address, who: &Address) -> i128 {
            token::Client::new(&self.env, token).balance(who)
        }

        /// Vault balance of the Stellar credit token.
        fn vault_balance(&self) -> i128 {
            self.balance(&self.stellar_token, &self.vault)
        }

        fn farmer_balance(&self) -> i128 {
            self.balance(&self.stellar_token, &self.farmer)
        }

        fn treasury_balance(&self) -> i128 {
            self.balance(&self.stellar_token, &self.treasury)
        }

        fn tx(&self, seed: u8) -> BytesN<32> {
            BytesN::from_array(&self.env, &[seed; 32])
        }

        fn addr(&self, value: &str) -> String {
            String::from_str(&self.env, value)
        }

        fn mint_to(&self, token: &Address, who: &Address, amount: i128) {
            token::StellarAssetClient::new(&self.env, token).mint(who, &amount);
        }

        /// Registers the wallet and its Polygon destination address.
        fn with_farmer_registered(&self) {
            self.client.register_wallet(&self.farmer);
            self.client.set_chain_address(
                &self.farmer,
                &Chain::Polygon,
                &self.addr("0x1111111111111111111111111111111111111111"),
            );
        }
    }

    fn config(
        chain: Chain,
        asset: &Address,
        fee_bps: u32,
        fixed_fee: i128,
        liquidity: i128,
        settlement_seconds: u64,
    ) -> ChainConfig {
        ChainConfig {
            chain,
            enabled: true,
            fee_bps,
            fixed_fee,
            liquidity,
            asset: asset.clone(),
            min_amount: CHAIN_MIN,
            max_amount: HARD_MAX_AMOUNT,
            settlement_seconds,
            updated_at: 0,
        }
    }

    fn setup() -> Ctx {
        let env = Env::default();
        env.mock_all_auths();
        env.ledger().set_timestamp(NOW);

        let issuer = Address::generate(&env);
        let stellar_token = env
            .register_stellar_asset_contract_v2(issuer.clone())
            .address();
        let polygon_token = env
            .register_stellar_asset_contract_v2(issuer.clone())
            .address();
        let ethereum_token = env.register_stellar_asset_contract_v2(issuer).address();

        let vault = env.register(FarmerWallet, ());
        let client = FarmerWalletClient::new(&env, &vault);

        let admin = Address::generate(&env);
        let treasury = Address::generate(&env);
        let farmer = Address::generate(&env);
        let relayer = Address::generate(&env);

        client.initialize(&admin, &treasury);
        client.add_relayer(&relayer);

        // Liquidity providers pre-fund each chain's wrapped-credit supply.
        token::StellarAssetClient::new(&env, &polygon_token).mint(&vault, &CHAIN_LIQUIDITY);
        token::StellarAssetClient::new(&env, &ethereum_token).mint(&vault, &CHAIN_LIQUIDITY);
        token::StellarAssetClient::new(&env, &stellar_token).mint(&farmer, &FARMER_BALANCE);

        client.set_chain_config(&config(
            Chain::Stellar,
            &stellar_token,
            STELLAR_BPS,
            STELLAR_FIXED,
            CHAIN_LIQUIDITY,
            30,
        ));
        client.set_chain_config(&config(
            Chain::Polygon,
            &polygon_token,
            POLYGON_BPS,
            POLYGON_FIXED,
            CHAIN_LIQUIDITY,
            120,
        ));
        client.set_chain_config(&config(
            Chain::Ethereum,
            &ethereum_token,
            ETHEREUM_BPS,
            ETHEREUM_FIXED,
            CHAIN_LIQUIDITY,
            300,
        ));

        Ctx {
            env,
            admin,
            treasury,
            farmer,
            relayer,
            stellar_token,
            polygon_token,
            ethereum_token,
            vault,
            client,
        }
    }

    /// Same wiring, but with authorization left real so `require_auth` paths
    /// can be exercised.
    /// Fresh contract, admin is a real account with no signature behind it.
    fn setup_unmocked() -> (Env, Address, FarmerWalletClient<'static>) {
        let env = Env::default();
        env.ledger().set_timestamp(NOW);
        let vault = env.register(FarmerWallet, ());
        let client = FarmerWalletClient::new(&env, &vault);
        let admin = Address::generate(&env);
        (env, admin, client)
    }

    /// `setup_unmocked` with the contract already initialised. Auth is mocked
    /// for `initialize` only; `set_auths` switches mocking back off, so every
    /// later call must present a real signature.
    fn setup_initialized_unmocked() -> (Env, Address, Address, FarmerWalletClient<'static>) {
        let (env, admin, client) = setup_unmocked();
        env.mock_all_auths();
        let treasury = Address::generate(&env);
        client.initialize(&admin, &treasury);
        env.set_auths(&[]);
        (env, admin, treasury, client)
    }

    /// One outbound transfer from the default setup, plus the expected fee.
    ///
    /// `open` is Starlar's own fee: `amount * 10 / 10_000 + 1_000`.
    fn open_default(ctx: &Ctx, amount: i128) -> (u64, i128) {
        let expected_fee = (amount * i128::from(STELLAR_BPS)) / BPS_DENOM + STELLAR_FIXED;
        let id = ctx
            .client
            .open_transfer(&ctx.farmer, &Chain::Polygon, &amount, &1, &(NOW + 600));
        (id, expected_fee)
    }

    // ── initialize / chain configuration ─────────────────────────────────────

    #[test]
    fn test_initialize_sets_defaults() {
        let ctx = setup();
        assert!(!ctx.client.is_paused());
        assert_eq!(ctx.client.total_transfer_count(), 0);
        assert_eq!(ctx.client.get_treasury(), ctx.treasury);
        assert_eq!(ctx.client.get_relayers().len(), 1);
        assert!(ctx.client.is_chain_enabled(&Chain::Stellar));
        assert!(ctx.client.is_chain_enabled(&Chain::Polygon));
        assert!(ctx.client.is_chain_enabled(&Chain::Ethereum));
    }

    #[test]
    #[should_panic(expected = "Error(Contract, #1)")]
    fn test_double_initialize_rejected() {
        let ctx = setup();
        ctx.client.initialize(&ctx.admin, &ctx.treasury);
    }

    #[test]
    #[should_panic(expected = "Error(Contract, #531)")]
    fn test_initialize_rejects_contract_as_treasury() {
        let env = Env::default();
        env.mock_all_auths();
        let vault = env.register(FarmerWallet, ());
        let client = FarmerWalletClient::new(&env, &vault);
        let admin = Address::generate(&env);
        client.initialize(&admin, &vault);
    }

    #[test]
    fn test_set_chain_config_stamps_updated_at() {
        let ctx = setup();
        let mut cfg = ctx.client.get_chain_config(&Chain::Polygon).unwrap();
        assert_eq!(cfg.updated_at, NOW);
        ctx.env.ledger().set_timestamp(NOW + 42);
        cfg.liquidity = 77;
        ctx.client.set_chain_config(&cfg);
        let stored = ctx.client.get_chain_config(&Chain::Polygon).unwrap();
        assert_eq!(stored.liquidity, 77);
        assert_eq!(stored.updated_at, NOW + 42);
    }

    #[test]
    fn test_unconfigured_chain_reports_disabled() {
        let (env, _admin, client) = setup_unmocked();
        env.mock_all_auths();
        assert!(client.get_chain_config(&Chain::Polygon).is_none());
        assert!(!client.is_chain_enabled(&Chain::Polygon));
    }

    #[test]
    #[should_panic(expected = "Error(Contract, #502)")]
    fn test_fee_bps_above_ceiling_rejected() {
        let ctx = setup();
        let mut cfg = ctx.client.get_chain_config(&Chain::Polygon).unwrap();
        cfg.fee_bps = MAX_FEE_BPS + 1;
        ctx.client.set_chain_config(&cfg);
    }

    #[test]
    #[should_panic(expected = "Error(Contract, #504)")]
    fn test_negative_liquidity_rejected() {
        let ctx = setup();
        let mut cfg = ctx.client.get_chain_config(&Chain::Polygon).unwrap();
        cfg.liquidity = -1;
        ctx.client.set_chain_config(&cfg);
    }

    #[test]
    #[should_panic(expected = "Error(Contract, #503)")]
    fn test_amount_band_inverted_rejected() {
        let ctx = setup();
        let mut cfg = ctx.client.get_chain_config(&Chain::Polygon).unwrap();
        cfg.min_amount = 500;
        cfg.max_amount = 100;
        ctx.client.set_chain_config(&cfg);
    }

    #[test]
    #[should_panic(expected = "Error(Contract, #503)")]
    fn test_amount_band_above_hard_cap_rejected() {
        let ctx = setup();
        let mut cfg = ctx.client.get_chain_config(&Chain::Polygon).unwrap();
        cfg.max_amount = HARD_MAX_AMOUNT + 1;
        ctx.client.set_chain_config(&cfg);
    }

    #[test]
    #[should_panic(expected = "Error(Contract, #530)")]
    fn test_disable_chain_with_pending_transfers_panics() {
        let ctx = setup();
        ctx.with_farmer_registered();
        open_default(&ctx, 1_000_000);
        let mut cfg = ctx.client.get_chain_config(&Chain::Polygon).unwrap();
        cfg.enabled = false;
        ctx.client.set_chain_config(&cfg);
    }

    // ── relayer set ───────────────────────────────────────────────────────────

    #[test]
    fn test_add_relayer_grants_permission() {
        let ctx = setup();
        let second = Address::generate(&ctx.env);
        ctx.client.add_relayer(&second);
        assert!(ctx.client.is_relayer(&second));
        assert_eq!(ctx.client.get_relayers().len(), 2);
    }

    #[test]
    #[should_panic(expected = "Error(Contract, #525)")]
    fn test_duplicate_relayer_rejected() {
        let ctx = setup();
        ctx.client.add_relayer(&ctx.relayer);
    }

    #[test]
    #[should_panic(expected = "Error(Contract, #526)")]
    fn test_remove_unknown_relayer_rejected() {
        let ctx = setup();
        ctx.client.remove_relayer(&Address::generate(&ctx.env));
    }

    #[test]
    #[should_panic(expected = "Error(Contract, #527)")]
    fn test_remove_last_relayer_rejected() {
        let ctx = setup();
        ctx.client.remove_relayer(&ctx.relayer);
    }

    #[test]
    fn test_remove_one_of_multiple_relayers() {
        let ctx = setup();
        let second = Address::generate(&ctx.env);
        ctx.client.add_relayer(&second);
        ctx.client.remove_relayer(&ctx.relayer);
        assert!(!ctx.client.is_relayer(&ctx.relayer));
        assert!(ctx.client.is_relayer(&second));
    }

    /// The relayer set is never empty once a transfer can exist: the last
    /// relayer cannot be removed and `relay_inbound` refuses to open a transfer
    /// without one, so `RelayersNotSet` is a defence-in-depth branch rather than
    /// a reachable state.
    #[test]
    #[should_panic(expected = "Error(Contract, #524)")]
    fn test_relay_inbound_rejects_a_stranger() {
        let ctx = setup();
        ctx.with_farmer_registered();
        let stranger = Address::generate(&ctx.env);
        ctx.client.relay_inbound(
            &stranger,
            &ctx.farmer,
            &Chain::Polygon,
            &ctx.tx(1),
            &1_000_000,
            &1,
        );
    }

    // ── pause ─────────────────────────────────────────────────────────────────

    #[test]
    fn test_pause_blocks_writes_and_unpause_restores() {
        let ctx = setup();
        let other = Address::generate(&ctx.env);
        ctx.client.pause();
        assert!(ctx.client.is_paused());
        assert!(ctx.client.try_register_wallet(&other).is_err());

        ctx.client.unpause();
        assert!(!ctx.client.is_paused());
        assert_eq!(ctx.client.register_wallet(&other), 1);
    }

    // ── wallet registration ───────────────────────────────────────────────────

    #[test]
    fn test_register_wallet_returns_sequential_ids() {
        let ctx = setup();
        let second = Address::generate(&ctx.env);
        assert_eq!(ctx.client.register_wallet(&ctx.farmer), 1);
        assert_eq!(ctx.client.register_wallet(&second), 2);

        let profile = ctx.client.get_wallet(&ctx.farmer).unwrap();
        assert_eq!(profile.farmer, ctx.farmer);
        assert_eq!(profile.wallet_id, 1);
        assert_eq!(profile.created_at, NOW);
        assert!(profile.active);
        assert_eq!(profile.transfer_count, 0);
    }

    #[test]
    fn test_register_wallet_seeds_stellar_chain() {
        let ctx = setup();
        ctx.client.register_wallet(&ctx.farmer);
        let chains = ctx.client.get_wallet_chains(&ctx.farmer);
        assert_eq!(chains.len(), 1);
        assert_eq!(chains.get(0), Some(Chain::Stellar));
    }

    #[test]
    #[should_panic(expected = "Error(Contract, #512)")]
    fn test_double_registration_rejected() {
        let ctx = setup();
        ctx.client.register_wallet(&ctx.farmer);
        ctx.client.register_wallet(&ctx.farmer);
    }

    #[test]
    #[should_panic(expected = "Error(Contract, #511)")]
    fn test_set_chain_address_without_wallet_rejected() {
        let ctx = setup();
        ctx.client.set_chain_address(
            &ctx.farmer,
            &Chain::Polygon,
            &ctx.addr("0x1111111111111111111111111111111111111111"),
        );
    }

    #[test]
    #[should_panic(expected = "Error(Contract, #511)")]
    fn test_open_transfer_without_wallet_rejected() {
        let ctx = setup();
        ctx.client
            .open_transfer(&ctx.farmer, &Chain::Polygon, &1_000_000, &1, &(NOW + 600));
    }

    #[test]
    fn test_freeze_wallet_blocks_transfers_and_blocks_reuse() {
        let ctx = setup();
        ctx.with_farmer_registered();
        ctx.client.set_wallet_active(&ctx.farmer, &false);
        let profile = ctx.client.get_wallet(&ctx.farmer).unwrap();
        assert!(!profile.active);

        let result = ctx.client.try_open_transfer(
            &ctx.farmer,
            &Chain::Polygon,
            &1_000_000,
            &1,
            &(NOW + 600),
        );
        assert!(result.is_err());

        ctx.client.set_wallet_active(&ctx.farmer, &true);
        assert!(ctx.client.get_wallet(&ctx.farmer).unwrap().active);
    }

    #[test]
    #[should_panic(expected = "Error(Contract, #529)")]
    fn test_freeze_blocked_while_transfers_pending() {
        let ctx = setup();
        ctx.with_farmer_registered();
        open_default(&ctx, 1_000_000);
        ctx.client.set_wallet_active(&ctx.farmer, &false);
    }

    // ── chain addresses ───────────────────────────────────────────────────────

    #[test]
    fn test_set_and_get_chain_address() {
        let ctx = setup();
        ctx.client.register_wallet(&ctx.farmer);
        let polygon = ctx.addr("0xAbC0000000000000000000000000000000000001");
        ctx.client
            .set_chain_address(&ctx.farmer, &Chain::Polygon, &polygon);

        assert_eq!(
            ctx.client.get_chain_address(&ctx.farmer, &Chain::Polygon),
            Some(ChainAddress::Evm(polygon.clone()))
        );
        let chains = ctx.client.get_wallet_chains(&ctx.farmer);
        assert_eq!(chains.len(), 2);
        assert_eq!(chains.get(1), Some(Chain::Polygon));
    }

    #[test]
    fn test_stellar_address_is_implicit() {
        let ctx = setup();
        ctx.client.register_wallet(&ctx.farmer);
        assert_eq!(
            ctx.client.get_chain_address(&ctx.farmer, &Chain::Stellar),
            Some(ChainAddress::Native(ctx.farmer.clone()))
        );
    }

    #[test]
    #[should_panic(expected = "Error(Contract, #516)")]
    fn test_setting_stellar_address_rejected() {
        let ctx = setup();
        ctx.client.register_wallet(&ctx.farmer);
        ctx.client.set_chain_address(
            &ctx.farmer,
            &Chain::Stellar,
            &ctx.addr("0x1111111111111111111111111111111111111111"),
        );
    }

    #[test]
    fn test_evm_address_validation() {
        let ctx = setup();
        ctx.client.register_wallet(&ctx.farmer);
        // Accepted: lowercase, uppercase hex, and an `0X` prefix.
        for valid in [
            "0x1111111111111111111111111111111111111111",
            "0xABCDEFabcdefABCDEFabcdefABCDEFabcdefABCD",
            "0X1111111111111111111111111111111111111111",
        ] {
            ctx.client
                .set_chain_address(&ctx.farmer, &Chain::Polygon, &ctx.addr(valid));
        }
        // Rejected: wrong length, missing prefix, non-hex characters.
        for invalid in [
            "0x123",
            "1111111111111111111111111111111111111111",
            "0x111111111111111111111111111111111111111",
            "0xZZ1111111111111111111111111111111111111111",
            "0x 111111111111111111111111111111111111111",
        ] {
            assert!(ctx
                .client
                .try_set_chain_address(&ctx.farmer, &Chain::Polygon, &ctx.addr(invalid))
                .is_err());
        }
    }

    #[test]
    fn test_update_chain_address_does_not_duplicate_chain() {
        let ctx = setup();
        ctx.client.register_wallet(&ctx.farmer);
        ctx.client.set_chain_address(
            &ctx.farmer,
            &Chain::Polygon,
            &ctx.addr("0x1111111111111111111111111111111111111111"),
        );
        ctx.client.set_chain_address(
            &ctx.farmer,
            &Chain::Polygon,
            &ctx.addr("0x2222222222222222222222222222222222222222"),
        );
        assert_eq!(ctx.client.get_wallet_chains(&ctx.farmer).len(), 2);
        assert_eq!(
            ctx.client.get_chain_address(&ctx.farmer, &Chain::Polygon),
            Some(ChainAddress::Evm(
                ctx.addr("0x2222222222222222222222222222222222222222")
            ))
        );
    }

    #[test]
    #[should_panic(expected = "Error(Contract, #500)")]
    fn test_set_chain_address_on_disabled_chain_rejected() {
        let ctx = setup();
        ctx.client.register_wallet(&ctx.farmer);
        let mut cfg = ctx.client.get_chain_config(&Chain::Polygon).unwrap();
        cfg.enabled = false;
        ctx.client.set_chain_config(&cfg);
        ctx.client.set_chain_address(
            &ctx.farmer,
            &Chain::Polygon,
            &ctx.addr("0x1111111111111111111111111111111111111111"),
        );
    }

    #[test]
    #[should_panic(expected = "Error(Contract, #529)")]
    fn test_address_change_blocked_while_transfers_pending() {
        let ctx = setup();
        ctx.with_farmer_registered();
        open_default(&ctx, 1_000_000);
        ctx.client.set_chain_address(
            &ctx.farmer,
            &Chain::Polygon,
            &ctx.addr("0x2222222222222222222222222222222222222222"),
        );
    }

    // ── routing ───────────────────────────────────────────────────────────────

    #[test]
    fn test_direct_route_is_cheapest_when_every_lane_is_liquid() {
        let ctx = setup();
        let amount = 1_000_000;
        let quote = ctx
            .client
            .get_best_route(&Chain::Stellar, &Chain::Polygon, &amount)
            .unwrap();
        // A hop is priced by the chain it leaves, so a Stellar -> Polygon leg
        // pays Stellar's fee: 1_000_000 * 10 / 10_000 + 1_000.
        assert_eq!(quote.amount_out, amount - 2_000);
        assert_eq!(quote.total_fee, 2_000);
        assert_eq!(quote.hops.len(), 1);
        assert_eq!(quote.amount_in, amount);
        assert_eq!(quote.quoted_at, NOW);
        assert_eq!(quote.hop_count(), 1);
    }

    /// The destination's own fee never applies to a leg arriving from outside:
    /// only the leg leaving a chain is charged, so a direct lane out of Stellar
    /// costs the same to either EVM and the destination's `fee_bps` only shapes
    /// transfers that later leave that chain.
    #[test]
    fn test_hop_fee_follows_the_chain_the_hop_leaves() {
        let ctx = setup();
        // Nothing can land on Stellar until the vault holds Stellar credit.
        fund_vault(&ctx, CHAIN_LIQUIDITY);
        let amount = 1_000_000;
        let to_polygon = ctx
            .client
            .get_best_route(&Chain::Stellar, &Chain::Polygon, &amount)
            .unwrap();
        let to_ethereum = ctx
            .client
            .get_best_route(&Chain::Stellar, &Chain::Ethereum, &amount)
            .unwrap();
        assert_eq!(to_polygon.amount_out, amount - 2_000);
        assert_eq!(to_ethereum.amount_out, amount - 2_000);
        assert_eq!(to_polygon.hops.len(), 1);
        assert_eq!(to_ethereum.hops.len(), 1);

        // Out of Ethereum the fee is Ethereum's 1000 bps, with no fixed cost.
        let from_ethereum = ctx
            .client
            .get_best_route(&Chain::Ethereum, &Chain::Stellar, &amount)
            .unwrap();
        assert_eq!(from_ethereum.amount_out, amount - 100_000);
        assert_eq!(from_ethereum.total_fee, 100_000);
    }

    /// Amounts only shrink along a route and every hop fee is non-negative, so
    /// a multi-hop path can never out-price a viable direct lane. Its job is
    /// to keep a transfer possible when the direct lane is over-subscribed.
    #[test]
    fn test_multihop_routes_around_a_liquidity_shortfall() {
        let ctx = setup();
        // Squeeze Ethereum into the window where only the Polygon detour fits.
        // Direct: 1_000_000 -> 998_000 (Stellar fee). Detour: 1_000_000 ->
        // 998_000 (Stellar) -> 997_401 (Polygon fee). Pinning available
        // liquidity at 997_700 rules out the direct lane and admits the detour.
        let mut cfg = ctx.client.get_chain_config(&Chain::Ethereum).unwrap();
        cfg.liquidity = 997_700;
        ctx.client.set_chain_config(&cfg);

        let amount = 1_000_000;
        let quote = ctx
            .client
            .get_best_route(&Chain::Stellar, &Chain::Ethereum, &amount)
            .unwrap();
        assert_eq!(quote.hops.len(), 2);
        assert_eq!(quote.hops.get(0).unwrap().to, Chain::Polygon);
        assert_eq!(quote.hops.get(1).unwrap().to, Chain::Ethereum);
        // 1_000_000 -> 998_000 (Stellar) -> 997_401 (Polygon fee).
        assert_eq!(quote.amount_out, 997_401);
        assert_eq!(quote.total_fee, 2_599);
    }

    #[test]
    fn test_no_route_when_destination_liquidity_is_exhausted() {
        let ctx = setup();
        let mut cfg = ctx.client.get_chain_config(&Chain::Ethereum).unwrap();
        cfg.liquidity = 1;
        ctx.client.set_chain_config(&cfg);
        let amount = 1_000_000;
        assert!(ctx
            .client
            .get_best_route(&Chain::Stellar, &Chain::Ethereum, &amount)
            .is_none());
        assert_eq!(
            ctx.client.max_transfer(&Chain::Stellar, &Chain::Ethereum),
            0
        );
    }

    #[test]
    fn test_disabled_destination_has_no_route() {
        let ctx = setup();
        let mut ethereum = ctx.client.get_chain_config(&Chain::Ethereum).unwrap();
        ethereum.enabled = false;
        ctx.client.set_chain_config(&ethereum);

        let amount = 1_000_000;
        let quote = ctx
            .client
            .get_best_route(&Chain::Stellar, &Chain::Ethereum, &amount);
        // A disabled chain is not routable at all, even as an intermediate.
        assert!(quote.is_none());
        assert_eq!(
            ctx.client.max_transfer(&Chain::Stellar, &Chain::Ethereum),
            0
        );
    }

    #[test]
    fn test_settlement_time_breaks_a_fee_tie() {
        let ctx = setup();
        // Zero fees everywhere: both candidates deliver the full amount, so
        // the faster lane wins and the single hop is preferred.
        for chain in [Chain::Stellar, Chain::Polygon, Chain::Ethereum] {
            let mut cfg = ctx.client.get_chain_config(&chain).unwrap();
            cfg.fee_bps = 0;
            cfg.fixed_fee = 0;
            ctx.client.set_chain_config(&cfg);
        }
        let amount = 1_000_000;
        let quote = ctx
            .client
            .get_best_route(&Chain::Stellar, &Chain::Ethereum, &amount)
            .unwrap();
        assert_eq!(quote.amount_out, amount);
        assert_eq!(quote.total_fee, 0);
        // Direct: 300s. Via Polygon: 120s + 300s.
        assert_eq!(quote.estimated_seconds, 300);
        assert_eq!(quote.hops.len(), 1);
    }

    #[test]
    fn test_route_rejects_same_chain_and_non_positive_amounts() {
        let ctx = setup();
        assert!(ctx
            .client
            .get_best_route(&Chain::Stellar, &Chain::Stellar, &1_000_000)
            .is_none());
        assert!(ctx
            .client
            .get_best_route(&Chain::Stellar, &Chain::Polygon, &0)
            .is_none());
        assert!(ctx
            .client
            .get_best_route(&Chain::Stellar, &Chain::Polygon, &-5)
            .is_none());
    }

    /// The headline number is gross, so it may exceed the destination's net
    /// liquidity: the fee is what the lane takes on the way out. What must
    /// always hold is that the quoted amount at that ceiling is still routable
    /// and never over-commits the destination.
    #[test]
    fn test_max_transfer_tracks_destination_liquidity_and_fees() {
        let ctx = setup();
        let max = ctx.client.max_transfer(&Chain::Stellar, &Chain::Polygon);
        assert_eq!(ctx.client.max_transfer(&Chain::Stellar, &Chain::Stellar), 0);
        // 1e9 of liquidity plus the 10 bps + 1_000 fee it can absorb.
        assert_eq!(max, 1_001_002_001);
        let quote = ctx
            .client
            .get_best_route(&Chain::Stellar, &Chain::Polygon, &max)
            .unwrap();
        assert_eq!(quote.amount_in, max);
        assert_eq!(quote.amount_out, 999_999_999);
        assert!(quote.amount_out <= CHAIN_LIQUIDITY);
    }

    /// A thin destination lane must not report "nothing is routable": the
    /// ceiling has to fall back to a smaller amount that still settles.
    #[test]
    fn test_max_transfer_survives_thin_destination_liquidity() {
        let ctx = setup();
        let mut cfg = ctx.client.get_chain_config(&Chain::Polygon).unwrap();
        cfg.liquidity = 1_000;
        ctx.client.set_chain_config(&cfg);

        let max = ctx.client.max_transfer(&Chain::Stellar, &Chain::Polygon);
        assert!(max > 0, "a routable lane must not report zero");
        let quote = ctx
            .client
            .get_best_route(&Chain::Stellar, &Chain::Polygon, &max)
            .unwrap();
        assert!(
            quote.amount_out <= 1_000,
            "must respect available liquidity"
        );
    }

    /// Nothing is routable once the destination is truly empty.
    #[test]
    fn test_max_transfer_zero_without_destination_liquidity() {
        let ctx = setup();
        let mut cfg = ctx.client.get_chain_config(&Chain::Polygon).unwrap();
        cfg.liquidity = 0;
        ctx.client.set_chain_config(&cfg);
        assert_eq!(ctx.client.max_transfer(&Chain::Stellar, &Chain::Polygon), 0);
    }

    #[test]
    fn test_max_transfer_respects_max_amount_band() {
        let ctx = setup();
        let mut stellar = ctx.client.get_chain_config(&Chain::Stellar).unwrap();
        stellar.max_amount = 2_000_000;
        ctx.client.set_chain_config(&stellar);
        assert_eq!(
            ctx.client.max_transfer(&Chain::Stellar, &Chain::Polygon),
            2_000_000
        );
    }

    #[test]
    fn test_max_transfer_zero_for_unconfigured_chain() {
        let ctx = setup();
        let mut cfg = ctx.client.get_chain_config(&Chain::Polygon).unwrap();
        cfg.enabled = false;
        ctx.client.set_chain_config(&cfg);
        assert_eq!(ctx.client.max_transfer(&Chain::Stellar, &Chain::Polygon), 0);
    }

    // ── outbound transfers ────────────────────────────────────────────────────

    #[test]
    fn test_open_transfer_escrows_gross_and_records_route() {
        let ctx = setup();
        ctx.with_farmer_registered();
        let amount = 1_000_000;
        let (id, fee) = open_default(&ctx, amount);

        assert_eq!(id, 1);
        assert_eq!(ctx.farmer_balance(), FARMER_BALANCE - amount);
        assert_eq!(ctx.vault_balance(), amount);
        assert_eq!(ctx.treasury_balance(), 0);

        let transfer = ctx.client.get_transfer(&id).unwrap();
        assert_eq!(transfer.farmer, ctx.farmer);
        assert_eq!(transfer.direction, Direction::Outbound);
        assert_eq!(transfer.source_chain, Chain::Stellar);
        assert_eq!(transfer.dest_chain, Chain::Polygon);
        assert_eq!(
            transfer.destination,
            ChainAddress::Evm(ctx.addr("0x1111111111111111111111111111111111111111"))
        );
        assert_eq!(transfer.amount, amount);
        assert_eq!(transfer.fee, fee);
        assert_eq!(transfer.net_amount, amount - fee);
        assert_eq!(transfer.hops, 1);
        assert_eq!(transfer.status, TransferStatus::Pending);
        assert_eq!(transfer.opened_at, NOW);
        assert_eq!(transfer.expires_at, NOW + 600);
        assert_eq!(transfer.settled_at, 0);
        assert_eq!(ctx.client.total_transfer_count(), 1);
        assert_eq!(
            ctx.client.get_wallet(&ctx.farmer).unwrap().transfer_count,
            1
        );
    }

    #[test]
    fn test_settle_transfer_releases_fee_to_treasury() {
        let ctx = setup();
        ctx.with_farmer_registered();
        let (id, fee) = open_default(&ctx, 1_000_000);

        let collected = ctx.client.settle_transfer(&ctx.relayer, &id, &ctx.tx(7));
        assert_eq!(collected, fee);
        assert_eq!(ctx.treasury_balance(), fee);
        // The net stays behind as destination-chain backing.
        assert_eq!(ctx.vault_balance(), 1_000_000 - fee);

        let transfer = ctx.client.get_transfer(&id).unwrap();
        assert_eq!(transfer.status, TransferStatus::Settled);
        assert_eq!(transfer.settled_at, NOW);
        assert_eq!(transfer.settled_by, ctx.relayer);
        assert_eq!(transfer.chain_tx, ctx.tx(7));
    }

    #[test]
    fn test_settle_requires_relayer() {
        let ctx = setup();
        ctx.with_farmer_registered();
        let (id, _) = open_default(&ctx, 1_000_000);
        let stranger = Address::generate(&ctx.env);
        assert!(ctx
            .client
            .try_settle_transfer(&stranger, &id, &ctx.tx(7))
            .is_err());
    }

    #[test]
    #[should_panic(expected = "Error(Contract, #518)")]
    fn test_settle_unknown_transfer_rejected() {
        let ctx = setup();
        ctx.client.settle_transfer(&ctx.relayer, &404, &ctx.tx(7));
    }

    #[test]
    #[should_panic(expected = "Error(Contract, #519)")]
    fn test_double_settle_rejected() {
        let ctx = setup();
        ctx.with_farmer_registered();
        let (id, _) = open_default(&ctx, 1_000_000);
        ctx.client.settle_transfer(&ctx.relayer, &id, &ctx.tx(7));
        ctx.client.settle_transfer(&ctx.relayer, &id, &ctx.tx(8));
    }

    #[test]
    #[should_panic(expected = "Error(Contract, #522)")]
    fn test_settle_replayed_destination_tx_rejected() {
        let ctx = setup();
        ctx.with_farmer_registered();
        let (first, _) = open_default(&ctx, 1_000_000);
        let (second, _) = open_default(&ctx, 1_000_000);
        ctx.client.settle_transfer(&ctx.relayer, &first, &ctx.tx(7));
        // A second, distinct transfer cannot reuse the same destination hash.
        ctx.client
            .settle_transfer(&ctx.relayer, &second, &ctx.tx(7));
    }

    #[test]
    #[should_panic(expected = "Error(Contract, #520)")]
    fn test_settle_after_deadline_rejected() {
        let ctx = setup();
        ctx.with_farmer_registered();
        let (id, _) = open_default(&ctx, 1_000_000);
        ctx.env.ledger().set_timestamp(NOW + 600);
        ctx.client.settle_transfer(&ctx.relayer, &id, &ctx.tx(7));
    }

    #[test]
    fn test_refund_returns_gross_after_deadline() {
        let ctx = setup();
        ctx.with_farmer_registered();
        let amount = 1_000_000;
        let (id, _) = open_default(&ctx, amount);
        assert_eq!(ctx.farmer_balance(), FARMER_BALANCE - amount);

        ctx.env.ledger().set_timestamp(NOW + 600);
        ctx.client.refund_transfer(&id);
        assert_eq!(ctx.farmer_balance(), FARMER_BALANCE);
        assert_eq!(ctx.vault_balance(), 0);
        assert_eq!(ctx.treasury_balance(), 0);

        let transfer = ctx.client.get_transfer(&id).unwrap();
        assert_eq!(transfer.status, TransferStatus::Refunded);
        assert_eq!(transfer.settled_at, NOW + 600);
        assert_eq!(transfer.settled_by, ctx.farmer);
    }

    #[test]
    #[should_panic(expected = "Error(Contract, #521)")]
    fn test_refund_before_deadline_rejected() {
        let ctx = setup();
        ctx.with_farmer_registered();
        let (id, _) = open_default(&ctx, 1_000_000);
        ctx.env.ledger().set_timestamp(NOW + 599);
        ctx.client.refund_transfer(&id);
    }

    #[test]
    #[should_panic(expected = "Error(Contract, #519)")]
    fn test_double_refund_rejected() {
        let ctx = setup();
        ctx.with_farmer_registered();
        let (id, _) = open_default(&ctx, 1_000_000);
        ctx.env.ledger().set_timestamp(NOW + 600);
        ctx.client.refund_transfer(&id);
        ctx.client.refund_transfer(&id);
    }

    #[test]
    #[should_panic(expected = "Error(Contract, #519)")]
    fn test_refund_after_settle_rejected() {
        let ctx = setup();
        ctx.with_farmer_registered();
        let (id, _) = open_default(&ctx, 1_000_000);
        ctx.client.settle_transfer(&ctx.relayer, &id, &ctx.tx(7));
        ctx.env.ledger().set_timestamp(NOW + 600);
        ctx.client.refund_transfer(&id);
    }

    #[test]
    fn test_settle_and_refund_are_mutually_exclusive() {
        let ctx = setup();
        ctx.with_farmer_registered();
        let (id, _) = open_default(&ctx, 1_000_000);
        ctx.client.settle_transfer(&ctx.relayer, &id, &ctx.tx(7));
        ctx.env.ledger().set_timestamp(NOW + 600);
        assert!(ctx.client.try_refund_transfer(&id).is_err());
        assert_eq!(
            ctx.client.get_transfer(&id).unwrap().status,
            TransferStatus::Settled
        );
    }

    #[test]
    #[should_panic(expected = "Error(Contract, #517)")]
    fn test_open_to_stellar_rejected() {
        let ctx = setup();
        ctx.with_farmer_registered();
        ctx.client
            .open_transfer(&ctx.farmer, &Chain::Stellar, &1_000_000, &1, &(NOW + 600));
    }

    #[test]
    #[should_panic(expected = "Error(Contract, #514)")]
    fn test_open_to_unregistered_chain_rejected() {
        let ctx = setup();
        ctx.client.register_wallet(&ctx.farmer);
        ctx.client
            .open_transfer(&ctx.farmer, &Chain::Polygon, &1_000_000, &1, &(NOW + 600));
    }

    #[test]
    #[should_panic(expected = "Error(Contract, #9)")]
    fn test_open_non_positive_amount_rejected() {
        let ctx = setup();
        ctx.with_farmer_registered();
        ctx.client
            .open_transfer(&ctx.farmer, &Chain::Polygon, &0, &1, &(NOW + 600));
    }

    #[test]
    #[should_panic(expected = "Error(Contract, #505)")]
    fn test_open_below_min_amount_rejected() {
        let ctx = setup();
        ctx.with_farmer_registered();
        ctx.client.open_transfer(
            &ctx.farmer,
            &Chain::Polygon,
            &(CHAIN_MIN - 1),
            &1,
            &(NOW + 600),
        );
    }

    #[test]
    #[should_panic(expected = "Error(Contract, #506)")]
    fn test_open_above_max_amount_rejected() {
        let ctx = setup();
        ctx.with_farmer_registered();
        ctx.client.open_transfer(
            &ctx.farmer,
            &Chain::Polygon,
            &(HARD_MAX_AMOUNT + 1),
            &1,
            &(NOW + 600),
        );
    }

    #[test]
    #[should_panic(expected = "Error(Contract, #510)")]
    fn test_min_amount_out_zero_rejected() {
        let ctx = setup();
        ctx.with_farmer_registered();
        ctx.client
            .open_transfer(&ctx.farmer, &Chain::Polygon, &1_000_000, &0, &(NOW + 600));
    }

    #[test]
    #[should_panic(expected = "Error(Contract, #510)")]
    fn test_min_amount_out_above_amount_rejected() {
        let ctx = setup();
        ctx.with_farmer_registered();
        ctx.client.open_transfer(
            &ctx.farmer,
            &Chain::Polygon,
            &1_000_000,
            &1_000_001,
            &(NOW + 600),
        );
    }

    #[test]
    #[should_panic(expected = "Error(Contract, #508)")]
    fn test_deadline_in_the_past_rejected() {
        let ctx = setup();
        ctx.with_farmer_registered();
        ctx.client
            .open_transfer(&ctx.farmer, &Chain::Polygon, &1_000_000, &1, &(NOW - 1));
    }

    #[test]
    #[should_panic(expected = "Error(Contract, #508)")]
    fn test_deadline_beyond_max_window_rejected() {
        let ctx = setup();
        ctx.with_farmer_registered();
        ctx.client.open_transfer(
            &ctx.farmer,
            &Chain::Polygon,
            &1_000_000,
            &1,
            &(NOW + MAX_DEADLINE_SECONDS + 1),
        );
    }

    #[test]
    fn test_slippage_bound_is_enforced() {
        let ctx = setup();
        ctx.with_farmer_registered();
        let amount = 1_000_000;
        // Fee is 600, so demanding the full amount back must revert.
        assert!(ctx
            .client
            .try_open_transfer(&ctx.farmer, &Chain::Polygon, &amount, &amount, &(NOW + 600))
            .is_err());
        // Exactly the quoted net is accepted.
        assert_eq!(open_default(&ctx, amount).0, 1);
    }

    #[test]
    fn test_fee_is_locked_at_open_time() {
        let ctx = setup();
        ctx.with_farmer_registered();
        let (id, fee) = open_default(&ctx, 1_000_000);

        // Reprice the Stellar lane after the transfer opened.
        let mut stellar = ctx.client.get_chain_config(&Chain::Stellar).unwrap();
        stellar.fee_bps = 500;
        stellar.fixed_fee = 50_000;
        ctx.client.set_chain_config(&stellar);

        let collected = ctx.client.settle_transfer(&ctx.relayer, &id, &ctx.tx(7));
        assert_eq!(collected, fee);
        assert_eq!(ctx.client.get_transfer(&id).unwrap().fee, fee);
    }

    // ── inbound transfers ─────────────────────────────────────────────────────

    /// Prefunds the vault's Stellar side so an inbound leg can pay out.
    fn fund_vault(ctx: &Ctx, amount: i128) {
        ctx.mint_to(&ctx.stellar_token, &ctx.vault, amount);
    }

    /// Each chain's wrapped credit is its own reserve. Settling an outbound to
    /// Polygon releases only the fee out of the Stellar escrow and must not
    /// touch either EVM reserve: the net stays escrowed as the backing against
    /// which the relayer mints on Polygon.
    #[test]
    fn test_settlement_releases_only_the_fee() {
        let ctx = setup();
        ctx.with_farmer_registered();
        let amount = 1_000_000;
        let quote = ctx
            .client
            .get_best_route(&Chain::Stellar, &Chain::Polygon, &amount)
            .unwrap();
        let id = ctx.client.open_transfer(
            &ctx.farmer,
            &Chain::Polygon,
            &amount,
            &quote.amount_out,
            &(NOW + 600),
        );
        assert_eq!(ctx.vault_balance(), amount);
        assert_eq!(ctx.balance(&ctx.polygon_token, &ctx.vault), CHAIN_LIQUIDITY);
        assert_eq!(
            ctx.balance(&ctx.ethereum_token, &ctx.vault),
            CHAIN_LIQUIDITY
        );

        ctx.client.settle_transfer(&ctx.relayer, &id, &ctx.tx(3));

        assert_eq!(ctx.vault_balance(), amount - quote.total_fee);
        assert_eq!(ctx.treasury_balance(), quote.total_fee);
        assert_eq!(
            ctx.balance(&ctx.polygon_token, &ctx.vault),
            CHAIN_LIQUIDITY,
            "an untouched chain's reserve must not move"
        );
        assert_eq!(
            ctx.balance(&ctx.ethereum_token, &ctx.vault),
            CHAIN_LIQUIDITY
        );
    }

    #[test]
    fn test_relay_inbound_pays_out_and_settles_immediately() {
        let ctx = setup();
        ctx.client.register_wallet(&ctx.farmer);
        fund_vault(&ctx, 5_000_000);

        let amount = 1_000_000;
        let source_tx = ctx.tx(11);
        let before = ctx.farmer_balance();
        let id = ctx.client.relay_inbound(
            &ctx.relayer,
            &ctx.farmer,
            &Chain::Polygon,
            &source_tx,
            &amount,
            &1,
        );

        // 1_000_000 * 5 / 10_000 + 100 = 600.
        let expected_net = amount - 600;
        assert_eq!(ctx.farmer_balance(), before + expected_net);
        assert_eq!(ctx.vault_balance(), 5_000_000 - expected_net);

        let transfer = ctx.client.get_transfer(&id).unwrap();
        assert_eq!(transfer.direction, Direction::Inbound);
        assert_eq!(transfer.source_chain, Chain::Polygon);
        assert_eq!(transfer.dest_chain, Chain::Stellar);
        assert_eq!(
            transfer.destination,
            ChainAddress::Native(ctx.farmer.clone())
        );
        assert_eq!(transfer.fee, 600);
        assert_eq!(transfer.net_amount, expected_net);
        assert_eq!(transfer.status, TransferStatus::Settled);
        assert_eq!(transfer.settled_at, NOW);
        assert_eq!(transfer.settled_by, ctx.relayer);
        assert_eq!(transfer.chain_tx, source_tx);
        assert_eq!(transfer.expires_at, 0);
    }

    #[test]
    fn test_relay_inbound_requires_relayer() {
        let ctx = setup();
        ctx.client.register_wallet(&ctx.farmer);
        fund_vault(&ctx, 5_000_000);
        let stranger = Address::generate(&ctx.env);
        assert!(ctx
            .client
            .try_relay_inbound(
                &stranger,
                &ctx.farmer,
                &Chain::Polygon,
                &ctx.tx(11),
                &1_000_000,
                &1
            )
            .is_err());
    }

    #[test]
    #[should_panic(expected = "Error(Contract, #522)")]
    fn test_relay_inbound_replay_rejected() {
        let ctx = setup();
        ctx.client.register_wallet(&ctx.farmer);
        fund_vault(&ctx, 5_000_000);
        let source_tx = ctx.tx(11);
        ctx.client.relay_inbound(
            &ctx.relayer,
            &ctx.farmer,
            &Chain::Polygon,
            &source_tx,
            &1_000_000,
            &1,
        );
        fund_vault(&ctx, 5_000_000);
        ctx.client.relay_inbound(
            &ctx.relayer,
            &ctx.farmer,
            &Chain::Polygon,
            &source_tx,
            &1_000_000,
            &1,
        );
    }

    #[test]
    #[should_panic(expected = "Error(Contract, #517)")]
    fn test_relay_inbound_from_stellar_rejected() {
        let ctx = setup();
        ctx.client.register_wallet(&ctx.farmer);
        fund_vault(&ctx, 5_000_000);
        ctx.client.relay_inbound(
            &ctx.relayer,
            &ctx.farmer,
            &Chain::Stellar,
            &ctx.tx(11),
            &1_000_000,
            &1,
        );
    }

    #[test]
    #[should_panic(expected = "Error(Contract, #507)")]
    fn test_relay_inbound_without_vault_liquidity_rejected() {
        let ctx = setup();
        ctx.client.register_wallet(&ctx.farmer);
        // The vault holds no Stellar credits. `available` is the lesser of the
        // advertised liquidity and the balance actually held, so the leg is
        // unroutable and the router refuses before any payout is attempted.
        ctx.client.relay_inbound(
            &ctx.relayer,
            &ctx.farmer,
            &Chain::Polygon,
            &ctx.tx(11),
            &1_000_000,
            &1,
        );
    }

    // ── pagination ────────────────────────────────────────────────────────────

    #[test]
    fn test_get_wallet_transfers_pages_oldest_first() {
        let ctx = setup();
        ctx.with_farmer_registered();
        for _ in 0..3 {
            open_default(&ctx, 1_000_000);
        }
        let first_page = ctx.client.get_wallet_transfers(&ctx.farmer, &0, &2);
        assert_eq!(first_page.len(), 2);
        assert_eq!(first_page.get(0).unwrap().id, 1);
        assert_eq!(first_page.get(1).unwrap().id, 2);

        let second_page = ctx.client.get_wallet_transfers(&ctx.farmer, &2, &2);
        assert_eq!(second_page.len(), 1);
        assert_eq!(second_page.get(0).unwrap().id, 3);
    }

    #[test]
    fn test_get_wallet_transfers_empty_for_unknown_wallet() {
        let ctx = setup();
        let stranger = Address::generate(&ctx.env);
        assert_eq!(ctx.client.get_wallet_transfers(&stranger, &0, &10).len(), 0);
    }

    #[test]
    #[should_panic(expected = "Error(Contract, #533)")]
    fn test_page_size_zero_rejected() {
        let ctx = setup();
        ctx.client.get_wallet_transfers(&ctx.farmer, &0, &0);
    }

    #[test]
    #[should_panic(expected = "Error(Contract, #533)")]
    fn test_page_size_above_max_rejected() {
        let ctx = setup();
        ctx.client
            .get_wallet_transfers(&ctx.farmer, &0, &(MAX_PAGE_SIZE + 1));
    }

    #[test]
    #[should_panic(expected = "Error(Contract, #532)")]
    fn test_cursor_past_end_rejected() {
        let ctx = setup();
        ctx.with_farmer_registered();
        open_default(&ctx, 1_000_000);
        ctx.client.get_wallet_transfers(&ctx.farmer, &2, &10);
    }

    // ── authorization wiring ──────────────────────────────────────────────────

    #[test]
    fn test_initialize_requires_admin_auth() {
        let (env, _admin, client) = setup_unmocked();
        let treasury = Address::generate(&env);
        assert!(client.try_initialize(&_admin, &treasury).is_err());
    }

    #[test]
    fn test_admin_functions_require_auth() {
        let (env, _admin, treasury, client) = setup_initialized_unmocked();
        assert!(client.try_pause().is_err());
        assert!(client.try_add_relayer(&Address::generate(&env)).is_err());
        assert!(client.try_set_treasury(&treasury).is_err());
    }

    #[test]
    fn test_farmer_functions_require_farmer_auth() {
        let (env, _admin, _treasury, client) = setup_initialized_unmocked();
        let farmer = Address::generate(&env);
        assert!(client.try_register_wallet(&farmer).is_err());
        assert!(client
            .try_set_chain_address(
                &farmer,
                &Chain::Polygon,
                &String::from_str(&env, "0x1111111111111111111111111111111111111111"),
            )
            .is_err());
        assert!(client.try_set_wallet_active(&farmer, &false).is_err());
    }

    /// A refund binds the authorization to the transfer's own farmer, not to
    /// whoever happens to call: the required address is the one recorded when
    /// the transfer was opened.
    #[test]
    fn test_refund_demands_the_transfers_own_farmer() {
        let ctx = setup();
        ctx.with_farmer_registered();
        let (id, _) = open_default(&ctx, 1_000_000);
        let transfer = ctx.client.get_transfer(&id).unwrap();

        ctx.env.ledger().set_timestamp(NOW + 600);
        ctx.client.refund_transfer(&id);

        let auths = ctx.env.auths();
        let demanded = auths
            .iter()
            .find(|(address, _)| *address == ctx.farmer)
            .map(|(address, _)| address);
        assert_eq!(demanded, Some(&ctx.farmer));
        assert!(auths.iter().all(|(address, _)| *address != ctx.admin));
        assert_eq!(
            transfer.status,
            TransferStatus::Pending,
            "the transfer was pending when the refund was authorised"
        );
    }

    // ── end-to-end ────────────────────────────────────────────────────────────

    #[test]
    fn test_full_outbound_settlement_flow() {
        let ctx = setup();
        ctx.with_farmer_registered();

        // A farmer quotes, then moves credits to the cheapest lane available.
        let amount = 5_000_000;
        let quote = ctx
            .client
            .get_best_route(&Chain::Stellar, &Chain::Polygon, &amount)
            .unwrap();
        let id = ctx.client.open_transfer(
            &ctx.farmer,
            &Chain::Polygon,
            &amount,
            &quote.amount_out,
            &(NOW + 900),
        );
        assert_eq!(ctx.vault_balance(), amount);

        ctx.env.ledger().set_timestamp(NOW + 300);
        let fee = ctx.client.settle_transfer(&ctx.relayer, &id, &ctx.tx(42));
        assert_eq!(fee, quote.total_fee);
        assert_eq!(ctx.treasury_balance(), quote.total_fee);
        assert_eq!(ctx.vault_balance(), amount - fee);
        assert_eq!(
            ctx.farmer_balance(),
            FARMER_BALANCE - amount,
            "the farmer's Stellar balance is debited once, at open time"
        );
    }

    #[test]
    fn test_round_trip_stellar_polygon_stellar() {
        let ctx = setup();
        ctx.with_farmer_registered();
        fund_vault(&ctx, 10_000_000);

        let amount = 2_000_000;
        let out_id =
            ctx.client
                .open_transfer(&ctx.farmer, &Chain::Polygon, &amount, &1, &(NOW + 600));
        let out_fee = ctx.client.get_transfer(&out_id).unwrap().fee;
        ctx.client
            .settle_transfer(&ctx.relayer, &out_id, &ctx.tx(1));

        // The bridged credits come back in as a fresh inbound leg.
        let in_id = ctx.client.relay_inbound(
            &ctx.relayer,
            &ctx.farmer,
            &Chain::Polygon,
            &ctx.tx(2),
            &amount,
            &1,
        );
        let inbound = ctx.client.get_transfer(&in_id).unwrap();

        let start = ctx.farmer_balance();
        assert!(start > 0);
        assert_eq!(ctx.client.total_transfer_count(), 2);
        // Two fees left the system across the round trip.
        assert_eq!(ctx.treasury_balance(), out_fee);
        assert!(inbound.fee > 0);
    }
}
