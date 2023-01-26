use cosmwasm_std::{Storage, Addr, Uint128, QuerierWrapper, Response, StakingMsg, DepsMut, Env, MessageInfo, DistributionMsg, coin, Deps, StdResult};
use cosmwasm_storage::{singleton, Singleton, singleton_read, ReadonlySingleton, Bucket, bucket};


use crate::{state::{TokenInfo, INVESTMENT, CLAIMS, TOKEN_INFO, InvestmentInfo}, ContractError, msg::{InvestmentResponse, TokenInfoResponse}};


pub const KEY_INVESTMENT: &[u8] = b"invest";
pub const KEY_TOKEN_INFO: &[u8] = b"token";
pub const PREFIX_BALANCE: &[u8] = b"balance";

pub fn invest_info(storage: &mut dyn Storage) -> Singleton<InvestmentInfo>{
    singleton(storage, KEY_INVESTMENT)
}

pub fn invest_info_read(storage: &dyn Storage) -> ReadonlySingleton<InvestmentInfo>{
    singleton_read(storage, KEY_INVESTMENT)
}

pub fn token_info(storage: &mut dyn Storage) -> Singleton<TokenInfo> {
    singleton(storage, KEY_TOKEN_INFO)
}
pub fn token_info_read(storage: &mut dyn Storage) -> ReadonlySingleton<TokenInfo>{
    singleton_read(storage, KEY_TOKEN_INFO)
}
pub fn balances(storage: &mut dyn Storage) -> Bucket<Uint128>{
    bucket(storage, PREFIX_BALANCE)
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
    let sender = deps.api.addr_canonicalize(info.sender.as_str())?;
    //check the token before bond
   
    let invest = invest_info_read(deps.storage).load()?;
    let payment = info  
        .funds
        .iter()
        .find(|x|x.denom == invest.bond_denom)
        .ok_or_else(||ContractError::EmptyBalance { 
            denom: invest.bond_denom.clone(),
        })?;

    let _bonded = get_bonded(&deps.querier, &env.contract.address)?;
    
    balances(deps.storage).update(&sender, |balance| -> StdResult<_>{
        Ok(balance.unwrap_or_default())
    })?;

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

pub fn redelegate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo
) -> Result<Response,ContractError>{
    
    let invest = INVESTMENT.load(deps.storage)?;
    let payment = info  
        .funds
        .iter()
        .find(|x|x.denom == invest.bond_denom)
        .ok_or_else(||ContractError::EmptyBalance { 
            denom: invest.bond_denom.clone(),
        })?;

    let _bonded = get_bonded(&deps.querier, &env.contract.address)?;

    Ok(Response::new()
        .add_message(StakingMsg::Redelegate { src_validator: info.sender.to_string(), dst_validator: invest.validator.clone(), amount: payment.clone() } )
        .add_attribute("method", "redelegate")
        .add_attribute("from", info.sender)
        .add_attribute("new_validator", invest.validator)
        .add_attribute("bonded", payment.amount)
    )
}


pub fn bond_all_tokens(
    deps: DepsMut, 
    env: Env,
    info: MessageInfo
) -> Result<Response,ContractError> {
    if info.sender !=env.contract.address{
        return Err(ContractError::Unauthorized {  });
    }

    let invest = INVESTMENT.load(deps.storage)?;
    let balance = deps
        .querier
        .query_balance(&env.contract.address, &invest.bond_denom)?;


    Ok(Response::new()
        .add_message(StakingMsg::Delegate { 
            validator: invest.validator,
            amount: balance.clone()})
        .add_attribute("method", "bond_all_tokens")
        .add_attribute("bonded", balance.amount)
    )
        
} 

pub fn claim(
    deps: DepsMut, 
    env: Env,
    info: MessageInfo,
) -> Result<Response, ContractError>{
    let invest = INVESTMENT.load(deps.storage)?;
    let mut balance = deps 
        .querier
        .query_balance(&env.contract.address, &invest.bond_denom)?;
    
    let reward = CLAIMS.claim_tokens(deps.storage, &info.sender, &env.block, Some(balance.amount))?;
    if reward == Uint128::zero(){
        return Err(ContractError::NothingToClaim {  });
    }
    balance.amount = reward;
    
    Ok(Response::new()
        .add_message(DistributionMsg::WithdrawDelegatorReward { validator: invest.validator })
        .add_attribute("method", "claim")
        .add_attribute("from", info.sender)
        .add_attribute("reward", reward)
    )
}

pub fn unbond(
    deps: DepsMut, 
    env: Env,
    info: MessageInfo,
    amount: Uint128,
) -> Result<Response, ContractError>{
    let invest = INVESTMENT.load(deps.storage)?;

    let _bonded = get_bonded(&deps.querier, &env.contract.address).unwrap();

    let balance = deps 
        .querier
        .query_balance(&env.contract.address, &invest.bond_denom)?;
    
    let reward = CLAIMS.claim_tokens(deps.storage, &info.sender, &env.block, Some(balance.amount))?;
    if reward == Uint128::zero(){
        return Err(ContractError::NothingToClaim {  });
    }

    // let unbond_amount= reward.checked_add(bonded);

    Ok(Response::new()
        . add_message(StakingMsg::Undelegate { validator: invest.validator, amount: coin(reward.u128(), &invest.bond_denom)})
        .add_attribute("method", "unbond")
        .add_attribute("owner", info.sender)
        .add_attribute("unbonded", amount)
    )

    
}

pub fn balance(deps: Deps, address: String) -> StdResult<Uint128>{
    let _address = deps.api.addr_validate(&address)?;
    Ok(Uint128::zero())
}

pub fn query_investment(
    deps: Deps,
    env: Env,
) -> StdResult<InvestmentResponse>{
    let invest = INVESTMENT.load(deps.storage)?;
    let bonded = get_bonded(&deps.querier, &env.contract.address).unwrap();
    
    let res = InvestmentResponse{
        staked_tokens: coin(bonded.u128(), &invest.bond_denom),
        owner: invest.owner.to_string(),
        validator: invest.validator,
        emergancy_fee: invest.emergancy_fee
    };
    Ok(res)
}

pub fn query_token_info(
    deps: Deps, 
) -> StdResult<TokenInfoResponse>{
    let info = TOKEN_INFO.load(deps.storage)?;
    

    let res = TokenInfoResponse{
        name_token: info.name_token,
        symbol_token: info.symbol_token,
        decimals: info.decimals
    };
    Ok(res)
}