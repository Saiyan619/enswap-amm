import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { EnswapAmm } from "../target/types/enswap_amm";
import {TOKEN_PROGRAM_ID, createMint, getOrCreateAssociatedTokenAccount, mintTo, getAccount, getMint} from "@solana/spl-token";
import { SYSTEM_PROGRAM_ID } from "@coral-xyz/anchor/dist/cjs/native/system";
import { expect } from "chai";
describe("enswap-amm", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.enswapAmm as Program<EnswapAmm>;

  let mintA: anchor.web3.PublicKey;
  let mintB: anchor.web3.PublicKey;
  let pool: anchor.web3.PublicKey;
  let poolBump: number;
  let lpMint: anchor.web3.PublicKey;
  let lpMintBump: number;
  let tokenReserveA: anchor.web3.PublicKey;
  let tokenReserveABump: number;
  let tokenReserveB: anchor.web3.PublicKey;
  let tokenReserveBBump: number;
  let poolAuthority: anchor.web3.PublicKey;
  let poolAuthorityBump: number;
  let signer: anchor.web3.PublicKey;
  const fee = 30; //0.3%;

  it("Is initialized!", async () => {
    // Create two mints
    mintA = await createMint(
      provider.connection,      // Connection
      provider.wallet.payer,    // Payer (the default test validator wallet)
      provider.wallet.publicKey, // Mint Authority
      null,                      // Freeze Authority
      6                          // Decimals
    );
     mintB = await createMint(
      provider.connection,      // Connection
      provider.wallet.payer,    // Payer (the default test validator wallet)
      provider.wallet.publicKey, // Mint Authority
      null,                      // Freeze Authority
      6                          // Decimals
    );
    
    console.log(`Created Mint A: ${mintA.toBase58()}`);
    console.log(`Created Mint B: ${mintB.toBase58()}`);

    // Derive Pool PDA
    [pool, poolBump] = await anchor.web3.PublicKey.findProgramAddress(
      [Buffer.from("pool"), mintA.toBuffer(), mintB.toBuffer()],
      program.programId
    );
        
    // Derive LP Mint PDA
    [lpMint, lpMintBump] = await anchor.web3.PublicKey.findProgramAddress(
      [Buffer.from("lp_mint"), pool.toBuffer()],
      program.programId
    );

    // Derive Token Reserve A PDA
    [tokenReserveA, tokenReserveABump] = await anchor.web3.PublicKey.findProgramAddress(
      [Buffer.from("reserve_a"), pool.toBuffer()],
      program.programId
    );

    // Derive Token Reserve B PDA
    [tokenReserveB, tokenReserveBBump] = await anchor.web3.PublicKey.findProgramAddress(
      [Buffer.from("reserve_b"), pool.toBuffer()],
      program.programId
    );

    // Derive Pool Authority PDA
    [poolAuthority, poolAuthorityBump] = await anchor.web3.PublicKey.findProgramAddress(
      [Buffer.from("authority"), pool.toBuffer()],
      program.programId
    );

    // Assign signer
    signer = provider.wallet.publicKey;

    // Add your test here.
    const tx = await program.methods.initializePool(fee).accounts({
      mintA: mintA,
      mintB: mintB,
      pool: pool,
      lpMint: lpMint,
      tokenReserveA: tokenReserveA,
      tokenReserveB: tokenReserveB,
      poolAuthority: poolAuthority,
      signer: signer,
      systemProgram: SYSTEM_PROGRAM_ID,
      tokenProgram: TOKEN_PROGRAM_ID
    }).rpc();
    console.log("Your transaction signature", tx);

    // Verify Transcation
    const poolAccountData = await program.account.pool.fetch(pool);
    const lpMintData = await getMint(provider.connection, lpMint);
    const tokenReserveAData = await getAccount(provider.connection, tokenReserveA);
    const tokenReserveBData = await getAccount(provider.connection, tokenReserveB);


    expect(poolAccountData.feeBps).to.equal(fee);
    expect(poolAccountData.mintA.toBase58()).to.equal(mintA.toBase58());
    expect(poolAccountData.mintB.toBase58()).to.equal(mintB.toBase58());
    expect(poolAccountData.tokenReserveA.toBase58()).to.equal(tokenReserveA.toBase58())
    expect(poolAccountData.tokenReserveB.toBase58()).to.equal(tokenReserveB.toBase58())
    expect(poolAccountData.signAuthorityBump).to.equal(poolAuthorityBump)
expect(Number(lpMintData.supply)).to.equal(0);
expect(Number(tokenReserveAData.amount)).to.equal(0);
expect(Number(tokenReserveBData.amount)).to.equal(0);
expect(tokenReserveAData.owner.toBase58()).to.equal(poolAuthority.toBase58());
expect(tokenReserveBData.owner.toBase58()).to.equal(poolAuthority.toBase58());
expect(tokenReserveAData.mint.toBase58()).to.equal(mintA.toBase58());
expect(tokenReserveBData.mint.toBase58()).to.equal(mintB.toBase58());  });

it("it fails when fee is too high", async()=>{
     let mintA: anchor.web3.PublicKey;
  let mintB: anchor.web3.PublicKey;
  let pool: anchor.web3.PublicKey;
  let poolBump: number;
  let lpMint: anchor.web3.PublicKey;
  let lpMintBump: number;
  let tokenReserveA: anchor.web3.PublicKey;
  let tokenReserveABump: number;
  let tokenReserveB: anchor.web3.PublicKey;
  let tokenReserveBBump: number;
  let poolAuthority: anchor.web3.PublicKey;
  let poolAuthorityBump: number;
  let signer: anchor.web3.PublicKey;
  const fee = 30; //0.3%;

     // Create two mints
    mintA = await createMint(
      provider.connection,      // Connection
      provider.wallet.payer,    // Payer (the default test validator wallet)
      provider.wallet.publicKey, // Mint Authority
      null,                      // Freeze Authority
      6                          // Decimals
    );
     mintB = await createMint(
      provider.connection,      // Connection
      provider.wallet.payer,    // Payer (the default test validator wallet)
      provider.wallet.publicKey, // Mint Authority
      null,                      // Freeze Authority
      6                          // Decimals
    );
    
    console.log(`Created Mint A: ${mintA.toBase58()}`);
    console.log(`Created Mint B: ${mintB.toBase58()}`);

    // Derive Pool PDA
    [pool, poolBump] = await anchor.web3.PublicKey.findProgramAddress(
      [Buffer.from("pool"), mintA.toBuffer(), mintB.toBuffer()],
      program.programId
    );
        
    // Derive LP Mint PDA
    [lpMint, lpMintBump] = await anchor.web3.PublicKey.findProgramAddress(
      [Buffer.from("lp_mint"), pool.toBuffer()],
      program.programId
    );

    // Derive Token Reserve A PDA
    [tokenReserveA, tokenReserveABump] = await anchor.web3.PublicKey.findProgramAddress(
      [Buffer.from("reserve_a"), pool.toBuffer()],
      program.programId
    );

    // Derive Token Reserve B PDA
    [tokenReserveB, tokenReserveBBump] = await anchor.web3.PublicKey.findProgramAddress(
      [Buffer.from("reserve_b"), pool.toBuffer()],
      program.programId
    );

    // Derive Pool Authority PDA
    [poolAuthority, poolAuthorityBump] = await anchor.web3.PublicKey.findProgramAddress(
      [Buffer.from("authority"), pool.toBuffer()],
      program.programId
    );

    // Assign signer
    signer = provider.wallet.publicKey;
  //provide a fee greater than 10000 bps (100%) should fail
  const highFee = 10001
  try {
    const tx = await program.methods.initializePool(highFee).accounts({
      mintA: mintA,
      mintB: mintB,
      pool: pool,
      lpMint: lpMint,
      tokenReserveA: tokenReserveA,
      tokenReserveB: tokenReserveB,
      poolAuthority: poolAuthority,
      signer: signer,
      systemProgram: SYSTEM_PROGRAM_ID,
      tokenProgram: TOKEN_PROGRAM_ID
    }).rpc();
    expect.fail("initaliation should fail with high fee");
  } catch (error) {
    expect(error).to.exist;
    console.log("should fail with high fee:", error);
  }
})

it("initialize fails when same pool is re-initialized", async () => {

  try {
    const tx = await program.methods
      .initializePool(fee)
      .accounts({
        mintA: mintA,
        mintB: mintB,
        pool: pool,
        lpMint: lpMint,
        tokenReserveA: tokenReserveA,
        tokenReserveB: tokenReserveB,
        poolAuthority: poolAuthority,
        signer: signer,
        systemProgram: SYSTEM_PROGRAM_ID,
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      .rpc();
    expect.fail("Re-initialization should fail");
  } catch (error) {
    expect(error).to.exist;
    console.log("Re-initialzation failed as expected:", error);
  }
});

it("Verifies LP mint authority is pool authority", async () => {
  const lpMintData = await getMint(provider.connection, lpMint);
  expect(lpMintData.mintAuthority.toBase58()).to.equal(poolAuthority.toBase58());
});

// Add liquidity tests 
describe("Add Liquidity tests", ()=>{
  it("Add liquidity basic functionality should work", async()=>{
    
    try {
      const tx = await program.methods.addLiquidity()
    } catch (error) {
      expect.fail(error);
    }
  })
})

});


