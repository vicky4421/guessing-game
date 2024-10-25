use anchor_lang::prelude::*;

declare_id!("4XsGy8pPM4wRHBXS37q7PbfUJMK3cY7yAez92MxhEvWi");

#[program]
pub mod guessing_game {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
