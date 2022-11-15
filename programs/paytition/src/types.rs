use anchor_lang::prelude::*;


pub const P_TAG: &[u8] = b"PETITION_";

#[account]
#[derive(Default)]
pub struct Petition {
    pub author: Pubkey,
    pub topic: String,
    pub link: String,
    pub goal: u64,
    pub timestamp: i64,
    pub count: u64,
    pub vote_lamport: u32
}
