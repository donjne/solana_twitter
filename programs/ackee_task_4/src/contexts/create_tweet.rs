use anchor_lang::prelude::*;
use crate::constants::*;
use crate::errors::*;
use crate::events::*;
use crate::states::*;

#[derive(Accounts)]
#[instruction(topic: String, content: String)]
pub struct CreateTweet<'info> {
    #[account(mut)]
    pub author: Signer<'info>,

    #[account(
        init,
        payer = author,
        space = Tweet::INIT_SPACE,
        seeds = [
            TWEET_SEED,
            author.key().as_ref(),
            topic.as_bytes(),
            content.as_bytes()
        ],
        bump
    )]
    pub tweet: Account<'info, Tweet>,

    pub system_program: Program<'info, System>,
}

impl<'info> CreateTweet<'info> {
    pub fn create(&mut self, bump: u8, topic: String, content: String) -> Result<()> {
        require!(
            topic.as_bytes().len() <= MAX_TOPIC_LENGTH,
            TwitterError::TopicTooLong
        );
        require!(
            content.as_bytes().len() <= MAX_CONTENT_LENGTH,
            TwitterError::ContentTooLong
        );

        let tweet = &mut self.tweet;
        tweet.author = self.author.key();
        tweet.topic = topic.clone();
        tweet.content = content;
        tweet.reaction_count = 0;
        tweet.comment_count = 0;
        tweet.bump = bump;

        emit!(TweetCreated {
            tweet: tweet.key(),
            author: self.author.key(),
            topic,
            timestamp: Clock::get()?.unix_timestamp,
        });

        Ok(())
    }
}