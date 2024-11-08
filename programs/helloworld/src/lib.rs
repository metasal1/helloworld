use anchor_lang::prelude::*;

declare_id!("HELLomJnTYj6V7i8cbeNFKJF4WsAwHj6WRBcwrHCXTGu");

#[program]
pub mod helloworld {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}