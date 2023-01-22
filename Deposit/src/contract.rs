#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, Uint128, BankMsg, Order, to_binary, WasmMsg};
use cw2::set_contract_version;
use cw20_base;

use crate::error::ContractError;
use crate::functions;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg, DepositResponse};
use crate::state::{CONFIG, Config, DEPOSITS, Deposits, Cw20Deposits, CW20_DEPOSITS};


const CONTRACT_NAME: &str = "crates.io:deposit";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");


#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    let config = Config { owner: info.sender.clone() };
    if config.owner != info.sender{
        return Err(ContractError::InvalidOwner {  });
    }
    CONFIG.save(deps.storage, &config).unwrap();

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", info.sender)
    )
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Deposits { amount, denom } => functions::execute_deposit(deps, info, amount, denom),
        ExecuteMsg::Withdraw { amount, denom } => functions::execute_withdraw(deps, info, amount, denom),
        ExecuteMsg::Cw20Deposits { owner, amount } => functions::execute_cw20_deposit(deps, info, owner, amount),  
        ExecuteMsg::Cw20Withdraws { owner, amount } => unimplemented!(),

    }
}

pub fn cw20_withdraw(
    deps: DepsMut,
    info: MessageInfo,
    contract: String,
    amount: Uint128
) -> Result<Response, ContractError>{
    let sender = info.sender.clone().to_string();
    match CW20_DEPOSITS.load(deps.storage, (&sender, &contract)){
        Ok(mut deposit) => {
            deposit.count = deposit.count.checked_sub(1).unwrap();
            deposit.amount = deposit.amount.checked_sub(amount).unwrap();

            //save it 
            CW20_DEPOSITS
                .save(deps.storage, (&sender,&contract), &deposit)
                .unwrap();

            let execute_msg = cw20_base::msg::ExecuteMsg::Transfer { recipient: sender.clone(), amount: Uint128::from(amount) };

            let msg = WasmMsg::Execute { contract_addr: contract.clone(), msg: to_binary(&execute_msg)?, funds: vec![] };

            // CW20_DEPOSITS.update(deps.storage, (&sender, &contract), |total:Option<Cw20Deposits>| -> StdResult<u64>{Ok(total.unwrap_or_default().checked_sub(1u64).unwrap())},)?;
            CW20_DEPOSITS.save(deps.storage, (&sender, &contract), &deposit)?;
            
            Ok(Response::new()
                .add_attribute("execute", "withdraw")
                .add_message(msg)
            )
        }
        Err(_) => {
            return Err(ContractError::NoCw20ToWithdraw {  });
        }
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(
    deps: Deps, 
    _env: Env, 
    msg: QueryMsg
) -> StdResult<Binary> {
    match msg {
        QueryMsg::Config {  }=> to_binary(&functions::get_config(deps)?),
        QueryMsg::Deposits { address } => to_binary(&functions::query_deposit(deps, address)?),
        QueryMsg::Cw20Deposits { address } => unimplemented!(),
    }
}