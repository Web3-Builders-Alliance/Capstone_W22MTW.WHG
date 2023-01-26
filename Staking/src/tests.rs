#[cfg(test)]
mod tests {
    
    
    use crate::{ test_functions::{self, CREATOR}, msg::ExecuteMsg, contract::execute, functions::{invest_info_read, invest_info}};
    use cosmwasm_std::{testing::{mock_dependencies, mock_info, mock_env}, Deps, coin};


    
    #[test]
    fn proper_initalization(){
        let mut deps = mock_dependencies();
        test_functions::set_contract(deps.as_mut());
       
    }



    #[test]
    fn test_bonding(){
        
        let mut deps = mock_dependencies();
        test_functions::set_contract(deps.as_mut());
        let msg = ExecuteMsg::Bond {  };
        let coins = vec![coin(100, "utest")];
    
        let info = mock_info(CREATOR, &[coin(100, "utest")]);
        let res = execute(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();
    
        assert_eq!(res.attributes[0].value, "bond");
        // assert_eq!(res.attributes[1].value, info.sender);
    }
}