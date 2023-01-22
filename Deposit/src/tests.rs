#[cfg(test)]
mod test{
    
    use cosmwasm_std;
    use cosmwasm_std::testing::mock_dependencies;
    
    use crate::{test_functions::{self, SENDER}, functions};

    

    
   #[test]
   fn zero_instantiate(){
    let mut deps = mock_dependencies();
    test_functions::setup_contract(deps.as_mut());
    
   }
   #[test]
   fn one_deposit(){
    let mut deps = mock_dependencies();
    test_functions::setup_contract(deps.as_mut());
    test_functions::deposit_coin(deps.as_mut());
   }

   #[test]
   fn zero_query_deposit(){
    let mut deps = mock_dependencies();
    test_functions::setup_contract(deps.as_mut());
    test_functions::deposit_coin(deps.as_mut());
   }

   #[test]
   fn one_query_deposit(){
    let mut deps = mock_dependencies();
    test_functions::setup_contract(deps.as_mut());
    test_functions::deposit_coin(deps.as_mut());
   }
}