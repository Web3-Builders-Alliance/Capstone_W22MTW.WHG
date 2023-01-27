#[cfg(test)]
mod tests {
    
    
    use crate::{ test_functions::{self}, msg::ExecuteMsg, contract::execute, functions::{invest_info_read, invest_info}};
    use cosmwasm_std::{testing::{mock_dependencies, mock_info, mock_env, MockQuerier}, Deps, coin, Querier};


    
    #[test]
    fn proper_initalization(){
        let mut deps = mock_dependencies();
        test_functions::set_contract();
    }

    #[test]
    fn test_bonding(){
        let mut deps = mock_dependencies();
        test_functions::set_contract();
        test_functions::bonding_tokens();
    }

    #[test]
    fn test_redelegate(){
        let mut deps = mock_dependencies();
        test_functions::set_contract();
        test_functions::redelegate();
    }
}