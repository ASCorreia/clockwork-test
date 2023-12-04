use crate::*;

#[account]
pub struct DummyAccount {
    pub counter: u32,
    pub bump: u8,
    pub thread_bump: u8,
}