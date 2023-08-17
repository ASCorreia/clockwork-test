use anchor_lang::{system_program, InstructionData};
use clockwork_sdk::state::Thread;
use solana_program::{instruction::Instruction, native_token::LAMPORTS_PER_SOL};

use crate::*;

#[derive(Accounts)]
#[instruction(thread_id: Vec<u8>)]
pub struct Initialize<'info> {
    /// The counter account to initialize.
    #[account(
        init,
        payer = payer,
        seeds = [SEED_COUNTER],
        bump,
        space = 8 + std::mem::size_of::<DummyAccount> (),
    )]
    pub dummy_account: Account<'info, DummyAccount>,

    /// The signer who will pay to initialize the program.
    /// (not to be confused with the thread executions).
    #[account(mut)]
    pub payer: Signer<'info>,

    /// The Clockwork thread program.
    #[account(address = clockwork_sdk::ID)]
    pub clockwork_program: Program<'info, clockwork_sdk::ThreadProgram>,

    /// The Solana system program.
    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,

    /// Address to assign to the newly created thread.
    #[account(mut, address = Thread::pubkey(thread_authority.key(), thread_id))]
    pub thread: SystemAccount<'info>,

    /// The pda that will own and manage the thread.
    #[account(seeds = [THREAD_AUTHORITY_SEED], bump)]
    pub thread_authority: SystemAccount<'info>,
}

#[derive(Accounts)]
pub struct Pause<'info> {
    /// The Dummy Account.
    #[account(
        mut,
        seeds = [SEED_COUNTER],
        bump = dummy_account.bump,
    )]
    pub dummy_account: Account<'info, DummyAccount>,

    /// The Clockwork thread program.
    #[account(address = clockwork_sdk::ID)]
    pub clockwork_program: Program<'info, clockwork_sdk::ThreadProgram>,

    /// The Solana system program.
    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,

    /// Address of the created thread.
    #[account(mut)]
    pub thread: Account<'info, Thread>,

    /// The pda that owns and manages the thread.
    #[account(seeds = [THREAD_AUTHORITY_SEED], bump)]
    pub thread_authority: SystemAccount<'info>,
}

#[derive(Accounts)]
pub struct Delete<'info> {
    /// The Dummy Account.
    #[account(
        mut,
        seeds = [SEED_COUNTER],
        bump = dummy_account.bump,
    )]
    pub dummy_account: Account<'info, DummyAccount>,

    #[account(mut)]
    pub user: SystemAccount<'info>, //Signer ??

    /// The Clockwork thread program.
    #[account(address = clockwork_sdk::ID)]
    pub clockwork_program: Program<'info, clockwork_sdk::ThreadProgram>,

    /// The thread to reset.
    #[account(mut, constraint = thread.authority.eq(&thread_authority.key()))]
    pub thread: Account<'info, Thread>,

    /// The pda that owns and manages the thread.
    #[account(seeds = [THREAD_AUTHORITY_SEED], bump)]
    pub thread_authority: SystemAccount<'info>,
}

impl<'info> Initialize<'info> {
    pub fn initialize(&mut self, thread_id: Vec<u8>) -> Result<()> {
        self.dummy_account.counter = 0;
        
        // Get accounts.
        let system_program = &self.system_program;
        let clockwork_program = &self.clockwork_program;
        let payer = &self.payer;
        let thread = &self.thread;
        let thread_authority = &self.thread_authority;
        let dummy_account = &mut self.dummy_account;
    
        // 1️⃣ Prepare an instruction to automate. 
        //    In this case, we will automate the Increment instruction.
        let target_ix = Instruction {
            program_id: ID,
            accounts: crate::accounts::Increment {
                dummy_account: dummy_account.key(),
                thread: thread.key(),
                thread_authority: thread_authority.key(),
            }
            .to_account_metas(Some(true)),
            data: crate::instruction::Increment {}.data(),
        };
    
        // 2️⃣ Define a trigger for the thread.
        let trigger = clockwork_sdk::state::Trigger::Cron {
            schedule: "*/10 * * * * * *".into(),
            skippable: true,
        };
    
        // 3️⃣ Create a Thread via CPI
        clockwork_sdk::cpi::thread_create(
            CpiContext::new_with_signer(
                clockwork_program.to_account_info(),
                clockwork_sdk::cpi::ThreadCreate {
                    payer: payer.to_account_info(),
                    system_program: system_program.to_account_info(),
                    thread: thread.to_account_info(),
                    authority: thread_authority.to_account_info(),
                },
                &[&[THREAD_AUTHORITY_SEED, &[self.dummy_account.thread_bump]]],
            ),
            LAMPORTS_PER_SOL,       // amount
            thread_id,              // id
            vec![target_ix.into()], // instructions
            trigger,                // trigger
        )?;
    
        Ok(())
    }
}

impl<'info> Pause<'info> {
    pub fn pause(&mut self) -> Result<()> {
        let clockwork_program = &self.clockwork_program;
        let thread = &mut self.thread;
        let thread_authority = &mut self.thread_authority;

        // 3️⃣ Pause thread via CPI.
        clockwork_sdk::cpi::thread_pause(
            CpiContext::new_with_signer(
                clockwork_program.to_account_info(),
                clockwork_sdk::cpi::ThreadPause {
                    thread: thread.to_account_info(),
                    authority: thread_authority.to_account_info(),
                },
                &[&[THREAD_AUTHORITY_SEED, &[self.dummy_account.thread_bump]]],
            ),
        )?;

        Ok(())
    }
}

impl<'info> Delete<'info> {
    pub fn delete(&mut self) -> Result<()> {
        let clockwork_program = &self.clockwork_program;
        let user = &self.user;
        let thread = &self.thread;
        let thread_authority = &self.thread_authority;

        // Delete thread via CPI.
        clockwork_sdk::cpi::thread_delete(CpiContext::new_with_signer(
            clockwork_program.to_account_info(),
            clockwork_sdk::cpi::ThreadDelete {
                authority: thread_authority.to_account_info(),
                close_to: user.to_account_info(),
                thread: thread.to_account_info(),
            },
            &[&[THREAD_AUTHORITY_SEED, &[self.dummy_account.thread_bump]]],
        ))?;
                
        Ok(())
    }
}

#[account]
pub struct DummyAccount {
    pub counter: u32,
    pub bump: u8,
    pub thread_bump: u8,
}