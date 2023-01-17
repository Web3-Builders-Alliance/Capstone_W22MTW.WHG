#[cfg(test)]
mod tests {
    use crate::contract::{instantiate, execute, query};
    use crate::functions;
    use crate::msg::InstantiateMsg;
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::{attr,from_binary};

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
}