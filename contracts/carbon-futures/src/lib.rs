#![no_std]

//! Carbon credit derivatives — futures trading (v1)
//!
//! Farmers can lock in a forward price for next season's verified credits, while
//! buyers can hedge against future price volatility. The contract stores signed
//! forward contracts and exposes a transparent price quote derived from the spot
//! price plus an annual carry rate.

use soroban_sdk::{
    contract, contracterror, contractimpl, contracttype, panic_with_error, symbol_short, Address,
    Env, Symbol,
};

const DEFAULT_ANNUAL_CARRY_BPS: u32 = 500; // 5.00%
const MAX_QUANTITY_TONNES: i128 = 1_000_000;
const BPS_DENOMINATOR: i128 = 10_000;

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum FuturesError {
    AlreadyInitialized = 1,
    NotInitialized = 2,
    Unauthorized = 3,
    ProjectRequired = 4,
    QuantityMustBePositive = 5,
    QuantityTooLarge = 6,
    PriceMustBePositive = 7,
    DeliveryMustBeFuture = 8,
    ContractNotFound = 9,
    ContractNotOpen = 10,
    InvalidSettlement = 11,
    InvalidCarryRate = 12,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Config {
    pub admin: Address,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ContractStatus {
    Open,
    Matched,
    Settled,
    Cancelled,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FuturesQuote {
    pub project_id: Symbol,
    pub quantity_tonnes: i128,
    pub delivery_year: u32,
    pub years_to_delivery: u32,
    pub locked_price_per_ton: i128,
    pub notional_value: i128,
    pub annual_carry_bps: u32,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForwardContract {
    pub id: u64,
    pub project_id: Symbol,
    pub seller: Address,
    pub buyer: Option<Address>,
    pub quantity_tonnes: i128,
    pub strike_price_per_ton: i128,
    pub delivery_year: u32,
    pub current_year: u32,
    pub status: ContractStatus,
    pub created_at: u64,
}

#[contracttype]
enum DataKey {
    Config,
    NextContractId,
    Contract(u64),
}

#[contract]
pub struct CarbonFutures;

#[contractimpl]
impl CarbonFutures {
    pub fn initialize(env: Env, admin: Address) {
        if env.storage().instance().has(&DataKey::Config) {
            panic_with_error!(&env, FuturesError::AlreadyInitialized);
        }
        admin.require_auth();
        env.storage().instance().set(&DataKey::Config, &Config { admin });
    }

    pub fn quote(
        env: Env,
        project_id: Symbol,
        quantity_tonnes: i128,
        spot_price_per_ton: i128,
        delivery_year: u32,
        current_year: u32,
        annual_carry_bps: Option<u32>,
    ) -> FuturesQuote {
        Self::validate_quote(
            &env,
            project_id.clone(),
            quantity_tonnes,
            spot_price_per_ton,
            delivery_year,
            current_year,
            annual_carry_bps,
        )
        .unwrap_or_else(|err| panic_with_error!(&env, err));

        let carry_bps = annual_carry_bps.unwrap_or(DEFAULT_ANNUAL_CARRY_BPS);
        let years_to_delivery = delivery_year
            .checked_sub(current_year)
            .expect("delivery_year must be >= current_year");

        let multiplier = BPS_DENOMINATOR + (carry_bps as i128 * years_to_delivery as i128);
        let locked_price = spot_price_per_ton
            .checked_mul(multiplier)
            .and_then(|value| value.checked_div(BPS_DENOMINATOR))
            .unwrap_or_else(|| panic_with_error!(&env, FuturesError::InvalidCarryRate));

        let notional_value = locked_price
            .checked_mul(quantity_tonnes)
            .unwrap_or_else(|| panic_with_error!(&env, FuturesError::QuantityTooLarge));

        FuturesQuote {
            project_id,
            quantity_tonnes,
            delivery_year,
            years_to_delivery,
            locked_price_per_ton: locked_price,
            notional_value,
            annual_carry_bps: carry_bps,
        }
    }

    pub fn open_contract(
        env: Env,
        seller: Address,
        project_id: Symbol,
        quantity_tonnes: i128,
        spot_price_per_ton: i128,
        delivery_year: u32,
        current_year: u32,
    ) -> u64 {
        seller.require_auth();
        let quote = Self::quote(
            env.clone(),
            project_id,
            quantity_tonnes,
            spot_price_per_ton,
            delivery_year,
            current_year,
            None,
        );

        let next_id: u64 = env.storage().instance().get(&DataKey::NextContractId).unwrap_or(0);
        let contract = ForwardContract {
            id: next_id,
            project_id: quote.project_id,
            seller: seller.clone(),
            buyer: None,
            quantity_tonnes: quote.quantity_tonnes,
            strike_price_per_ton: quote.locked_price_per_ton,
            delivery_year: quote.delivery_year,
            current_year,
            status: ContractStatus::Open,
            created_at: env.ledger().timestamp(),
        };

        env.storage().persistent().set(&DataKey::Contract(next_id), &contract);
        env.storage()
            .instance()
            .set(&DataKey::NextContractId, &(next_id + 1));

        next_id
    }

    pub fn match_contract(env: Env, buyer: Address, contract_id: u64) -> FuturesQuote {
        buyer.require_auth();
        let mut contract: ForwardContract = env
            .storage()
            .persistent()
            .get(&DataKey::Contract(contract_id))
            .unwrap_or_else(|| panic_with_error!(&env, FuturesError::ContractNotFound));

        if contract.status != ContractStatus::Open {
            panic_with_error!(&env, FuturesError::ContractNotOpen);
        }

        contract.buyer = Some(buyer.clone());
        contract.status = ContractStatus::Matched;
        env.storage().persistent().set(&DataKey::Contract(contract_id), &contract);

        let years_to_delivery = contract
            .delivery_year
            .checked_sub(contract.current_year)
            .unwrap_or(0);
        let notional_value = contract
            .strike_price_per_ton
            .checked_mul(contract.quantity_tonnes)
            .unwrap_or_else(|| panic_with_error!(&env, FuturesError::QuantityTooLarge));

        FuturesQuote {
            project_id: contract.project_id.clone(),
            quantity_tonnes: contract.quantity_tonnes,
            delivery_year: contract.delivery_year,
            years_to_delivery,
            locked_price_per_ton: contract.strike_price_per_ton,
            notional_value,
            annual_carry_bps: DEFAULT_ANNUAL_CARRY_BPS,
        }
    }

    pub fn settle_contract(env: Env, caller: Address, contract_id: u64) {
        caller.require_auth();
        let mut contract: ForwardContract = env
            .storage()
            .persistent()
            .get(&DataKey::Contract(contract_id))
            .unwrap_or_else(|| panic_with_error!(&env, FuturesError::ContractNotFound));

        let is_seller = caller == contract.seller;
        let is_buyer = contract.buyer == Some(caller.clone());
        if !is_seller && !is_buyer {
            panic_with_error!(&env, FuturesError::Unauthorized);
        }
        if contract.status == ContractStatus::Settled || contract.status == ContractStatus::Cancelled {
            panic_with_error!(&env, FuturesError::InvalidSettlement);
        }

        contract.status = ContractStatus::Settled;
        env.storage().persistent().set(&DataKey::Contract(contract_id), &contract);
    }

    pub fn cancel_contract(env: Env, seller: Address, contract_id: u64) {
        seller.require_auth();
        let mut contract: ForwardContract = env
            .storage()
            .persistent()
            .get(&DataKey::Contract(contract_id))
            .unwrap_or_else(|| panic_with_error!(&env, FuturesError::ContractNotFound));

        if contract.seller != seller || contract.status != ContractStatus::Open {
            panic_with_error!(&env, FuturesError::Unauthorized);
        }

        contract.status = ContractStatus::Cancelled;
        env.storage().persistent().set(&DataKey::Contract(contract_id), &contract);
    }

    pub fn get_contract(env: Env, contract_id: u64) -> ForwardContract {
        env.storage()
            .persistent()
            .get(&DataKey::Contract(contract_id))
            .unwrap_or_else(|| panic_with_error!(&env, FuturesError::ContractNotFound))
    }

    fn validate_quote(
        _env: &Env,
        project_id: Symbol,
        quantity_tonnes: i128,
        spot_price_per_ton: i128,
        delivery_year: u32,
        current_year: u32,
        annual_carry_bps: Option<u32>,
    ) -> Result<(), FuturesError> {
        if project_id == symbol_short!("") {
            return Err(FuturesError::ProjectRequired);
        }
        if quantity_tonnes <= 0 {
            return Err(FuturesError::QuantityMustBePositive);
        }
        if quantity_tonnes > MAX_QUANTITY_TONNES {
            return Err(FuturesError::QuantityTooLarge);
        }
        if spot_price_per_ton <= 0 {
            return Err(FuturesError::PriceMustBePositive);
        }
        if delivery_year <= current_year {
            return Err(FuturesError::DeliveryMustBeFuture);
        }
        if let Some(carry_bps) = annual_carry_bps {
            if carry_bps > 10_000 {
                return Err(FuturesError::InvalidCarryRate);
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use soroban_sdk::testutils::Address as _;

    fn setup() -> (Env, Address, CarbonFuturesClient<'static>) {
        let env = Env::default();
        env.mock_all_auths();
        let contract_id = env.register_contract(None, CarbonFutures);
        let client = CarbonFuturesClient::new(&env, &contract_id);
        let admin = Address::generate(&env);
        client.initialize(&admin);
        (env, admin, client)
    }

    #[test]
    fn quote_uses_forward_price_with_carry() {
        let env = Env::default();
        env.mock_all_auths();
        let contract_id = env.register_contract(None, CarbonFutures);
        let client = CarbonFuturesClient::new(&env, &contract_id);
        let admin = Address::generate(&env);
        client.initialize(&admin);

        let quote = client.quote(
            &Symbol::new(&env, "proj_001"),
            &100,
            &45_50,
            &2028,
            &2026,
            &Some(500),
        );

        assert_eq!(quote.years_to_delivery, 2);
        assert_eq!(quote.locked_price_per_ton, 50_05);
        assert_eq!(quote.notional_value, 5_005_00);
    }

    #[test]
    fn open_and_match_contract_flow() {
        let (env, _admin, client) = setup();
        let seller = Address::generate(&env);
        let buyer = Address::generate(&env);

        let id = client.open_contract(
            &seller,
            &Symbol::new(&env, "proj_001"),
            &100,
            &45_50,
            &2028,
            &2026,
        );

        let quote = client.match_contract(&buyer, &id);
        assert_eq!(quote.quantity_tonnes, 100);
        assert_eq!(quote.locked_price_per_ton, 50_05);

        let contract = client.get_contract(&id);
        assert_eq!(contract.status, ContractStatus::Matched);
        assert_eq!(contract.buyer, Some(buyer));
    }

    #[test]
    fn reject_past_delivery_year() {
        let env = Env::default();
        let result = CarbonFutures::validate_quote(
            &env,
            Symbol::new(&env, "proj_001"),
            100,
            45_50,
            2026,
            2026,
            None,
        );

        assert_eq!(result, Err(FuturesError::DeliveryMustBeFuture));
    }
}
