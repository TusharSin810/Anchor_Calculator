use anchor_lang::prelude::*;

declare_id!("5igcP1nawEo1A31YtZSj9z4nqrnbAvhmcQsoHXccwEDD");

#[program]
pub mod calculator {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
