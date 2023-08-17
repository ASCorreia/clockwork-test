use anchor_lang::prelude::*;

declare_id!("CtoKnpCU5pDfKiXsAD3mz73BThpZC9fzxCKkBsActXuf");

pub mod contexts;

pub use contexts::*;

/// Seed for deriving the `Counter` account PDA.
pub const SEED_COUNTER: &[u8] = b"Dummyy";

/// Seed for thread_authority pda
/// ⚠️ Make sure it matches whatever you are using on the client-side
pub const THREAD_AUTHORITY_SEED: &[u8] = b"authority";

#[program]
pub mod clockwork_test {

    use super::*;

    pub fn initialize(ctx: Context<Initialize>, thread_id: Vec<u8>) -> Result<()> {
        ctx.accounts.dummy_account.bump = *ctx.bumps.get("dummy_account").unwrap();
        ctx.accounts.dummy_account.thread_bump = *ctx.bumps.get("thread_authority").unwrap();

        ctx.accounts.initialize(thread_id)?;

        msg!("Dummy Account Initialized");

        Ok(())
    }

    pub fn increment(ctx: Context<Increment>) -> Result<()> {
        ctx.accounts.increment()?;
        
        Ok(())
    }

    pub fn pause(ctx: Context<Pause>) -> Result<()> {
        ctx.accounts.pause()?;
        
        Ok(())
    }

    pub fn delete(ctx: Context<Delete>) -> Result<()> {
        ctx.accounts.delete()?;
        
        Ok(())
    }
}
