use anchor_lang::prelude::*;

#[account]
pub struct VaultState {
    pub token_mint: Pubkey,
    pub reward_rate: u64,
    pub total_staked: u64,
    pub last_update_slot: u64,
    pub vault_bump: u8,
}

impl VaultState {
    pub const SIZE: usize = 32 + 8 + 8 + 8 + 1; // token_mint + reward_rate + total_staked + last_update_slot + bump
}

#[account]
pub struct UserStake {
    pub user: Pubkey,
    pub vault: Pubkey,
    pub amount: u64,
    pub reward_debt: u64,
}

impl UserStake {
    pub const SIZE: usize = 8 + 32 + 32 + 8 + 8; // discriminator + user pubkey + vault pubkey + staked amount + reward debt
}
