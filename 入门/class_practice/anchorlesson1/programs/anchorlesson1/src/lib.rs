use anchor_lang::prelude::*;

declare_id!("5zpafbsuiEUrBd97Hca36q9uLkyu8Y47U55qmupuTzvt");

#[program]
pub mod anchorlesson1 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
