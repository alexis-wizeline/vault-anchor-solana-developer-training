use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct VaultState {
    pub owner: Pubkey,
    pub current_deposited_lamports: u64,
    pub vault_bump: u8,
    pub bump: u8,
}
