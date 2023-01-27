use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Addr, Decimal, Uint128};
use cw_storage_plus::Item;
use cw_utils::Duration;
use cw_controllers:: Claims;

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

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]

pub struct Supply{
    pub bonded: Uint128,
    pub claims: Uint128
}

pub const TOKEN_INFO: Item<TokenInfo> = Item::new("token_info");
pub const INVESTMENT: Item<InvestmentInfo> = Item::new("invest");
pub const CLAIMS: Claims = Claims::new("claims");
