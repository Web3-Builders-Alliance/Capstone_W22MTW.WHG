use cosmwasm_std::{DepsMut, MessageInfo, Response, BankMsg, coin, Uint128};

use crate::{state::{CONFIG, DEPOSITS, Deposits, CW20_DEPOSITS, Cw20Deposits}, ContractError};


pub fn execute_deposit(
    deps: DepsMut,
    info: MessageInfo,
    amount: u128,
    denom: String,
) -> Result<Response, ContractError> {
    let sender = info.sender.clone().to_string();

    let d_coins = info.funds[0].clone();
    
    //check sender is the owner 
    let owner = CONFIG.load(deps.storage).unwrap().owner;
    if sender != owner.to_string(){
        return Err(ContractError::InvalidOwner {  });
    }
    
    //check funds arre is a lenght of 1

    if info.funds.len() != 1 {
        return Err(ContractError::InvalidCoin {  });
    }

    //Check the sender has already deposit
    match DEPOSITS.load(deps.storage, (&sender, d_coins.denom.as_str())) {
        Ok (mut deposit) => {
            //increase the counter
            deposit.count = deposit.count.checked_add(1).unwrap();

            deposit.coins.amount = deposit
                .coins
                .amount
                .checked_add(d_coins.amount)
                .unwrap();

            //save it
            DEPOSITS
                .save(deps.storage, (&sender, d_coins.denom.as_str()), &deposit)
                .unwrap();

                let msg = BankMsg::Send { 
                    to_address: sender.clone(),
                    amount: vec![coin(amount, denom.clone())],
                 };
        }
        Err(_) => {
            //if deposit doesn't exist than create one
            let deposit = Deposits{
                owner: info.sender,
                count: 1,
                coins: d_coins.clone()
            };
            //save it
            DEPOSITS.save(deps.storage, (&sender, d_coins.denom.as_str()), &deposit).unwrap();
        }
    }

    Ok(Response::new()
        .add_attribute("method", "execute_deposit")
        .add_attribute("denom", d_coins.denom)
        .add_attribute("amount", d_coins.amount)
    )
}

pub fn execute_withdraw(
    deps: DepsMut, 
    info: MessageInfo,
    amount: u128,
    denom: String
) -> Result<Response, ContractError>{
    let sender = info.sender.clone().to_string();
    let mut deposit = DEPOSITS
        .load(deps.storage, (&sender, denom.as_str()))
        .unwrap();

        deposit.count = deposit.count.checked_sub(1).unwrap();
        //withdraw the amount
        deposit.coins.amount = deposit
            .coins
            .amount
            .checked_sub(Uint128::from(amount)) 
            .unwrap();

    DEPOSITS
        .save(deps.storage, (&sender, denom.as_str()), &deposit)
        .unwrap();

        let msg = BankMsg::Send { to_address: sender.clone(), amount: vec![coin(amount, denom.clone())]};
    
    Ok(Response::new()
        .add_attribute("method", "withdraw")
        .add_attribute("denom", denom)
        .add_attribute("amount", amount.to_string())
        .add_message(msg)
    )
}

pub fn execute_cw20_deposit(
    deps: DepsMut,
    info: MessageInfo,
    owner: String,
    amount: Uint128
) -> Result<Response, ContractError>{
    //owner(contract address)
    let cw20_contract_address = info.sender.clone().to_string();
    //check deposit is already exist
    match CW20_DEPOSITS.load(deps.storage, (&owner, &cw20_contract_address)){
        Ok(mut deposit)=> {
            deposit.count = deposit.count.checked_add(1).unwrap();
            deposit.amount = deposit.amount.checked_add(amount).unwrap();

            //save it
            CW20_DEPOSITS
                .save(deps.storage, (&owner, &cw20_contract_address), &deposit)
                .unwrap();
        }
        Err(_) => {
            //contract doesn't exist, create one
            let deposit = Cw20Deposits {
                count:1,
                owner: owner.clone(),
                contract: info.sender.clone().to_string(),
                amount: amount
            };
            //save it
            CW20_DEPOSITS
                .save(deps.storage, (&owner, &cw20_contract_address), &deposit)
                .unwrap();
        }
    }   
   Ok(Response::new()
        .add_attribute("method", "cw20_deposit")
        .add_attribute("owner", owner)
        .add_attribute("amount", amount.to_string())
    )
}