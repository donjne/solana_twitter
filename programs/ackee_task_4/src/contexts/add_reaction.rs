use anchor_lang::prelude::*;
use crate::constants::*;
use crate::events::*;
use crate::states::*;

#[derive(Accounts)]
pub struct AddReaction<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(mut)]
    pub tweet: Account<'info, Tweet>,

    #[account(
        init,
        payer = user,
        space = Reaction::INIT_SPACE,
        seeds = [
            REACTION_SEED,
            tweet.key().as_ref(),
            user.key().as_ref(),
        ],
        bump
    )]
    pub reaction: Account<'info, Reaction>,

    pub system_program: Program<'info, System>,
}

impl<'info> AddReaction<'info> {
    pub fn add(&mut self, bump: u8) -> Result<()> {
        let reaction = &mut self.reaction;
        reaction.user = self.user.key();
        reaction.tweet = self.tweet.key();
        reaction.bump = bump;

        let tweet = &mut self.tweet;
        tweet.reaction_count = tweet.reaction_count.checked_add(1).unwrap();

        emit!(ReactionAdded {
            tweet: tweet.key(),
            user: self.user.key(),
            timestamp: Clock::get()?.unix_timestamp,
        });

        Ok(())
    }
}