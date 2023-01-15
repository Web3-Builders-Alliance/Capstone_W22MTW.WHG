use cosmwasm_std::{Decimal, Uint128, Coin};
use cw_utils::Duration;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct InstantiateMsg {
    pub name_token: String,
    pub symbol_token: String,
    pub decimals: u8,
    pub validator: String,
    pub unbonding_period: Duration,
    pub emergancy_fee: Decimal
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    Bond {},
    Unbond{ amount: Uint128},
    Claim {},
    BondAllTokens{},
    Redelegate{},
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    Claims { address: String},
    Investment{},
    Balance{ address: String},
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct InvestmentResponse {
    pub staked_tokens: Coin,
    pub owner: String,
    pub emergancy_fee: Decimal,
    pub validator: String
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum MigrateMsg {}
