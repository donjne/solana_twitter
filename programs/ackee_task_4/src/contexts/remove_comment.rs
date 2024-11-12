use anchor_lang::prelude::*;
use crate::constants::*;
use crate::events::*;
use crate::states::*;

#[derive(Accounts)]
pub struct RemoveComment<'info> {
    #[account(mut)]
    pub author: Signer<'info>,

    #[account(mut)]
    pub tweet: Account<'info, Tweet>,

    #[account(
        mut,
        close = author,
        seeds = [
           COMMENT_SEED,
            tweet.key().as_ref(),
            author.key().as_ref(),
            comment.content.as_bytes(),
        ],
        bump = comment.bump,
        has_one = author,
        has_one = tweet,
    )]
    pub comment: Account<'info, Comment>,
}

impl<'info> RemoveComment<'info> {
    pub fn remove(&mut self) -> Result<()> {
        let tweet = &mut self.tweet;
        tweet.comment_count = tweet.comment_count.checked_sub(1).unwrap();

        emit!(CommentRemoved {
            comment: self.comment.key(),
            tweet: tweet.key(),
            author: self.author.key(),
            timestamp: Clock::get()?.unix_timestamp,
        });

        Ok(())
    }
}