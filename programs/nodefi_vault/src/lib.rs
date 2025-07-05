use anchor_lang::prelude::*;

declare_id!("FFUoomcwc8Btpq1TgMSt28z6fueUmUpJNXKW2N6RETye");

#[program]
pub mod nodefi_vault {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
