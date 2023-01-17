use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized error")]
    UnauthorizedError {},

    #[error("Poll doesn't exist! Do you want to create one?")]
    PollNotExist{},
}
