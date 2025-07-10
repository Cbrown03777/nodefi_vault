use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};

use crate::context::*;
use crate::errors::*;
use crate::handlers::*;
use crate::state::*;

pub mod constants;
pub mod context;
pub mod errors;
pub mod handlers;
pub mod state;

declare_id!("F14yVkHb8S1ByEStSshM1Bep6DkFpdA9yiUkgNcs9r1V");

#[program]
pub mod nodefi_vault {
    use super::*;

    pub fn initialize_vault(
        ctx: Context<InitializeVault>,
        reward_rate: u64,
    ) -> Result<()> {
        handle_initialize_vault(ctx, reward_rate)
    }

    pub fn stake_tokens(
        ctx: Context<StakeTokens>,
        amount: u64,
    ) -> Result<()> {
        handle_stake_tokens(ctx, amount)
    }

    pub fn withdraw_tokens(
        ctx: Context<WithdrawTokens>,
        amount: u64,
    ) -> Result<()> {
        handle_withdraw_tokens(ctx, amount)
    }

    pub fn claim_rewards(
        ctx: Context<ClaimRewards>,
    ) -> Result<()> {
        handle_claim_rewards(ctx)
    }
}

