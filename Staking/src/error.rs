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

    #[error("No {denom} token sent")]
    EmptyBalance{ denom: String},

    #[error("Different donimations in bonds: '{denom1}' vs. '{denom2}'")]
    DifferentBondDenom{denom1:String, denom2:String}
}
