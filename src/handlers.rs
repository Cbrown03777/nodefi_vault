use anchor_lang::prelude::*;
use anchor_spl::token::{transfer, Transfer, Token};

use crate::context::{InitializeVault, StakeTokens, WithdrawTokens, ClaimRewards};
use crate::errors::VaultError;
use crate::state::*;

pub fn handle_initialize_vault(
    ctx: Context<InitializeVault>,
    reward_rate: u64,
) -> Result<()> {
    let vault = &mut ctx.accounts.vault;

    vault.token_mint = ctx.accounts.token_mint.key();
    vault.reward_rate = reward_rate;
    vault.total_staked = 0;
    vault.last_update_slot = Clock::get()?.slot;
    vault.vault_bump = ctx.bumps.vault;

    Ok(())
}

pub fn handle_stake_tokens(
    ctx: Context<StakeTokens>,
    amount: u64,
) -> Result<()> {
    let vault = &mut ctx.accounts.vault;
    let user_stake = &mut ctx.accounts.user_stake;

    transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.user_token_account.to_account_info(),
                to: ctx.accounts.vault_token_account.to_account_info(),
                authority: ctx.accounts.user.to_account_info(),
            },
        ),
        amount,
    )?;

    vault.total_staked = vault
        .total_staked
        .checked_add(amount)
        .ok_or(VaultError::NumericalOverflow)?;

    user_stake.user = ctx.accounts.user.key();
    user_stake.vault = vault.key();
    user_stake.amount = user_stake
        .amount
        .checked_add(amount)
        .ok_or(VaultError::NumericalOverflow)?;
    user_stake.reward_debt = 0;

    Ok(())
}

pub fn handle_withdraw_tokens(
    ctx: Context<WithdrawTokens>,
    amount: u64,
) -> Result<()> {
    let vault = &mut ctx.accounts.vault;
    let user_stake = &mut ctx.accounts.user_stake;

    require!(
        user_stake.amount >= amount,
        VaultError::InsufficientStake
    );

    transfer(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.vault_token_account.to_account_info(),
                to: ctx.accounts.user_token_account.to_account_info(),
                authority: vault.to_account_info(),
            },
            &[&[b"vault", vault.token_mint.as_ref(), &[vault.vault_bump]]],
        ),
        amount,
    )?;

    vault.total_staked = vault
        .total_staked
        .checked_sub(amount)
        .ok_or(VaultError::NumericalOverflow)?;
    user_stake.amount = user_stake
        .amount
        .checked_sub(amount)
        .ok_or(VaultError::NumericalOverflow)?;

    Ok(())
}

pub fn handle_claim_rewards(ctx: Context<ClaimRewards>) -> Result<()> {
    let vault = &ctx.accounts.vault;
    let user_stake = &mut ctx.accounts.user_stake;

    let pending = user_stake
        .amount
        .checked_mul(vault.reward_rate)
        .ok_or(VaultError::NumericalOverflow)?;

    user_stake.reward_debt = user_stake
        .reward_debt
        .checked_add(pending)
        .ok_or(VaultError::NumericalOverflow)?;

    msg!("Claimed {} tokens as rewards", pending);

    Ok(())
}
