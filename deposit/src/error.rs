use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Custom Error val: {val:?}")]
    CustomError { val: String },

    #[error("unathorized error")]
    InvalidExecuteMsg {},

    #[error("Invalid Coin")]
    InvalidCoin{},

    #[error("Invalid  Owner")]
    InvalidOwner{},

    #[error("Nothing to Withdraw")]
    NoCw20ToWithdraw{},
}
