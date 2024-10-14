use anchor_lang::prelude::*;

declare_id!("CzJr18diAB3RETMVPNQSZsb9mpv4hTHyStHJhe3Te1jj");

#[program]
pub mod favorites {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
