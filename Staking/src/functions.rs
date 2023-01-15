use cosmwasm_std::{Storage, Addr, Uint128, QuerierWrapper, Response, StakingMsg, DepsMut, Env, MessageInfo};
use cosmwasm_storage::{singleton, Singleton};

use crate::{state::{TokenInfo, INVESTMENT}, contract, ContractError};



pub const KEY_TOKEN_INFO: &[u8] = b"token";

pub fn token_info(storage: &mut dyn Storage) -> Singleton<TokenInfo> {
    singleton(storage, KEY_TOKEN_INFO)
}

pub fn get_bonded(
    querier:&QuerierWrapper,
    contract:&Addr,
) -> Result<Uint128, ContractError>{
    let bonds = querier.query_all_delegations(contract)?;
    if bonds.is_empty(){
        return Ok(Uint128::zero());
    }

    let denom = bonds[0].amount.denom.as_str();
    bonds.iter().fold(Ok(Uint128::zero()), |racc,d|{
        let acc = racc?;
        if d.amount.denom.as_str() != denom{
            Err(ContractError::DifferentBondDenom { 
                denom1: denom.into(), 
                denom2: d.amount.denom.to_string()
             })
        } else {
            Ok(acc + d.amount.amount)
        }
    })
}

pub fn bond(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
) -> Result<Response,ContractError>{
    //check the token before bond
    let invest = INVESTMENT.load(deps.storage)?;
    let payment = info  
        .funds
        .iter()
        .find(|x|x.denom == invest.bond_denom)
        .ok_or_else(||ContractError::EmptyBalance { 
            denom: invest.bond_denom.clone(),
        })?;

    let bonded = get_bonded(&deps.querier, &env.contract.address)?;

    Ok(Response::new()
        .add_message(StakingMsg::Delegate { 
            validator: invest.validator,
            amount: payment.clone() 
        })
        .add_attribute("method", "bond")
        .add_attribute("from", info.sender)
        .add_attribute("bonded", payment.amount)
    )
}