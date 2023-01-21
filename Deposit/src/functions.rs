use cosmwasm_std::{DepsMut, MessageInfo, Response, BankMsg, coin};

use crate::{state::{CONFIG, DEPOSITS, Deposits}, ContractError};


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