use cosmwasm_std::testing::{mock_dependencies, mock_info, mock_env, MockQuerier, MOCK_CONTRACT_ADDR};
use cosmwasm_std::{Decimal, Deps, StdResult, Uint128, coin, CosmosMsg, coins, Validator, StakingMsg};
use crate::msg::{InstantiateMsg, ExecuteMsg, BalanceResponse, ClaimResponse}; 
use crate::contract::{instantiate, execute};
use crate::functions::{query_investment, query_token_info, balances_read, claims_read};

pub const VAL1:&str = "val1";
pub const CREATOR:&str = "creator_address";

fn sample_val(addr: &str) -> Validator {
    Validator { address: addr.to_owned(), commission: Decimal::percent(5), max_commission: Decimal::percent(10) , max_change_rate: Decimal::percent(1) }
}


pub fn set_validator(querier: &mut MockQuerier) {
    querier.update_staking(
        "utest",
        &[sample_val(VAL1)], &[]
    );
}

fn set_delegation(querier: &mut MockQuerier, _amount: u128, _denom: &str) {
    querier.update_staking(
        "utest", &[sample_val(VAL1)], &[]
    );
}
pub fn query_balance(
    deps: Deps, 
    address: &str
) -> StdResult<BalanceResponse> {
    let address = deps.api. addr_canonicalize(address)?;
    let balance = balances_read(deps.storage)
        .may_load(address.as_slice())?
        .unwrap_or_default();
    Ok(BalanceResponse { balance })
}

pub fn query_claim(
    deps: Deps, 
    address: &str,
) -> StdResult<ClaimResponse>{
    let address = deps.api.addr_canonicalize(address)?;
    let claims = claims_read(deps.storage)
        .may_load(address.as_slice())?
        .unwrap_or_default();

    Ok(ClaimResponse { claims })
}

fn get_balance(
    deps: Deps,
    addr: &str,
) -> Uint128 {
    query_balance(deps, addr).unwrap().balance
}

fn get_claim(
    deps: Deps,
    addr: &str,
) -> Uint128{
    query_claim(deps, addr).unwrap().claims
}

fn first_instantiate() -> InstantiateMsg{
    InstantiateMsg { 
        name_token: "utest".to_string(), 
        symbol_token: "UT".to_string(), 
        decimals: 0, 
        validator: String::from("val1"), 
        unbonding_period: cw_utils::Duration::Height(100), 
        emergancy_fee: Decimal::percent(20) 
    }
}


pub fn set_contract(){
        
        let mut deps = mock_dependencies();

        deps.querier.update_staking("utest", &[sample_val("val1")], &[]);
        let bonder = String::from("bonder");
        let msg = InstantiateMsg{
            name_token:"utest".to_string(),
            symbol_token:"UT".to_string(),
            decimals:0,
            validator: "val1".to_string(),
            unbonding_period: cw_utils::Duration::Height(100),
            emergancy_fee: Decimal::percent(20),
        };
        
        let info = mock_info(&bonder, &[]);
        let res = instantiate(deps.as_mut(), mock_env(), info, msg.clone()).unwrap();
        assert_eq!(0, res.messages.len());

        let token = query_token_info(deps.as_ref()).unwrap();
        assert_eq!(&token.name_token, "utest");
        assert_eq!(&token.symbol_token, "UT");
        assert_eq!(token.decimals, 0);
        

        assert_eq!(get_balance(deps.as_ref(), &bonder), Uint128::new(0));
        assert_eq!(get_claim(deps.as_ref(), &bonder), Uint128::new(0));

        let invest = query_investment(deps.as_ref()).unwrap();
        assert_eq!(&invest.owner, &bonder);
        assert_eq!(&invest.validator, &msg.validator);
        assert_eq!(invest.emergancy_fee, msg.emergancy_fee);
        assert_eq!(invest.unbonding_period, msg.unbonding_period);

        assert_eq!(invest.token_supply, Uint128::new(0));
        assert_eq!(invest.staked_tokens, coin(0, "utest"));
}

pub fn bonding_tokens(){
    let mut deps = mock_dependencies();
    set_validator(&mut deps.querier);
    let new_bonder = String::from("new_bonder");
    let msg = first_instantiate();
    let info = mock_info(&new_bonder, &[]);
    let res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
    assert_eq!(0, res.messages.len());
    
    let msg = ExecuteMsg::Bond {  };
    let info = mock_info(&new_bonder, &[coin(10, "utest")]); //bonding 10 utest token
    let res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();
    assert_eq!(1, res.messages.len());

    let delegate = &res.messages[0].msg;
    match delegate{ 
        CosmosMsg::Staking(StakingMsg::Delegate{validator, amount}) => {
            assert_eq!(validator.as_str(), VAL1);
            assert_eq!(amount, &coin(10, "utest"));
        }
        _=> panic!("Unexpected message: {:?}", delegate)
    }   

    let invest = query_investment(deps.as_ref()).unwrap();
    assert_eq!(invest.token_supply, Uint128::new(10));
    assert_eq!(invest.staked_tokens, coin(10, "utest"));
}

pub fn redelegate(){
    let mut deps = mock_dependencies();
    set_validator(&mut deps.querier);

    let bonder = String::from("bonder");
    let msg = first_instantiate();
    let info = mock_info(&bonder, &[]);
    let res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
    assert_eq!(0, res.messages.len());

    let new_bonder = String::from("new_bonder");
    let msg = ExecuteMsg::Bond {  };
    let info = mock_info(&new_bonder, &[coin(10, "utest")]);
    let res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();
    assert_eq!(1, res.messages.len());

    set_delegation(&mut deps.querier, 10, "utest");

    let redelagate_msg = ExecuteMsg::BondAllTokens {  };
    let info = mock_info(MOCK_CONTRACT_ADDR, &[]);
    deps.querier
        .update_balance(MOCK_CONTRACT_ADDR, coins(20, "utest"));
    let _ = execute(deps.as_mut(), mock_env(), info, redelagate_msg).unwrap();
    
    set_delegation(&mut deps.querier, 30, "utest");

    let invest = query_investment(deps.as_ref()).unwrap();
    assert_eq!(invest.token_supply, Uint128::new(10));
    assert_eq!(invest.staked_tokens, coin(10, "utest"));

}



