use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("the transfer will result in an overflow operation")]
    InvalidTransferOverflow,
    #[msg("you can not transfer all your lamports to the vault, a minimum is needed to be rent exempt")]
    InvalidTransferNotRentExempt,
    #[msg("not enough balance to transfer to the vault")]
    InsufficientFunds,
    #[msg("the amount to withdraw should be more than 0")]
    InvalidWithdrawAmount,
    #[msg("the vault does not have enough funds to withdraw")]
    InsufficientFundsVault,
    #[msg("the withdrawl can not let the vault without lamports, use close instead")]
    InvalidWithdrawDrainedVault,
    #[msg("the amount to withdraw is more than the max amount allowed in the authority")]
    InvalidMaxWithdrawExcceded,
}
