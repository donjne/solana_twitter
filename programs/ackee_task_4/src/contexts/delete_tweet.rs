use anchor_lang::prelude::*;
use crate::constants::*;
use crate::events::*;
use crate::states::*;

#[derive(Accounts)]
pub struct DeleteTweet<'info> {
    #[account(mut)]
    pub author: Signer<'info>,

    #[account(
        mut,
        close = author,
        seeds = [
            TWEET_SEED,
            author.key().as_ref(),
            tweet.topic.as_bytes(),
            tweet.content.as_bytes()
        ],
        bump = tweet.bump,
        has_one = author
    )]
    pub tweet: Account<'info, Tweet>,
}

impl<'info> DeleteTweet<'info> {
    pub fn delete(&mut self) -> Result<()> {
        emit!(TweetDeleted {
            tweet: self.tweet.key(),
            author: self.author.key(),
            timestamp: Clock::get()?.unix_timestamp,
        });

        Ok(())
    }
}