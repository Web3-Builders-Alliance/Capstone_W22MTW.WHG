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
   fn one_withdraw(){
    let mut deps = mock_dependencies();
    test_functions::setup_contract(deps.as_mut());
    test_functions::deposit_coin(deps.as_mut());
    test_functions::withdraw_coin(deps.as_mut());
   }

   #[test]
   fn one_cw20_deposit(){
    let mut deps = mock_dependencies();
    test_functions::setup_contract(deps.as_mut());
    test_functions::cw20_deposit(deps.as_mut());
   }

   #[test]
   fn one_cw20_withdraw(){
    let mut deps = mock_dependencies();
    test_functions::setup_contract(deps.as_mut());
    test_functions::cw20_deposit(deps.as_mut());
    test_functions::cw20_withdraw(deps.as_mut());
   }

   #[test]
   fn zero_query_deposit(){ 
    let mut deps = mock_dependencies();
    test_functions::setup_contract(deps.as_mut());
    test_functions::deposit_coin(deps.as_mut());
    let res = functions::query_deposit(deps.as_ref(), SENDER.to_string()).unwrap();

    assert_eq!(res.deposits.len(), 1);
   }

   #[test]
   fn one_query_deposit(){
    let mut deps = mock_dependencies();
    test_functions::setup_contract(deps.as_mut());
    test_functions::deposit_coin(deps.as_mut());

    let res = functions::query_deposit(deps.as_ref(), SENDER.to_string()).unwrap();
    assert_eq!(res.deposits[0].1.count, 1);
    assert_eq!(res.deposits[0].1.owner, "sender_address");
   }

   #[test]
   fn one_query_cw20_deposit(){
    let mut deps = mock_dependencies();
    test_functions::setup_contract(deps.as_mut());
    test_functions::cw20_deposit(deps.as_mut());

    let res = functions::query_cw20_deposits(deps.as_ref(), SENDER.to_string()).unwrap();
    assert_eq!(res.deposits[0].1.count, 1);
    assert_eq!(res.deposits[0].1.owner, "sender_address");
   }
   
}