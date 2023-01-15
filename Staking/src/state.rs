use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Addr, Decimal};
use cw_storage_plus::Item;
use cw_utils::Duration;

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

pub const INVESTMENT: Item<InvestmentInfo> = Item::new("invest");
