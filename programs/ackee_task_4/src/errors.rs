use anchor_lang::prelude::*;

#[error_code]
pub enum TwitterError {
    #[msg("The provided topic should be 32 bytes maximum")]
    TopicTooLong,
    #[msg("The provided content should be 500 bytes maximum")]
    ContentTooLong,
}