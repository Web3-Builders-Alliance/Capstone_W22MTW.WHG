use std::ops::Add;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::Addr;
use cw_storage_plus::{Item, Map};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Config {
    pub admin: Addr,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Poll {
    pub creator: Addr,
    pub topic: String,
    pub options: Vec<(String,u64)>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Votes {
    pub option: String,
}

pub const CONFIG: Item<Config> = Item::new("config");
pub const POLL: Map<&str, Poll> = Map::new("polls");
pub const VOTES: Map<(Addr,&str), Votes> = Map::new("votes");

