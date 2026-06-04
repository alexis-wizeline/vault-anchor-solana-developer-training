use anchor_lang::{
    prelude::*,
    system_program::{transfer, Transfer},
};

use crate::{error::ErrorCode::InsufficientFundsVault, VaultState, VAULT_SEED, VAULT_STATE_SEED};

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(
        mut,
        has_one=owner,
        seeds=[
            VAULT_STATE_SEED,
            owner.key().as_ref()
        ],
        bump = vault_authority.bump
    )]
    pub vault_authority: Account<'info, VaultState>,

    #[account(
        mut,
        seeds=[
            VAULT_SEED,
            owner.key().as_ref()
        ],
        bump= vault_authority.vault_bump
    )]
    pub vault: SystemAccount<'info>,

    pub system_porgram: Program<'info, System>,
}

pub fn withdraw_handler(ctx: Context<Withdraw>, amount_to_withdraw: u64) -> Result<()> {
    require!(
        amount_to_withdraw > 0,
        crate::error::ErrorCode::InvalidWithdrawAmount
    );

    match ctx.accounts.vault_authority.max_withdraw {
        Some(amount) => {
            require!(
                amount >= amount_to_withdraw,
                crate::error::ErrorCode::InvalidMaxWithdrawExcceded
            );
        }
        None => {}
    }

    ctx.accounts
        .vault
        .lamports()
        .checked_sub(amount_to_withdraw)
        .ok_or(InsufficientFundsVault)?;

    let cpi_accounts = Transfer {
        from: ctx.accounts.vault.to_account_info(),
        to: ctx.accounts.owner.to_account_info(),
    };

    let owner_key = ctx.accounts.owner.key();
    let seeds: &[&[u8]] = &[
        VAULT_SEED,
        owner_key.as_ref(),
        &[ctx.accounts.vault_authority.vault_bump],
    ];
    let signer_seeds = &[seeds];

    let cpi_ctx = CpiContext::new_with_signer(system_program::id(), cpi_accounts, signer_seeds);

    transfer(cpi_ctx, amount_to_withdraw)?;

    Ok(())
}
