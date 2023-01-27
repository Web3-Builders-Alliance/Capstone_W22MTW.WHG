use cosmwasm_std::{DepsMut, MessageInfo, Response, StdResult, Deps, Env, Binary, to_binary, Order};

use crate::{ContractError, state::{Poll, POLL, BALLOTS, Ballots}, msg::{AllPollResponse, PollResponse, VoteResponse}};

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
        topic: "".to_string(),
        options: opts
        
    };
    POLL.save(deps.storage, &poll_id, &poll)?;

    Ok(Response::new()
        .add_attribute("method", "create_poll")
        .add_attribute("poll_id", poll_id)
        .add_attribute("topic", topic)
    )
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
            // poll.options[position].1 += 1;
            poll.options[position].1.checked_add(1).unwrap();

            POLL.save(deps.storage, &poll_id, &poll)?;

            Ok(Response::new()
                .add_attribute("method", "vote")
                .add_attribute("voting", vote)
            )
        },
        //poll doesn't exist
        None => {
            create_poll(deps, info, poll_id, topic, options)
        },
    }
    
}

pub fn query_all_polls(
    deps: Deps,
    _env: Env,
) -> StdResult<Binary>{
    let polls = POLL
        .range(deps.storage, None, None, Order::Ascending)
        .map(|p| Ok(p?.1))
        .collect::<StdResult<Vec<_>>>()?;

    to_binary(&AllPollResponse {polls})
}

pub fn query_poll(
    deps: Deps,
    _env: Env,
    poll_id: String
) -> StdResult<Binary>{
    let poll = POLL. may_load(deps.storage, &poll_id)?;
    to_binary(&PollResponse{poll})
}

pub fn query_vote(
    deps: Deps,
    _env: Env,
    poll_id: String,
    address: String,
) -> StdResult<Binary>{
    let validated_address = deps.api.addr_validate(&address).unwrap();

    let vote = BALLOTS.may_load(deps.storage, (validated_address, &poll_id))?;

    to_binary(&VoteResponse { vote })
}