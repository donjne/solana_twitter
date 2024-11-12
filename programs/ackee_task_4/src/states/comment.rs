use anchor_lang::prelude::*;
use crate::constants::*;

#[account]
#[derive(InitSpace)]
pub struct Comment {
    pub author: Pubkey,
    pub tweet: Pubkey,
    #[max_len(MAX_CONTENT_LENGTH)]
    pub content: String,
    pub bump: u8,
}
