use cosmwasm_std::{DepsMut, testing::{mock_dependencies, mock_info, mock_env}, Decimal, Deps};

use crate::{msg::InstantiateMsg, contract::instantiate, functions, state::TokenInfo};

pub const VAL1:&str = "val1";
pub const CREATOR:&str = "creator_address";

pub fn set_contract(deps: DepsMut){
        let mut deps = mock_dependencies();
        
        let msg = InstantiateMsg{
            name_token:"utest".to_string(),
            symbol_token:"UT".to_string(),
            decimals:0,
            validator:String::from("val1"),
            unbonding_period: cw_utils::Duration::Time(0),
            emergancy_fee: Decimal::percent(20),
        };
        
        

    }



