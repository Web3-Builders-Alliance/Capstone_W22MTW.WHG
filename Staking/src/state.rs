use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Addr, Decimal, Coin, Uint128};
use cw_storage_plus::{Item, Map};
use cw_utils::Duration;
use cw_controllers:: Claims;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Config{
    pub owner : Addr,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Deposit{
    pub count: i32,
    pub owner: Addr,
    pub coins: Coin,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Withdraw{
    pub count: u128,
    pub owner: Addr,
    pub coins: Coin,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Cw20Deposits{
    pub count: u64,
    pub owner: String,
    pub contract: String,
    pub amount: Uint128
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InvestmentInfo {
    pub owner: Addr,
    pub bond_denom: String,
    pub unbonding_period: Duration,
    pub emergancy_fee: Decimal,
    pub validator: String,
    
}
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]

pub struct TokenInfo{
    pub name_token: String,
    pub symbol_token: String,
    pub decimals: u8,
}

pub const CONFIG: Item<Config> = Item::new("config");
pub const DEPOSITS: Map<(&str,&str), Deposit> = Map::new("deposits");
pub const CW20_DEPOSITS: Map<(&str, &str), Cw20Deposits> = Map::new("cw20deposits");
pub const TOKEN_INFO: Item<TokenInfo> = Item::new("token_info");
pub const INVESTMENT: Item<InvestmentInfo> = Item::new("invest");
pub const CLAIMS: Claims = Claims::new("claims");
