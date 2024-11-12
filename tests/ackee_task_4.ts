import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { AckeeTask4 } from "../target/types/ackee_task_4";
import { PublicKey, Keypair, SystemProgram } from '@solana/web3.js';
import { expect } from 'chai';

describe("solana-twitter", () => {

  const TWEET_SEED = "tweet";
  const REACTION_SEED = "reaction";
  const COMMENT_SEED = "comment";

  // Configure the client to use the local cluster
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.SolanaTwitter as Program<AckeeTask4>;
  
  // Test accounts
  const author = anchor.web3.Keypair.fromSecretKey(
    Buffer.from(JSON.parse(require('fs').readFileSync('~/.config/solana/id.json', 'utf-8')))
  );
  const reactor = Keypair.generate();
  const commenter = Keypair.generate();

  // Test data
  const topic = "solana";
  const content = "This is a test tweet about Solana!";
  const commentContent = "This is a test comment!";

  before(async () => {
    // Airdrop SOL to test accounts
    const airdropAuthors = [author, reactor, commenter].map(
      async (kp) => 
        await provider.connection.requestAirdrop(
          kp.publicKey, 
          2 * anchor.web3.LAMPORTS_PER_SOL
        )
    );

    await Promise.all(airdropAuthors);
  });

  it("Creates a tweet", async () => {
    // Derive the tweet PDA
    const [tweetPDA] = await PublicKey.findProgramAddress(
      [
        Buffer.from(TWEET_SEED),
        author.publicKey.toBuffer(),
        Buffer.from(topic),
        Buffer.from(content),
      ],
      program.programId
    );

    // Create tweet
    await program.methods
      .createTweet(topic, content)
      .accounts({
        author: author.publicKey,
        tweet: tweetPDA,
        systemProgram: SystemProgram.programId,
      })
      .signers([author])
      .rpc();

    // Fetch the created tweet account
    const tweet = await program.account.tweet.fetch(tweetPDA);

    // Verify tweet data
    expect(tweet.author.toString()).to.equal(author.publicKey.toString());
    expect(tweet.topic).to.equal(topic);
    expect(tweet.content).to.equal(content);
    expect(tweet.reactionCount.toNumber()).to.equal(0);
    expect(tweet.commentCount.toNumber()).to.equal(0);

    // Store tweet PDA for later tests
    return { tweetPDA };
  });

  it("Adds a reaction to tweet", async () => {
    const { tweetPDA } = await program.account.tweet.all()
      .then(tweets => ({ tweetPDA: tweets[0].publicKey }));

    // Derive the reaction PDA
    const [reactionPDA] = await PublicKey.findProgramAddress(
      [
        Buffer.from(REACTION_SEED),
        tweetPDA.toBuffer(),
        reactor.publicKey.toBuffer(),
      ],
      program.programId
    );

    // Add reaction
    await program.methods
      .addReaction()
      .accounts({
        user: reactor.publicKey,
        tweet: tweetPDA,
        reaction: reactionPDA,
        systemProgram: SystemProgram.programId,
      })
      .signers([reactor])
      .rpc();

    // Verify reaction account
    const reaction = await program.account.reaction.fetch(reactionPDA);
    expect(reaction.user.toString()).to.equal(reactor.publicKey.toString());
    expect(reaction.tweet.toString()).to.equal(tweetPDA.toString());

    // Verify tweet reaction count
    const tweet = await program.account.tweet.fetch(tweetPDA);
    expect(tweet.reactionCount.toNumber()).to.equal(1);

    return { tweetPDA, reactionPDA };
  });

  it("Removes a reaction from tweet", async () => {
    const { tweetPDA, reactionPDA } = await program.account.reaction.all()
      .then(reactions => ({
        tweetPDA: reactions[0].account.tweet,
        reactionPDA: reactions[0].publicKey
      }));

    // Remove reaction
    await program.methods
      .removeReaction()
      .accounts({
        user: reactor.publicKey,
        tweet: tweetPDA,
        reaction: reactionPDA,
      })
      .signers([reactor])
      .rpc();

    // Verify reaction account is closed
    const reactionAccount = await provider.connection.getAccountInfo(reactionPDA);
    expect(reactionAccount).to.be.null;

    // Verify tweet reaction count
    const tweet = await program.account.tweet.fetch(tweetPDA);
    expect(tweet.reactionCount.toNumber()).to.equal(0);
  });

  it("Adds a comment to tweet", async () => {
    const { tweetPDA } = await program.account.tweet.all()
      .then(tweets => ({ tweetPDA: tweets[0].publicKey }));

    // Derive the comment PDA
    const [commentPDA] = await PublicKey.findProgramAddress(
      [
        Buffer.from(COMMENT_SEED),
        tweetPDA.toBuffer(),
        commenter.publicKey.toBuffer(),
        Buffer.from(commentContent),
      ],
      program.programId
    );

    // Add comment
    await program.methods
      .addComment(commentContent)
      .accounts({
        author: commenter.publicKey,
        tweet: tweetPDA,
        comment: commentPDA,
        systemProgram: SystemProgram.programId,
      })
      .signers([commenter])
      .rpc();

    // Verify comment account
    const comment = await program.account.comment.fetch(commentPDA);
    expect(comment.author.toString()).to.equal(commenter.publicKey.toString());
    expect(comment.tweet.toString()).to.equal(tweetPDA.toString());
    expect(comment.content).to.equal(commentContent);

    // Verify tweet comment count
    const tweet = await program.account.tweet.fetch(tweetPDA);
    expect(tweet.commentCount.toNumber()).to.equal(1);

    return { tweetPDA, commentPDA };
  });

  it("Removes a comment from tweet", async () => {
    const { tweetPDA, commentPDA } = await program.account.comment.all()
      .then(comments => ({
        tweetPDA: comments[0].account.tweet,
        commentPDA: comments[0].publicKey
      }));

    // Remove comment
    await program.methods
      .removeComment()
      .accounts({
        author: commenter.publicKey,
        tweet: tweetPDA,
        comment: commentPDA,
      })
      .signers([commenter])
      .rpc();

    // Verify comment account is closed
    const commentAccount = await provider.connection.getAccountInfo(commentPDA);
    expect(commentAccount).to.be.null;

    // Verify tweet comment count
    const tweet = await program.account.tweet.fetch(tweetPDA);
    expect(tweet.commentCount.toNumber()).to.equal(0);
  });

  it("Deletes a tweet", async () => {
    const { tweetPDA } = await program.account.tweet.all()
      .then(tweets => ({ tweetPDA: tweets[0].publicKey }));

    // Delete tweet
    await program.methods
      .deleteTweet()
      .accounts({
        author: author.publicKey,
        tweet: tweetPDA,
      })
      .signers([author])
      .rpc();

    // Verify tweet account is closed
    const tweetAccount = await provider.connection.getAccountInfo(tweetPDA);
    expect(tweetAccount).to.be.null;
  });

  // Error cases
  it("Cannot create tweet with too long topic", async () => {
    const longTopic = "a".repeat(33); // Max is 32
    const [tweetPDA] = await PublicKey.findProgramAddress(
      [
        Buffer.from(TWEET_SEED),
        author.publicKey.toBuffer(),
        Buffer.from(longTopic),
        Buffer.from(content),
      ],
      program.programId
    );

    try {
      await program.methods
        .createTweet(longTopic, content)
        .accounts({
          author: author.publicKey,
          tweet: tweetPDA,
          systemProgram: SystemProgram.programId,
        })
        .signers([author])
        .rpc();
      expect.fail("Expected transaction to fail");
    } catch (error) {
      expect(error).to.be.instanceOf(Error);
      expect(error.toString()).to.include("TopicTooLong");
    }
  });

  it("Cannot create tweet with too long content", async () => {
    const longContent = "a".repeat(501); // Max is 500
    const [tweetPDA] = await PublicKey.findProgramAddress(
      [
        Buffer.from(TWEET_SEED),
        author.publicKey.toBuffer(),
        Buffer.from(topic),
        Buffer.from(longContent),
      ],
      program.programId
    );

    try {
      await program.methods
        .createTweet(topic, longContent)
        .accounts({
          author: author.publicKey,
          tweet: tweetPDA,
          systemProgram: SystemProgram.programId,
        })
        .signers([author])
        .rpc();
      expect.fail("Expected transaction to fail");
    } catch (error) {
      expect(error).to.be.instanceOf(Error);
      expect(error.toString()).to.include("ContentTooLong");
    }
  });

  it("Cannot add duplicate reaction", async () => {
    // First create a tweet
    const [tweetPDA] = await PublicKey.findProgramAddress(
      [
        Buffer.from(TWEET_SEED),
        author.publicKey.toBuffer(),
        Buffer.from(topic),
        Buffer.from(content),
      ],
      program.programId
    );

    await program.methods
      .createTweet(topic, content)
      .accounts({
        author: author.publicKey,
        tweet: tweetPDA,
        systemProgram: SystemProgram.programId,
      })
      .signers([author])
      .rpc();

    // Add first reaction
    const [reactionPDA] = await PublicKey.findProgramAddress(
      [
        Buffer.from(REACTION_SEED),
        tweetPDA.toBuffer(),
        reactor.publicKey.toBuffer(),
      ],
      program.programId
    );

    await program.methods
      .addReaction()
      .accounts({
        user: reactor.publicKey,
        tweet: tweetPDA,
        reaction: reactionPDA,
        systemProgram: SystemProgram.programId,
      })
      .signers([reactor])
      .rpc();

    // Try to add duplicate reaction
    try {
      await program.methods
        .addReaction()
        .accounts({
          user: reactor.publicKey,
          tweet: tweetPDA,
          reaction: reactionPDA,
          systemProgram: SystemProgram.programId,
        })
        .signers([reactor])
        .rpc();
      expect.fail("Expected transaction to fail");
    } catch (error) {
      expect(error).to.be.instanceOf(Error);
    }
  });
});