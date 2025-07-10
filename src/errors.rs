use anchor_lang::prelude::*;

#[error_code]
pub enum VaultError {
    #[msg("Overflow occurred during calculation.")]
    NumericalOverflow,

    #[msg("User does not have enough staked tokens to withdraw.")]
    InsufficientStake,

    #[msg("Missing bump seed for vault.")]
    MissingVaultBump,

    #[msg("Invalid authority signature.")]
    InvalidVaultAuthority,

    #[msg("Unauthorized access to this staking position.")]
    UnauthorizedAccess,

    #[msg("Insufficient reward pool balance.")]
    InsufficientRewards,

    #[msg("Math error during reward calculation.")]
    RewardCalculationError,
}
