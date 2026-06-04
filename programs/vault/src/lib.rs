pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("FyDvvhk88TLkkYAKNkE2YhrV6g8XDyJVcggWXKLe7jta");

#[program]
pub mod vault {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        initialize::deposit_handler(ctx)
    }

    pub fn deposit(ctx: Context<Deposit>, lamports_to_transfer: u64) -> Result<()> {
        deposit::handler(ctx, lamports_to_transfer)
    }

    pub fn withdraw(ctx: Context<Withdraw>, amount_to_withdraw: u64) -> Result<()> {
        withdraw::withdraw_handler(ctx, amount_to_withdraw)
    }
}
