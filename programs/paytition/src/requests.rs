use anchor_lang::prelude::*;
use crate::types::{self, P_TAG};

#[derive(Accounts)]
#[instruction()]
pub struct CreatePetition<'info> {
    #[account(mut)]
    pub author: Signer<'info>,

    #[account(
        init,
        seeds = [P_TAG, author.key().as_ref()],
        bump,
        payer = author,
        space = 8 + std::mem::size_of::<types::Petition>(),
             )]
        pub petition: Box<Account<'info, types::Petition>>,

        pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction()]
pub struct AddVote<'info> {
    #[account(mut)]
    pub voter: Signer<'info>,

    #[account(
        mut,
        seeds = [P_TAG],
        bump,
             )]
    pub petition: Box<Account<'info, types::Petition>>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction()]
pub struct ClosePetition<'info> {
    #[account(
        mut,
        close = author,
        seeds = [P_TAG, author.key().as_ref()],
        bump,
        has_one = author,
             )]
    pub petition: Box<Account<'info, types::Petition>>,

    #[account(mut)]
    pub author: Signer<'info>,

    pub system_program: Program<'info, System>,
}

pub fn bump(seeds: &[&[u8]], program_id: &Pubkey) -> u8 {
    let (_found_key, bump) = Pubkey::find_program_address(seeds, program_id);
    bump
}
