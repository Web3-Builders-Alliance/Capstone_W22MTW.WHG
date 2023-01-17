#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{Config, CONFIG, Poll, POLL, BALLOTS, Ballots};
use crate::test::*;


const CONTRACT_NAME: &str = "crates.io:voting";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");


#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    let admin = info.sender.to_string();
    let validated_admin = deps.api.addr_validate(&admin)?;
    let config = Config{
        admin: validated_admin.clone(),
    };
    CONFIG.save(deps.storage, &config)?;
    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("admin", validated_admin.to_string())
    )

}


#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg{
        ExecuteMsg::CreatePoll { poll_id, topic, options } => unimplemented!(),
        ExecuteMsg::Vote { poll_id, vote } => unimplemented!(),
    }
}

pub fn create_poll(
    deps: DepsMut, 
    info: MessageInfo,
    poll_id: String, 
    topic: String, 
    options: Vec<String>
) -> Result<Response, ContractError>{
    let mut opts:Vec<(String, u64)> = vec![];
    for option in options {
        opts.push((option, 0));
    }

    let poll = Poll {
        creator: info.sender,
        topic,
        options: opts
        
    };
    POLL.save(deps.storage, &poll_id, &poll)?;

    Ok(Response::new())
}

pub fn vote(
    deps: DepsMut,
    info: MessageInfo,
    poll_id: String,
    vote: String,
    topic: String,
    options: Vec<String>
) -> Result<Response, ContractError>{
    let poll = POLL.may_load(deps.storage, &poll_id)?;

    match poll {
        //poll exist
        Some( mut poll) => {
            BALLOTS.update(
                deps.storage, 
                (info.sender, &poll_id), 
                |ballot| -> StdResult<Ballots>{
                    match ballot {
                        Some(ballot) => {
                            let stored_vote = poll
                                .options
                                .iter()
                                .position(|option| option.0 == ballot.option)
                                .unwrap();

                            poll.options[stored_vote].1 -= 1;

                            Ok(Ballots { option: vote.clone(), })
                        }
                        None => {
                            Ok(Ballots { option: vote.clone(), })
                        }
                    }
                    
                },
            )?;

            let position = poll.options.iter().position(|option|option.0 == vote);
            if position.is_none(){
                return Err(ContractError::UnauthorizedError {  });
            }
            let position = position.unwrap();
            poll.options[position].1 += 1;

            POLL.save(deps.storage, &poll_id, &poll)?;

            Ok(Response::new())
        },
        //poll doesn't exist
        None => {
            create_poll(deps, info, poll_id, topic, options)
        },
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(_deps: Deps, _env: Env, _msg: QueryMsg) -> StdResult<Binary> {
    unimplemented!()
}


