use cosmwasm_std::Storage;
use cosmwasm_storage::{singleton, Singleton};

use crate::state::TokenInfo;



pub const KEY_TOKEN_INFO: &[u8] = b"token";

pub fn token_info(storage: &mut dyn Storage) -> Singleton<TokenInfo> {
    singleton(storage, KEY_TOKEN_INFO)
}
