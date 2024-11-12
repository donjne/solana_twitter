use anchor_lang::prelude::*;
use crate::constants::*;

#[account]
#[derive(InitSpace)]
pub struct Tweet {
    pub author: Pubkey,
    #[max_len(MAX_TOPIC_LENGTH)]
    pub topic: String,
    #[max_len(MAX_CONTENT_LENGTH)]
    pub content: String,
    pub reaction_count: u64,
    pub comment_count: u64,
    pub bump: u8,
}