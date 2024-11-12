// tests/solana_twitter.rs
use anchor_lang::prelude::*;
use anchor_lang::solana_program::system_program;
use solana_program_test::*;
use solana_sdk::{signature::Keypair, signer::Signer};
use solana_twitter::states::Tweet;
use solana_twitter::TWEET_SEED;

#[tokio::test]
async fn test_create_tweet() {
    let program = ProgramTest::new(
        "solana_twitter",
        solana_twitter::ID,
        processor!(solana_twitter::entry),
    );

    let mut context = program.start_with_context().await;
    let payer = Keypair::new();

    // Airdrop SOL to payer
    context.banks_client
        .process_transaction(Transaction::new_signed_with_payer(
            &[system_instruction::transfer(
                &context.payer.pubkey(),
                &payer.pubkey(),
                1_000_000_000,
            )],
            Some(&context.payer.pubkey()),
            &[&context.payer],
            context.last_blockhash,
        ))
        .await
        .unwrap();

    // Test data
    let topic = String::from("solana");
    let content = String::from("This is a test tweet about Solana!");

    // Find PDA for tweet
    let (tweet_pda, _bump) = Pubkey::find_program_address(
        &[
            TWEET_SEED,
            payer.pubkey().as_ref(),
            topic.as_bytes(),
            content.as_bytes(),
        ],
        &solana_twitter::ID,
    );

    // Create tweet
    let ix = solana_twitter::instruction::create_tweet {
        topic: topic.clone(),
        content: content.clone(),
    };

    let accounts = solana_twitter::accounts::CreateTweet {
        author: payer.pubkey(),
        tweet: tweet_pda,
        system_program: system_program::ID,
    };

    let mut tx = Transaction::new_with_payer(
        &[ix.accounts(accounts).instruction()],
        Some(&payer.pubkey()),
    );
    tx.sign(&[&payer], context.last_blockhash);

    context.banks_client.process_transaction(tx).await.unwrap();

    // Verify tweet account data
    let tweet_account = context
        .banks_client
        .get_account(tweet_pda)
        .await
        .unwrap()
        .unwrap();
    
    let tweet = Tweet::try_deserialize(&mut tweet_account.data.as_ref()).unwrap();
    assert_eq!(tweet.author, payer.pubkey());
    assert_eq!(tweet.topic, topic);
    assert_eq!(tweet.content, content);
    assert_eq!(tweet.reaction_count, 0);
    assert_eq!(tweet.comment_count, 0);
}