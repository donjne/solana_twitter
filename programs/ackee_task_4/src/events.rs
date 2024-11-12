use anchor_lang::prelude::*;

#[event]
pub struct TweetCreated {
    pub tweet: Pubkey,
    pub author: Pubkey,
    pub topic: String,
    pub timestamp: i64,
}

#[event]
pub struct TweetDeleted {
    pub tweet: Pubkey,
    pub author: Pubkey,
    pub timestamp: i64,
}

#[event]
pub struct ReactionAdded {
    pub tweet: Pubkey,
    pub user: Pubkey,
    pub timestamp: i64,
}

#[event]
pub struct ReactionRemoved {
    pub tweet: Pubkey,
    pub user: Pubkey,
    pub timestamp: i64,
}

#[event]
pub struct CommentAdded {
    pub comment: Pubkey,
    pub tweet: Pubkey,
    pub author: Pubkey,
    pub timestamp: i64,
}

#[event]
pub struct CommentRemoved {
    pub comment: Pubkey,
    pub tweet: Pubkey,
    pub author: Pubkey,
    pub timestamp: i64,
}