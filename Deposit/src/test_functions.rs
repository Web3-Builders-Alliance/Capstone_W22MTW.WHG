use cosmwasm_std::{DepsMut, testing::{mock_info, mock_env}, coin, Deps, from_binary, Uint128, coins};
use crate::{msg::{InstantiateMsg, ExecuteMsg, QueryMsg, DepositResponse}, contract::{instantiate, execute, query}, functions, state::Config};

pub const SENDER: &str = "sender_address";
const AMOUNT: u128 = 1_000;
const DENOM: &str = "utest";

pub fn setup_contract(deps: DepsMut){
    let msg = InstantiateMsg{};
    let info = mock_info(SENDER, &[]);
    let res = instantiate(deps, mock_env(), info, msg).unwrap();

    assert_eq!(res.messages.len(), 0);
}

pub fn deposit_coin(deps: DepsMut){
    let msg = ExecuteMsg::Deposits { amount: 100, denom: "utest".to_string() };
    let coins = vec![coin(100, DENOM)];
    let info = mock_info(SENDER, &coins);
    let res = execute(deps, mock_env(), info, msg).unwrap();

    assert_eq!(res.attributes[0].value, "execute_deposit".to_string());
    assert_eq!(res.attributes[1].value, DENOM.to_string());
    assert_eq!(res.attributes[2].value, 100.to_string());
}
pub fn withdraw_coin(deps: DepsMut){
    let msg = ExecuteMsg::Withdraw { amount: 10, denom: "utest".to_string() };
    let coins = vec![coin(10, DENOM)];
    let info = mock_info(SENDER, &coins);
    let res = execute(deps, mock_env(), info, msg).unwrap();

    assert_eq!(res.attributes[0].value, "execute_withdraw".to_string());
    assert_eq!(res.attributes[1].value, DENOM.to_string());
    assert_eq!(res.attributes[2].value, 10.to_string());
}

pub fn cw20_deposit(deps: DepsMut){
    let msg = ExecuteMsg::Cw20Deposits { owner:SENDER.to_string(), amount: Uint128::new(1_000)};
    let info = mock_info(SENDER, &[]);
    let owner = SENDER.to_string();
    let res = execute(deps, mock_env(), info, msg).unwrap();

    assert_eq!(res.attributes[0].value, "cw20_deposit".to_string());
    assert_eq!(res.attributes[1].value, owner.to_string());
    assert_eq!(res.attributes[2].value, AMOUNT.to_string());
}

pub fn cw20_withdraw(deps: DepsMut){
    let msg = ExecuteMsg::Cw20Withdraws { contract: SENDER.to_string(), amount: Uint128::new(10) };
    let info = mock_info(SENDER, &[]);
    let owner = SENDER.to_string();
    let res = execute(deps, mock_env(), info, msg).unwrap();

    assert_eq!(res.attributes[0].value, "cw20_withdraw".to_string());
    assert_eq!(res.attributes[1].value, SENDER.to_string());
    assert_eq!(res.attributes[2].value, 990.to_string());
}

pub fn query_deposit_coin(deps: Deps){
    let msg = QueryMsg::Deposits { address: SENDER.to_string() };
    let res = query(deps, mock_env(), msg).unwrap();
    let query = from_binary::<DepositResponse>(&res).unwrap();
    // let res = functions::query_deposit(deps, SENDER.to_string()).unwrap();

    assert_eq!(SENDER, query.deposits[0].1.owner);
    assert_eq!(DENOM, query.deposits[0].1.coins.denom);
    assert_eq!(AMOUNT.to_string(), query.deposits[0].1.coins.amount.to_string());
    assert_eq!(1, query.deposits[0].1.count);
    // assert_eq!(res.deposits[0].1.count,1);
    // assert_eq!(res.deposits[0].1.owner, "sender_address");
    
}

pub fn query_cw20_deposits(deps: Deps){
    let msg = QueryMsg::Cw20Deposits { address: SENDER.to_string() };
    let res = query(deps, mock_env(), msg).unwrap();
    let query = from_binary::<DepositResponse>(&res).unwrap();
    // let res = functions::query_deposit(deps, SENDER.to_string()).unwrap();

    assert_eq!(SENDER, query.deposits[0].1.owner);
    assert_eq!(DENOM, query.deposits[0].1.coins.denom);
    assert_eq!(AMOUNT.to_string(), query.deposits[0].1.coins.amount.to_string());
    assert_eq!(1, query.deposits[0].1.count);
    // assert_eq!(res.deposits[0].1.count,1);
    // assert_eq!(res.deposits[0].1.owner, "sender_address");
    
}