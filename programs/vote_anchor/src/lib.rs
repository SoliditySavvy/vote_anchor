use anchor_lang::prelude::*;

declare_id!("8V4jC5dERowrER8qjFgPgVks6HiqC7vvuoba9k4ofKyC");

#[program]
pub mod vote_anchor {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, _hash: Vec<u8>) -> Result<()> {
        ctx.accounts.vote.votes = 0;
        ctx.accounts.vote.bump = *ctx.bumps.get("vote").unwrap();
        Ok(())
    }
    pub fn upvote(ctx: Context<VoteInteraction>, _hash: Vec<u8>) -> Result<()> {
        ctx.accounts.vote.votes = ctx.accounts.vote.votes + 1;
        Ok(())
    }
    pub fn downvote(ctx: Context<VoteInteraction>, _hash: Vec<u8>) -> Result<()> {
        ctx.accounts.vote.votes = ctx.accounts.vote.votes - 1;
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(_hash: Vec<u8>)]
pub struct Initialize<'info> {
    #[account(mut)]
    signer: Signer<'info>,
    #[account(
        init,
        payer = signer,
        space = Vote::LEN,
        seeds = [b"vote", _hash.as_slice().as_ref()],
        bump
    )]
    vote: Account<'info, Vote>,
    system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(_hash: Vec<u8>)]
pub struct VoteInteraction<'info> {
    #[account(mut)]
    signer: Signer<'info>,
    #[account(
        mut,
        seeds = [b"vote", _hash.as_slice().as_ref()],
        bump = vote.bump
    )]
    vote: Account<'info, Vote>,
    system_program: Program<'info, System>,
}

#[account]
pub struct Vote {
    votes: i64,
    bump: u8,
}

impl Vote {
    const LEN: usize = 8 + 8 + 1;
}
