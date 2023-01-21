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
    pub amount: Uint128,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Withdraws {
    pub count: u128,
    pub owner: Addr,
    pub coins: Coin,
}

pub const STATE: Item<Config> = Item::new("config");

pub const DEPOSITS: Map<(&str, &str), Deposits> = Map::new("deposits");

pub const CW20DEPOSITS: Map<(&str,&str), Deposits> = Map::new("cw20_deposits");