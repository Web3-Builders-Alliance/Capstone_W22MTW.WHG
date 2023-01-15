use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized{},

    #[error("Validator '{validator}' does not exist")]
    NotInValidatorSet {validator: String},

    
}
