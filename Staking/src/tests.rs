#[cfg(test)]
mod tests {
    
    
    use crate::{ test_functions};
   
    use cosmwasm_std::testing::{mock_dependencies};


    
    #[test]
    fn proper_initalization(){
        let mut deps = mock_dependencies();
        test_functions::set_contract(deps.as_mut());
       
    }
    
}