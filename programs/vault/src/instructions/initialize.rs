use anchor_lang::{
    prelude::*,
    system_program::{transfer, Transfer},
};

use crate::{
    constants::{ANCHOR_DISCRIMINATOR_SIZE, VAULT_SEED, VAULT_STATE_SEED},
    error::ErrorCode::InvalidTransferNotRentExempt,
    VaultState,
};

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(mut,
        seeds=[
            VAULT_SEED,
            owner.key.as_ref()
        ],
        bump
    )]
    pub vault: SystemAccount<'info>,
    #[account(
        init,
        space=ANCHOR_DISCRIMINATOR_SIZE + VaultState::INIT_SPACE,
        payer=owner,
        seeds =[
            VAULT_STATE_SEED,
            owner.key.as_ref()
        ],
        bump
    )]
    pub vault_authority: Account<'info, VaultState>,
    pub system_program: Program<'info, System>,
}

pub fn initialize_handler(ctx: Context<Initialize>, max_withdraw: Option<u64>) -> Result<()> {
    let rent = Rent::get()?;
    let current_rent_lamports = rent.minimum_balance(0);

    let owner_lamports_after = ctx
        .accounts
        .owner
        .lamports()
        .checked_sub(current_rent_lamports)
        .ok_or(InvalidTransferNotRentExempt)?;

    require!(
        owner_lamports_after >= current_rent_lamports,
        crate::error::ErrorCode::InvalidTransferNotRentExempt
    );

    let cpi_accounts = Transfer {
        from: ctx.accounts.owner.to_account_info(),
        to: ctx.accounts.vault.to_account_info(),
    };

    let cpi_ctx = CpiContext::new(system_program::id(), cpi_accounts);

    transfer(cpi_ctx, current_rent_lamports)?;

    ctx.accounts.vault_authority.set_inner(VaultState {
        owner: ctx.accounts.owner.key(),
        max_withdraw,
        vault_bump: ctx.bumps.vault,
        bump: ctx.bumps.vault_authority,
    });

    Ok(())
}
