use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("The transfer will result in an overflow operation")]
    InvalidTransferOverflow,
    #[msg("You can not transfer all your lamports to the vault, a minimum is needed to be rent excepmt")]
    InvalidTransferNotRentExcept,
    #[msg("Not enough blanace to transfer to the vault")]
    InsufficientFunds,
    #[msg("the amoun to withdraw should be more than 0")]
    InvalidWithdrawAmount,
    #[msg("the vault does not have enough funds to withdraw")]
    InsufficientFundsVault,
}
