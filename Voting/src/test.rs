#[cfg(test)]
mod tests {
    use std::vec;

    use crate::contract::{instantiate, execute, query};
    use crate::functions::{self, vote};
    use crate::msg::{InstantiateMsg, ExecuteMsg};
    use crate::state::Poll;
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::{attr,from_binary, DepsMut};

    pub const ADDR1: &str = "addr1";
    pub const ADDR2: &str = "addr2";

    #[test]
    fn proper_instantiate(){
        let mut deps = mock_dependencies();
        let env = mock_env();
        let info = mock_info(ADDR1, &[]);

        let msg = InstantiateMsg {admin: None};

        let res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

        assert_eq!(res.attributes, vec![attr("method", "instantiate"), attr("admin",ADDR1)])
        // assert_eq!(0, res.messages.len());
        // assert_eq!(ADDR1, "addr1");
        // assert_eq!("addr2", ADDR2);
    }

    #[test]
    fn test_create_poll(){

        let mut deps = mock_dependencies();
        let env = mock_env();
        let info = mock_info(ADDR1, &[]);
        //instantiate the contract

        let msg = InstantiateMsg{ admin :None};
        let res = instantiate(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();

        //create poll
        let msg = ExecuteMsg::CreatePoll { poll_id: "07".to_string(), topic: "Do you want ...?".to_string(), options: vec![
            "yes".to_string(),
            "no".to_string(),
            "I'm not interested".to_string()
        ] };

        let res = execute(deps.as_mut(),env, info, msg).unwrap();
        assert_eq!(0, res.messages.len())
    }

    #[test]

    fn test_create_poll_id(){
        let mut deps = mock_dependencies();
        let env = mock_env();
        let info = mock_info(ADDR1, &[]);

        //instantiage 
        let msg = InstantiateMsg {admin: None};
        
        let res = instantiate(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

        //create poll
    
        let msg = ExecuteMsg::CreatePoll { poll_id: "07".to_string(), topic: "".to_string(), options: vec![
            "yes".to_string(),
            "no".to_string(),
            "I'm not interested".to_string()
        ]};

        
        let res = execute(deps.as_mut(), env, info, msg).unwrap();
        assert_eq!(res.attributes, vec![attr("method", "create_poll"), attr("poll_id", "07".to_string()), attr("topic", "".to_string())])
        
    }
    #[test]
    fn test_vote(){
        let mut deps = mock_dependencies();
        let env = mock_env();
        let info = mock_info(ADDR1, &[]);
        let msg = InstantiateMsg { admin: None};
        let res = instantiate(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();
        
        //create poll
        let msg = ExecuteMsg::CreatePoll { poll_id: "07".to_string(), topic: "Do you want ...?".to_string(), options: vec![
            "yes".to_string(),
            "no".to_string(),
            "I'm not interested".to_string()
        ]};

        let res = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

        //voting 
        let msg = ExecuteMsg::Vote { poll_id: "07".to_string(), vote:"yes".to_string(), topic: "Do you want ...?".to_string(), options: vec![
            "yes".to_string(),
            "no".to_string(),
            "I'm not interested".to_string()
        ]};
        let res = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

        assert_eq!(res.attributes, vec![attr("method", "vote"), attr("voting", "yes".to_string())])
        
    }
}