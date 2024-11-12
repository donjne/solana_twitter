use anchor_lang::prelude::*;
use crate::constants::*;
use crate::events::*;
use crate::states::*;

#[derive(Accounts)]
pub struct RemoveReaction<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(mut)]
    pub tweet: Account<'info, Tweet>,

    #[account(
        mut,
        close = user,
        seeds = [
            REACTION_SEED,
            tweet.key().as_ref(),
            user.key().as_ref(),
        ],
        bump = reaction.bump,
    )]
    pub reaction: Account<'info, Reaction>,
}

impl<'info> RemoveReaction<'info> {
    pub fn remove(&mut self) -> Result<()> {
        let tweet = &mut self.tweet;
        tweet.reaction_count = tweet.reaction_count.checked_sub(1).unwrap();

        emit!(ReactionRemoved {
            tweet: tweet.key(),
            user: self.user.key(),
            timestamp: Clock::get()?.unix_timestamp,
        });

        Ok(())
    }
}
