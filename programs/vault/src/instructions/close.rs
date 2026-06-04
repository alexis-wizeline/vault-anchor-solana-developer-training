use anchor_lang::{
    prelude::*,
    system_program::{transfer, Transfer},
};

use crate::{VaultState, VAULT_SEED, VAULT_STATE_SEED};

#[derive(Accounts)]
pub struct Close<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(
        mut,
        seeds=[
            VAULT_STATE_SEED,
            owner.key().as_ref()
        ],
        bump=vault_authority.bump,
        has_one = owner,
        close = owner
    )]
    pub vault_authority: Account<'info, VaultState>,
    #[account(
        mut,
        seeds=[
            VAULT_SEED,
            owner.key().as_ref()
        ],
        bump=vault_authority.vault_bump
    )]
    pub vault: SystemAccount<'info>,

    pub system_progra: Program<'info, System>,
}

pub fn close_handler(ctx: Context<Close>) -> Result<()> {
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
    transfer(cpi_ctx, ctx.accounts.vault.lamports())?;

    Ok(())
}
