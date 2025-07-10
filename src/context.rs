use anchor_lang::prelude::*;
use anchor_spl::token::Token;

use crate::state::{VaultState, UserStake};

#[derive(Accounts)]
pub struct InitializeVault<'info> {
    #[account(
        init,
        payer = payer,
        space = VaultState::SIZE,
        seeds = [b"vault", token_mint.key().as_ref()],
        bump
    )]
    pub vault: Account<'info, VaultState>,

    #[account(mut)]
    pub payer: Signer<'info>,

    /// CHECK: Token2022 mint, validated manually in handler
    pub token_mint: AccountInfo<'info>,

    /// CHECK: PDA holding reward tokens, validated manually
    #[account(mut)]
    pub reward_vault: AccountInfo<'info>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct StakeTokens<'info> {
    #[account(
        mut,
        seeds = [b"vault", vault.token_mint.as_ref()],
        bump
    )]
    pub vault: Account<'info, VaultState>,

    #[account(mut)]
    pub user: Signer<'info>,

    /// CHECK: user’s associated Token2022 account
    #[account(mut)]
    pub user_token_account: AccountInfo<'info>,

    #[account(
        init_if_needed,
        payer = user,
        space = UserStake::SIZE,
        seeds = [b"user-stake", user.key().as_ref(), vault.key().as_ref()],
        bump
    )]
    pub user_stake: Account<'info, UserStake>,

    /// CHECK: vault’s Token2022 account
    #[account(mut)]
    pub vault_token_account: AccountInfo<'info>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct WithdrawTokens<'info> {
    #[account(
        mut,
        seeds = [b"vault", vault.token_mint.as_ref()],
        bump
    )]
    pub vault: Account<'info, VaultState>,

    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        mut,
        seeds = [b"user-stake", user.key().as_ref(), vault.key().as_ref()],
        bump
    )]
    pub user_stake: Account<'info, UserStake>,

    /// CHECK: user's receiving Token2022 account
    #[account(mut)]
    pub user_token_account: AccountInfo<'info>,

    /// CHECK: vault’s Token2022 account
    #[account(mut)]
    pub vault_token_account: AccountInfo<'info>,

    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct ClaimRewards<'info> {
    #[account(
        mut,
        seeds = [b"vault", vault.token_mint.as_ref()],
        bump
    )]
    pub vault: Account<'info, VaultState>,

    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        mut,
        seeds = [b"user-stake", user.key().as_ref(), vault.key().as_ref()],
        bump
    )]
    pub user_stake: Account<'info, UserStake>,

    /// CHECK: user’s Token2022 reward destination
    #[account(mut)]
    pub user_reward_token_account: AccountInfo<'info>,

    /// CHECK: vault’s reward vault
    #[account(mut)]
    pub reward_vault: AccountInfo<'info>,

    pub token_program: Program<'info, Token>,
}
