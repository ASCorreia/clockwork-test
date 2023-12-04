use crate::*;

#[derive(Accounts)]
pub struct CloseAccount<'info> {
    #[account(mut)]
    destination: Signer<'info>,
    #[account(
        mut, 
        seeds = [SEED_COUNTER], 
        bump = dummy_account.bump,
        close = destination,
    )]
    dummy_account: Account<'info, DummyAccount>,
}

impl<'info> CloseAccount<'info> {
    pub fn close(&mut self) -> Result<()> {
        msg!("Account Closed");
        
        Ok(())
    }
}