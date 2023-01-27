#[cfg(test)]
mod tests {
    use std::vec;

    use crate::contract::{instantiate, execute, query};
    use crate::msg::{InstantiateMsg, ExecuteMsg, QueryMsg, AllPollResponse, PollResponse, VoteResponse};
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::{attr,from_binary};

    pub const ADDR1: &str = "addr1";
    
    #[test]
    fn proper_instantiate(){
        let mut deps = mock_dependencies();
        let _env = mock_env();
        let info = mock_info(ADDR1, &[]);

        let msg = InstantiateMsg {admin: None};

        let res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

        assert_eq!(res.attributes, vec![attr("method", "instantiate"), attr("admin",ADDR1)]);
        assert_eq!(0, res.messages.len());
        assert_eq!(ADDR1, "addr1");
       
    }

    #[test]
    fn test_create_poll(){

        let mut deps = mock_dependencies();
        let env = mock_env();
        let info = mock_info(ADDR1, &[]);
        //instantiate the contract

        let msg = InstantiateMsg{ admin :None};
        instantiate(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();

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
            
        instantiate(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

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
        instantiate(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();
        
        //create poll
        let msg = ExecuteMsg::CreatePoll { poll_id: "07".to_string(), topic: "Do you want ...?".to_string(), options: vec![
            "yes".to_string(),
            "no".to_string(),
            "I'm not interested".to_string()
        ]};

        execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

        //voting 
        let msg = ExecuteMsg::Vote { poll_id: "07".to_string(), vote:"yes".to_string(), topic: "Do you want ...?".to_string(), options: vec![
            "yes".to_string(),
            "no".to_string(),
            "I'm not interested".to_string()
        ]};
        let res = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

        assert_eq!(res.attributes, vec![attr("method", "vote"), attr("voting", "yes".to_string())])
        
    }

    #[test]
    fn query_all_poll(){
        let mut deps = mock_dependencies();
        let env = mock_env();
        let info = mock_info(ADDR1, &[]);
        let msg = InstantiateMsg { admin: None};
        instantiate(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

        //create poll
        let msg = ExecuteMsg::CreatePoll { poll_id: "07".to_string(), topic: "Do you want ...?".to_string(), options: vec![
            "yes".to_string(),
            "no".to_string(),
            "I'm not interested".to_string(),
        ] 
        };
        execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

        //create 2. poll
        let msg = ExecuteMsg::CreatePoll { poll_id: "08".to_string(), topic: "Do you want ...?".to_string(), options: vec![
            "yes".to_string(),
            "no".to_string(),
            "I'm not interested".to_string()
        ] 
        };
        execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

        //query
        let msg = QueryMsg::AllPolls {  };
        let result = query(deps.as_ref(), env, msg).unwrap();
        let res:AllPollResponse = from_binary(&result).unwrap();
        assert_eq!(res.polls.len(), 2)
    }

    #[test]
    fn test_query_poll_id(){
        let mut deps = mock_dependencies();
        let env = mock_env();
        let info =  mock_info(ADDR1, &[]);
        let msg = InstantiateMsg{admin:None };
        instantiate(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

        //create poll
        let msg = ExecuteMsg::CreatePoll { poll_id: "08".to_string(), topic: "".to_string(), options: vec![] };
        execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();
    
        //Query for poll_id
        let msg = QueryMsg::Poll { poll_id: "08".to_string() };

        let result = query(deps.as_ref(), env.clone(), msg).unwrap();
        let res: PollResponse = from_binary(&result).unwrap();

        assert!(res.poll.is_some());

        let msg = QueryMsg::Poll { poll_id: "10".to_string() };
        let result = query(deps.as_ref(), env.clone(), msg).unwrap();
        let res: PollResponse = from_binary(&result).unwrap();

        assert!(res.poll.is_none());

    }

    #[test]
    fn test_query_vote(){
        let mut deps = mock_dependencies();
        let env = mock_env();
        let info = mock_info(ADDR1, &[]);
        let msg = InstantiateMsg{admin: None};
        instantiate(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

        //create poll
        let msg = ExecuteMsg::CreatePoll { poll_id: "07".to_string(), topic: "Do you want ...?".to_string(), options: vec![
            "yes".to_string(),
            "no".to_string(),
            "I'm not interested".to_string()
        ]};
        execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

        //Vote
        let msg = ExecuteMsg::Vote { poll_id: "07".to_string(), vote: "yes".to_string(), topic: "Do you want ...?".to_string(), options: vec![
            "yes".to_string(),
            "no".to_string(),
            "I'm not interested".to_string()
        ]};
        execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();
        //query for the vote
        let msg = QueryMsg::Vote { poll_id: "07".to_string(), address: ADDR1.to_string() };

        let result = query(deps.as_ref(), env, msg).unwrap();
        let res:VoteResponse = from_binary(&result).unwrap();
        assert!(res.vote.is_some());
    }
}