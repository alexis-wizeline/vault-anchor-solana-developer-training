use anchor_lang::{
    prelude::*,
    solana_program::rent::Rent,
    system_program::{transfer, Transfer},
};

use crate::{
    error::ErrorCode::{InsufficientFunds, InvalidTransferOverflow},
    VaultState, VAULT_SEED, VAULT_STATE_SEED,
};

#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(
        mut,
        seeds=[
            VAULT_SEED,
            owner.key.as_ref()
        ],
        bump=vault_authority.vault_bump
    )]
    pub vault: SystemAccount<'info>,
    #[account(
        mut,
        has_one=owner,
        seeds=[
            VAULT_STATE_SEED,
            owner.key().as_ref()
        ],
        bump=vault_authority.bump
    )]
    pub vault_authority: Account<'info, VaultState>,

    pub system_porgram: Program<'info, System>,
}

pub fn handler(ctx: Context<Deposit>, lamports_to_transfer: u64) -> Result<()> {
    let new_deposited_lamports = ctx
        .accounts
        .vault_authority
        .current_deposited_lamports
        .checked_add(lamports_to_transfer)
        .ok_or(InvalidTransferOverflow)?;

    let reminded_lamports = ctx
        .accounts
        .owner
        .lamports()
        .checked_sub(lamports_to_transfer)
        .ok_or(InsufficientFunds)?;

    let rent = Rent::get()?;
    let neccesary_lamports = rent.minimum_balance(0);

    require!(
        reminded_lamports >= neccesary_lamports,
        crate::error::ErrorCode::InvalidTransferNotRentExcept
    );

    let cpi_accounts = Transfer {
        from: ctx.accounts.owner.to_account_info(),
        to: ctx.accounts.vault.to_account_info(),
    };

    let cpi_context = CpiContext::new(system_program::id(), cpi_accounts);
    transfer(cpi_context, lamports_to_transfer)?;

    ctx.accounts.vault_authority.current_deposited_lamports = new_deposited_lamports;
    Ok(())
}
