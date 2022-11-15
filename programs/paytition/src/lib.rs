
use anchor_lang::prelude::*;
mod types;
mod custom_errors;
mod requests;

use requests::*;

declare_id!("BWYpP9uLHVTcEDDbHJKLdTX6ePM1VpzUoUjcK6oKUzyt");


#[program]
pub mod paytition {
    use super::*;

    pub fn create_petition(
        ctx: Context<CreatePetition>,
        topic: String,
        link: String,
        goal: u64,
        vote_cost: u32
        ) -> Result<()>{
        let petition = &mut ctx.accounts.petition;
        // The creator of the petition
        petition.author = ctx.accounts.author.key();
        // A small note about the topic
        require!(topic.len() <= 50, custom_errors::PetitionError::StringSizeOverflow);
        petition.topic = topic;

        // A link to the said issue
        require!(link.len() <= 50, custom_errors::PetitionError::StringSizeOverflow);
        petition.link = link;

        // Goal for the number of votes required
        petition.goal = goal;
        
        // vote buy-in cost in lamports
        petition.vote_lamport = vote_cost;
        Ok(())
    }

    pub fn add_vote(ctx: Context<AddVote>, bump: u8, consent_lamports: u32) -> Result<()> {
        let petition = &mut ctx.accounts.petition;
        require!(petition.vote_lamport == consent_lamports, custom_errors::PetitionError::ConsentError);

        transfer_from_to(&ctx.accounts.voter, bump, petition, petition.vote_lamport)?;
        petition.count = petition.count.checked_add(1).unwrap();


        Ok(())
    }

    pub fn finish_petition(_ctx: Context<ClosePetition>) -> Result<()> {



        Ok(())
    }
}


pub fn transfer_from_to<'a, U>(from_account: &Signer<'a>, from_bump: u8, to_account: &mut Account<'a, U>, amount: u32) -> Result<()> 
where
    U: Clone + anchor_lang::Owner + anchor_lang::AccountSerialize + anchor_lang::AccountDeserialize
{
    let from_key = from_account.key();
    let to_key = to_account.key();

    let bump_sol_vector = from_bump.to_le_bytes();
    let inner = vec![from_key.as_ref(), bump_sol_vector.as_ref()];
    let outer_sol = vec![inner.as_slice()];

    let ix = anchor_lang::solana_program::system_instruction::transfer(&from_key, &to_key, amount.into());

    anchor_lang::solana_program::program::invoke_signed(&ix, 
                                                        &[from_account.to_account_info(), to_account.to_account_info()], 
                                                        outer_sol.as_slice())?;

    Ok(())
}
