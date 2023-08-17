use clockwork_sdk::state::Thread;

use crate::*;

#[derive(Accounts)]
pub struct Increment<'info> {
    #[account(mut, seeds = [SEED_COUNTER], bump = dummy_account.bump)]
    dummy_account: Account<'info, DummyAccount>,
    /// Verify that only this thread can execute the Increment Instruction
    #[account(signer, constraint = thread.authority.eq(&thread_authority.key()))]
    pub thread: Account<'info, Thread>,

    /// The Thread Admin
    /// The authority that was used as a seed to derive the thread address
    /// `thread_authority` should equal `thread.thread_authority`
    #[account(seeds = [THREAD_AUTHORITY_SEED], bump)]
    pub thread_authority: SystemAccount<'info>,
}

impl<'info> Increment<'info> {
    pub fn increment(&mut self) -> Result<()> {
        self.dummy_account.counter += 1;

        msg!("user counter incremented to {:?}", self.dummy_account.counter);
        
        Ok(())
    }
}