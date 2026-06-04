use anchor_lang::prelude::*;

#[constant]
pub const VAULT_SEED: &[u8] = b"vault";

#[constant]
pub const VAULT_STATE_SEED: &[u8] = b"vault_state";

pub const ANCHOR_DISCRIMINATOR_SIZE: usize = 8;
