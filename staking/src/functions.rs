use cosmwasm_std::{Storage, Addr, Uint128, QuerierWrapper, Response, StakingMsg, DepsMut, Env, MessageInfo, DistributionMsg, coin, Deps, StdResult, StdError, to_binary, WasmMsg};
use cosmwasm_storage::{singleton, Singleton, singleton_read, ReadonlySingleton, Bucket, bucket, bucket_read, ReadonlyBucket};


use crate::state::{TokenInfo, INVESTMENT, CLAIMS, InvestmentInfo, Supply};
use crate::ContractError;
use crate::msg::{InvestmentResponse, TokenInfoResponse, ExecuteMsg};


pub const KEY_INVESTMENT: &[u8] = b"invest";
pub const KEY_TOKEN_INFO: &[u8] = b"token";
pub const PREFIX_BALANCE: &[u8] = b"balance";
pub const KEY_TOTAL_SUPPLY: &[u8] = b"total_supply";
pub const PREFIX_CLAIMS: &[u8] = b"claimm";
use deposit;



pub fn invest_info(storage: &mut dyn Storage) -> Singleton<InvestmentInfo>{
    singleton(storage, KEY_INVESTMENT)
}

pub fn invest_info_read(storage: &dyn Storage) -> ReadonlySingleton<InvestmentInfo>{
    singleton_read(storage, KEY_INVESTMENT)
}

pub fn token_info(storage: &mut dyn Storage) -> Singleton<TokenInfo> {
    singleton(storage, KEY_TOKEN_INFO)
}
pub fn token_info_read(storage: & dyn Storage) -> ReadonlySingleton<TokenInfo>{
    singleton_read(storage, KEY_TOKEN_INFO)
}
pub fn balances(storage: &mut dyn Storage) -> Bucket<Uint128>{
    bucket(storage, PREFIX_BALANCE)
}

pub fn balances_read(storage: &dyn Storage) -> ReadonlyBucket<Uint128> {
    bucket_read(storage, PREFIX_BALANCE)
}

pub fn total_supply(storage: &mut dyn Storage) -> Singleton<Supply> {
    singleton(storage, KEY_TOTAL_SUPPLY)
}

pub fn total_supply_read(storage: &dyn Storage) -> ReadonlySingleton<Supply> {
    singleton_read(storage, KEY_TOTAL_SUPPLY) 
}

pub fn claims(storage: &mut dyn Storage) -> Bucket<Uint128> {
    bucket(storage, PREFIX_CLAIMS)
}

pub fn claims_read(storage: &dyn Storage) -> ReadonlyBucket<Uint128> {
    bucket_read(storage, PREFIX_CLAIMS)
}

// pub fn transfer(
//     deps: DepsMut,
//     _env: Env,
//     info: MessageInfo,
//     recipient: String,
//     amount: Uint128
// ) -> Result<Response, ContractError>{
//     let rcpt = deps.api. addr_canonicalize(&recipient)?;
//     let sender = deps.api.addr_canonicalize(info.sender.as_str())?;

//     let mut accounts = balances(deps.storage);
//     accounts.update(&sender, |balance| -> StdResult<_> {
//         Ok(balance.unwrap_or_default().checked_sub(amount)?)
//     })?;
//     accounts.update(&rcpt, |balance| -> StdResult<_> {
//         Ok(balance.unwrap_or_default() + amount)
//     })?;

//     Ok(Response::new()
//     .add_attribute("method", "transfer")
//     .add_attribute("from", info.sender)
//     .add_attribute("to", recipient)
//     .add_attribute("amount", amount.to_string())
//     )
// }

pub fn get_bonded(
    querier:&QuerierWrapper,
    contract:&Addr,
) -> Result<Uint128, ContractError>{
    let bonds = querier.query_all_delegations(contract)?;
    if bonds.is_empty() {
        return Ok(Uint128::new(0));
    }
    let denom = bonds[0].amount.denom.as_str();
    bonds.iter().fold(Ok(Uint128::new(0)), |racc, d| {
        let acc = racc?;
        if d.amount.denom.as_str() != denom {
            Err(ContractError::Unauthorized {  })
        } else {
            Ok(acc + d.amount.amount)
        }
    })
}

fn assert_bonds(supply: &Supply, bonded: Uint128) -> StdResult<()>{
    if supply.bonded != bonded {
        Err(StdError::generic_err(format!("Stored bonded {}, but query bonded :{}", 
                supply.bonded, bonded    
        )))
    } else {
        Ok(())
    }
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

    let bonded = get_bonded(&deps.querier, &env.contract.address)?;

    let mut totals = total_supply(deps.storage);
    let mut supply = totals.load()?;
    assert_bonds(&supply, bonded)?;

    supply.bonded = bonded + payment.amount;

    totals.save(&supply)?;
    
    balances(deps.storage).update(&sender, |balance| -> StdResult<_>{
        Ok(balance.unwrap_or_default() )
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
    _info: MessageInfo
) -> Result<Response,ContractError>{
    let contract_addr = env.contract.address;
    let invest = invest_info_read(deps.storage).load()?;
    let msg = to_binary(&ExecuteMsg::BondAllTokens {  })?;

    Ok(Response::new()
        .add_message(DistributionMsg::WithdrawDelegatorReward { validator: invest.validator })
        .add_message(WasmMsg::Execute { 
            contract_addr: contract_addr.into(), 
            msg, 
            funds: vec![]})
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

    let invest = invest_info_read(deps.storage).load()?;
    let  balance = deps
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


pub fn query_token_info(
    deps: Deps
) -> StdResult<TokenInfoResponse>{
    
    let TokenInfo{
        name_token,
        symbol_token,
        decimals
    } = token_info_read(deps.storage).load()?;
    
    Ok(TokenInfoResponse { name_token, symbol_token, decimals})

    
}

pub fn query_investment(
    deps: Deps,
) -> StdResult<InvestmentResponse>{
    let invest = invest_info_read(deps.storage).load()?;
    let supply = total_supply_read(deps.storage).load()?;
    let total_supply = supply.bonded;
    
    Ok(InvestmentResponse { 
        staked_tokens: coin(supply.bonded.u128(), &invest.bond_denom), 
        owner: invest.owner.into(), 
        emergancy_fee: invest.emergancy_fee, 
        validator: invest.validator,
        token_supply: total_supply,
        unbonding_period:cw_utils::Duration::Height(100),
     })
}

pub fn transfer(
    deps: DepsMut,
    info: MessageInfo,
    amount: u128,
    denom: String
) -> Result<Response, ContractError> {
    deposit::deposit_functions::execute_deposit(deps, info, amount, denom);
    Ok(Response::default())
}

