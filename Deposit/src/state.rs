use std::ops::Add;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Addr, Coin, Uint128};
use cw_storage_plus::{Item, Map};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Config {
    pub owner: Addr,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Deposits {
    pub count: i32,
    pub owner: Addr,
    pub coins: Coin,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Cw20Deposits {
    pub count: u64,
    pub owner: String,
    pub contract: String,
    pub amount: Uint128
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Withdraws {
    pub count: u128,
    pub owner: Addr,
    pub coins: Coin,
}
pub struct Cw20Withdraws{
    pub count: u128,
    pub owner: String,
    pub contract: String,
    pub amount: Uint128,
}

pub const CONFIG: Item<Config> = Item::new("config");

//key sender address, denom (that's native token what we will deposit)
pub const DEPOSITS: Map<(&str, &str),Deposits> = Map::new("deposits");

//key owner, cw20_contract_address
pub const CW20_DEPOSITS: Map<(&str, &str), Cw20Deposits> = Map::new("cw20_deposits");
