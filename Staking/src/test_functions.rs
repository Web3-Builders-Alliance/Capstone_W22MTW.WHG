use std::{clone, env::set_var};

use cosmwasm_std::{DepsMut, testing::{mock_dependencies, mock_info, mock_env}, Decimal, Deps, coin, Api, Addr, QuerierWrapper};

use crate::{msg::{InstantiateMsg, ExecuteMsg}, contract::{instantiate, execute, self}, functions::{self, query_investment, get_bonded}, state::{TokenInfo, InvestmentInfo}};

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
        
        let info = mock_info(CREATOR, &[]);
        let res = instantiate(deps.as_mut(), mock_env(), info, msg.clone()).unwrap();
        assert_eq!(0, res.messages.len());
        assert_eq!(res.attributes[0].value,"instantiate");
        assert_eq!(res.attributes[1].value, "val1");
        assert_eq!(res.attributes[2].value, CREATOR);
}

pub fn bond(deps: DepsMut){
   
    
}



