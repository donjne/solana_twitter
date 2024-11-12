use anchor_lang::prelude::*;
use crate::constants::*;
use crate::errors::*;
use crate::events::*;
use crate::states::*;

#[derive(Accounts)]
#[instruction(content: String)]
pub struct AddComment<'info> {
    #[account(mut)]
    pub author: Signer<'info>,

    #[account(mut)]
    pub tweet: Account<'info, Tweet>,

    #[account(
        init,
        payer = author,
        space = Comment::INIT_SPACE,
        seeds = [
            COMMENT_SEED,
            tweet.key().as_ref(),
            author.key().as_ref(),
            content.as_bytes(),
        ],
        bump
    )]
    pub comment: Account<'info, Comment>,

    pub system_program: Program<'info, System>,
}

impl<'info> AddComment<'info> {
    pub fn add(&mut self, bump:u8, content: String) -> Result<()> {
        require!(
            content.as_bytes().len() <= MAX_CONTENT_LENGTH,
            TwitterError::ContentTooLong
        );

        let comment = &mut self.comment;
        comment.author = self.author.key();
        comment.tweet = self.tweet.key();
        comment.content = content;
        comment.bump = bump;

        let tweet = &mut self.tweet;
        tweet.comment_count = tweet.comment_count.checked_add(1).unwrap();

        emit!(CommentAdded {
            comment: comment.key(),
            tweet: tweet.key(),
            author: self.author.key(),
            timestamp: Clock::get()?.unix_timestamp,
        });

        Ok(())
    }
}