import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { EnswapAmm } from "../target/types/enswap_amm";
import {TOKEN_PROGRAM_ID, createMint, getOrCreateAssociatedTokenAccount, mintTo, getAccount, getMint, getAssociatedTokenAddress, ASSOCIATED_TOKEN_PROGRAM_ID, Account} from "@solana/spl-token";
import { SYSTEM_PROGRAM_ID } from "@coral-xyz/anchor/dist/cjs/native/system";
import { expect } from "chai";
import { token } from "@coral-xyz/anchor/dist/cjs/utils";
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
  let userTokenAAccount: Account;
  let userTokenBAccount: Account;
  let userLpTokenAccount: anchor.web3.PublicKey;
  before(async () => {
      userTokenAAccount = await getOrCreateAssociatedTokenAccount(
      provider.connection,
      provider.wallet.payer,
      mintA,
      signer
    );

     userTokenBAccount = await getOrCreateAssociatedTokenAccount(
      provider.connection,
      provider.wallet.payer,
      mintB,
      signer
    );

     userLpTokenAccount = await getAssociatedTokenAddress(
      lpMint,
      signer
    );
    // mint some tokens to user accounts
    await mintTo(
      provider.connection,
      provider.wallet.payer,
      mintA,
      userTokenAAccount.address,
      provider.wallet.publicKey,
      5_000_000_000 
    );

    await mintTo(
      provider.connection,
      provider.wallet.payer,
      mintB,
      userTokenBAccount.address,
      provider.wallet.publicKey,
      5_000_000_000
    )
    
  });

  it("Add liquidity basic functionality should work", async()=>{
    
      const tx = await program.methods.addLiquidity(new anchor.BN(5_000_000 ), new anchor.BN(5_000_000 ), new anchor.BN(1)).accounts({
        mintA: mintA,
        mintB: mintB,
        pool: pool,
        lpMint: lpMint,
        userTokenA: userTokenAAccount.address,
        userTokenB: userTokenBAccount.address,
        tokenReserveA: tokenReserveA,
        tokenReserveB: tokenReserveB,
        userLpTokenVault: userLpTokenAccount,
        poolAuthority: poolAuthority,
        signer: signer,
        systemProgram: SYSTEM_PROGRAM_ID,
        tokenProgram: TOKEN_PROGRAM_ID,
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID
      }).rpc();
      console.log("Add liquidity transaction signature", tx);
      const tokenReserveAData = await getAccount(provider.connection, tokenReserveA)
      const tokenReserveBData = await getAccount(provider.connection, tokenReserveB)
      const userLpTokenAccountData = await getAccount(provider.connection, userLpTokenAccount);

      expect(Number(userLpTokenAccountData.amount)).to.equal(4_999_000);
      expect(Number(tokenReserveAData.amount)).to.equal(5_000_000);
      expect(Number(tokenReserveBData.amount)).to.equal(5_000_000);
  })

  it("Add liquidity again to test with existing liquidity", async() =>{

    const tx = await program.methods.addLiquidity(new anchor.BN(5_000_000 ), new anchor.BN(5_000_000 ), new anchor.BN(1)).accounts({
      mintA: mintA,
        mintB: mintB,
        pool: pool,
        lpMint: lpMint,
        userTokenA: userTokenAAccount.address,
        userTokenB: userTokenBAccount.address,
        tokenReserveA: tokenReserveA,
        tokenReserveB: tokenReserveB,
        userLpTokenVault: userLpTokenAccount,
        poolAuthority: poolAuthority,
        signer: signer,
        systemProgram: SYSTEM_PROGRAM_ID,
        tokenProgram: TOKEN_PROGRAM_ID,
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID
  }).rpc();
  console.log("Add liquidity to exisiting liquidity transaction signture", tx);
  const tokenReserveAData = await getAccount(provider.connection, tokenReserveA);
  const tokenReserveBData = await getAccount(provider.connection, tokenReserveB);
  const userLpTokenAccountData = await getAccount(provider.connection, userLpTokenAccount);
  
  expect(Number(userLpTokenAccountData.amount)).to.equal(9_998_000);
  expect(Number(tokenReserveAData.amount)).to.equal(10_000_000);
  expect(Number(tokenReserveBData.amount)).to.equal(10_000_000);
})

it("fails when user funds is insufficient", async()=>{

   const userTokenAAccount = await getOrCreateAssociatedTokenAccount(
      provider.connection,
      provider.wallet.payer,
      mintA,
      signer
    );

    const userTokenBAccount = await getOrCreateAssociatedTokenAccount(
      provider.connection,
      provider.wallet.payer,
      mintB,
      signer
    );

    const userLpTokenAccount = await getAssociatedTokenAddress(
      lpMint,
      signer
    );


try{ const tx = await program.methods.addLiquidity(new anchor.BN(50_000_000_000 ), new anchor.BN(50_000_000_000 ), new anchor.BN(1)).accounts({
        mintA: mintA,
        mintB: mintB,
        pool: pool,
        lpMint: lpMint,
        userTokenA: userTokenAAccount.address,
        userTokenB: userTokenBAccount.address,
        tokenReserveA: tokenReserveA,
        tokenReserveB: tokenReserveB,
        userLpTokenVault: userLpTokenAccount,
        poolAuthority: poolAuthority,
        signer: signer,
        systemProgram: SYSTEM_PROGRAM_ID,
        tokenProgram: TOKEN_PROGRAM_ID,
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID
  }).rpc();
  expect.fail("Add liquidity should fail due to insufficient user funds");
}catch(error){
  expect(error).to.exist;
  console.log("Add liquidity failed due to insufficient user funds:", error);
}

})

it("fails when inputs are zero", async()=>{
  let userTokenAAccount: Account;
  let userTokenBAccount: Account;
  let userLpTokenAccount: anchor.web3.PublicKey;

try{ const tx = await program.methods.addLiquidity(new anchor.BN(0), new anchor.BN(0), new anchor.BN(1)).accounts({
      mintA: mintA,
        mintB: mintB,
        pool: pool,
        lpMint: lpMint,
        userTokenA: userTokenAAccount.address,
        userTokenB: userTokenBAccount.address,
        tokenReserveA: tokenReserveA,
        tokenReserveB: tokenReserveB,
        userLpTokenVault: userLpTokenAccount,
        poolAuthority: poolAuthority,
        signer: signer,
        systemProgram: SYSTEM_PROGRAM_ID,
        tokenProgram: TOKEN_PROGRAM_ID,
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID
  }).rpc();
  expect.fail("Add liquidity should fail due to inputs equal to zero");
}catch(error){
  expect(error).to.exist;
  console.log("Add liquidity failed due to inputs equal to zero:", error);
}})

})

describe("Swap tokens tests", ()=>{
 let userTokenAAccount: Account;
  let userTokenBAccount: Account;
  let userLpTokenAccount: anchor.web3.PublicKey;
  before(async()=>{
    userTokenAAccount = await getOrCreateAssociatedTokenAccount(
      provider.connection,
      provider.wallet.payer,
      mintA,
      signer
    )

       userTokenBAccount = await getOrCreateAssociatedTokenAccount(
      provider.connection,
      provider.wallet.payer,
      mintB,
      signer
    )

    userLpTokenAccount = await getAssociatedTokenAddress(
      lpMint,
      signer
    );

    // mint some tokens to user accounts
    await mintTo(
      provider.connection,
      provider.wallet.payer,
      mintA,
      userTokenAAccount.address,
      provider.wallet.publicKey,
      5_000_000_000
    )
    
    await mintTo(
      provider.connection,
      provider.wallet.payer,
      mintB,
      userTokenBAccount.address,
      provider.wallet.publicKey,
      5_000_000_000
    )

  })

 it("Token swap basic functionality should work", async()=>{

    const tx = await program.methods.swap(new anchor.BN(1_000_000), new anchor.BN(500_000)).accounts({
       mintA: mintA,
      mintB: mintB,
      pool: pool,
      lpMint: lpMint,
      userSrcTokenAcc: userTokenAAccount.address,
      userDstTokenAcc: userTokenBAccount.address,
      tokenReserveSrc: tokenReserveB,
      tokenReserveDst: tokenReserveA,
      userLpTokenVault: userLpTokenAccount,
      poolAuthority: poolAuthority,
      signer: signer,
      systemProgram: SYSTEM_PROGRAM_ID,
      tokenProgram: TOKEN_PROGRAM_ID,
      associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID
    }).rpc();
    
    const tokenReserveAData = await getAccount(provider.connection, tokenReserveA);
    const tokenReserveBData = await getAccount(provider.connection, tokenReserveB);
    const userTokenAAccountData = await getAccount(provider.connection, userTokenAAccount.address);
    const userTokenBAccountData = await getAccount(provider.connection, userTokenBAccount.address);

    // User A: started with 9,990,000,000, sent 1,000,000
    expect(Number(userTokenAAccountData.amount)).to.equal(9_989_000_000);

    // User B: started with 9,990,000,000, received 906,610
    expect(Number(userTokenBAccountData.amount)).to.equal(9_990_906_610);

    // Reserve A: started 10M, received 1M  
    expect(Number(tokenReserveAData.amount)).to.equal(11_000_000);

    // Reserve B: started 10M, sent 906,610
    expect(Number(tokenReserveBData.amount)).to.equal(9_093_390);
});

it("fail when input amount is zero", async () => {
 try {
   const tx = await program.methods.swap(new anchor.BN(0), new anchor.BN(500_000)).accounts({
    mintA: mintA,
      mintB: mintB,
      pool: pool,
      lpMint: lpMint,
      userSrcTokenAcc: userTokenAAccount.address,
      userDstTokenAcc: userTokenBAccount.address,
      tokenReserveSrc: tokenReserveB,
      tokenReserveDst: tokenReserveA,
      userLpTokenVault: userLpTokenAccount,
      poolAuthority: poolAuthority,
      signer: signer,
      systemProgram: SYSTEM_PROGRAM_ID,
      tokenProgram: TOKEN_PROGRAM_ID,
      associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID
  }).rpc();
  expect.fail("Swap should fail due to zero input amount");
 } catch (error) {
  expect(error).to.exist;
  console.log("Swap failed due to zero input amount:", error);
 }

});

it("fail when user has insufficient funds", async () => {
  let excessive_amount = Number(userTokenAAccount.amount) + 1_000_000;
  try {
   const tx = await program.methods.swap(new anchor.BN(excessive_amount), new anchor.BN(500_000)).accounts({
    mintA: mintA,
      mintB: mintB,
      pool: pool,
      lpMint: lpMint,
      userSrcTokenAcc: userTokenAAccount.address,
      userDstTokenAcc: userTokenBAccount.address,
      tokenReserveSrc: tokenReserveB,
      tokenReserveDst: tokenReserveA,
      userLpTokenVault: userLpTokenAccount,
      poolAuthority: poolAuthority,
      signer: signer,
      systemProgram: SYSTEM_PROGRAM_ID,
      tokenProgram: TOKEN_PROGRAM_ID,
      associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID
  }).rpc();
  expect.fail("Swap should fail due to insufficient user funds");
 } catch (error) {
  expect(error).to.exist;
  console.log("Swap failed due to insufficient user funds:", error);
 }
})

it("fails when pool has insufficient liquidity for output", async() => {
  // Try to drain the entire pool (impossible due to constant product)
  const reserveBalance = await getAccount(provider.connection, tokenReserveB);
  
  try {
    // Try to get more output than pool has
    await program.methods.swap(
      new anchor.BN(1_000_000_000),  // Huge input
      new anchor.BN(Number(reserveBalance.amount))  // Want entire reserve
    ).accounts({
      mintA: mintA,
      mintB: mintB,
      pool: pool,
      lpMint: lpMint,
      userSrcTokenAcc: userTokenAAccount.address,
      userDstTokenAcc: userTokenBAccount.address,
      tokenReserveSrc: tokenReserveB,
      tokenReserveDst: tokenReserveA,
      userLpTokenVault: userLpTokenAccount,
      poolAuthority: poolAuthority,
      signer: signer,
      systemProgram: SYSTEM_PROGRAM_ID,
      tokenProgram: TOKEN_PROGRAM_ID,
      associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID
    }).rpc();
    expect.fail("Should fail due to insufficient pool liquidity");
  } catch (error) {
    expect(error).to.exist;
    console.log("swap failed due to insufficient pool liquidity:", error);
  }
});

it("Token swap works in reverse direction (B -> A)", async() => {
  const userABefore = await getAccount(provider.connection, userTokenAAccount.address);
  const userBBefore = await getAccount(provider.connection, userTokenBAccount.address);
  const reserveABefore = await getAccount(provider.connection, tokenReserveA);
  const reserveBBefore = await getAccount(provider.connection, tokenReserveB);
  
  await program.methods.swap(
    new anchor.BN(1_000_000),
    new anchor.BN(500_000)
  ).accounts({
    mintA: mintA,
    mintB: mintB,
    pool: pool,
    lpMint: lpMint,
    userSrcTokenAcc: userTokenBAccount.address,  // Swap B -> A
    userDstTokenAcc: userTokenAAccount.address,
    tokenReserveSrc: tokenReserveA,  // A comes out
    tokenReserveDst: tokenReserveB,  // B goes in
    userLpTokenVault: userLpTokenAccount,
    poolAuthority: poolAuthority,
    signer: signer,
    systemProgram: SYSTEM_PROGRAM_ID,
    tokenProgram: TOKEN_PROGRAM_ID,
    associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID
  }).rpc();
  
  const userAAfter = await getAccount(provider.connection, userTokenAAccount.address);
  const userBAfter = await getAccount(provider.connection, userTokenBAccount.address);
  const reserveAAfter = await getAccount(provider.connection, tokenReserveA);
  const reserveBAfter = await getAccount(provider.connection, tokenReserveB);
  
  // Verify balances changed correctly
  expect(Number(userBAfter.amount)).to.equal(Number(userBBefore.amount) - 1_000_000);
  expect(Number(userAAfter.amount)).to.be.greaterThan(Number(userABefore.amount));
  expect(Number(reserveBAfter.amount)).to.equal(Number(reserveBBefore.amount) + 1_000_000);
  expect(Number(reserveAAfter.amount)).to.be.lessThan(Number(reserveABefore.amount));
});


it("verifies fees accumulate in pool over time", async() => {
  const reserveABefore = await getAccount(provider.connection, tokenReserveA);
  const reserveBBefore = await getAccount(provider.connection, tokenReserveB);
  
  const kBefore = Number(reserveABefore.amount) * Number(reserveBBefore.amount);
  
  // Do several swaps
  for (let i = 0; i < 5; i++) {
    await program.methods.swap(
      new anchor.BN(1_000_000),
      new anchor.BN(1)
    ).accounts({
       mintA: mintA,
      mintB: mintB,
      pool: pool,
      lpMint: lpMint,
      userSrcTokenAcc: userTokenAAccount.address,
      userDstTokenAcc: userTokenBAccount.address,
      tokenReserveSrc: tokenReserveB,
      tokenReserveDst: tokenReserveA,
      userLpTokenVault: userLpTokenAccount,
      poolAuthority: poolAuthority,
      signer: signer,
      systemProgram: SYSTEM_PROGRAM_ID,
      tokenProgram: TOKEN_PROGRAM_ID,
      associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID
    }).rpc();
  }
  
  const reserveAAfter = await getAccount(provider.connection, tokenReserveA);
  const reserveBAfter = await getAccount(provider.connection, tokenReserveB);
  const kAfter = Number(reserveAAfter.amount) * Number(reserveBAfter.amount);
  
  // K should increase due to fees staying in pool
  expect(kAfter).to.be.greaterThan(kBefore);
  console.log(`K increased from ${kBefore} to ${kAfter}`);
});
  });

describe("Remove Liquidity tests", () => {
  let userTokenAAccount: Account;
  let userTokenBAccount: Account;
  let userLpTokenAccount: anchor.web3.PublicKey;

  beforeEach(async () => {
    userLpTokenAccount = await getAssociatedTokenAddress(lpMint, signer);
    userTokenAAccount = await getOrCreateAssociatedTokenAccount(
      provider.connection,
      provider.wallet.payer,
      mintA,
      signer
    );
    userTokenBAccount = await getOrCreateAssociatedTokenAccount(
      provider.connection,
      provider.wallet.payer,
      mintB,
      signer
    );
  });

  it("Remove liquidity basic functionality should work", async () => {
    const lpBalanceBefore = await getAccount(provider.connection, userLpTokenAccount);
    const actualLpBalance = Number(lpBalanceBefore.amount);
    
    console.log("=== ACTUAL LP BALANCE ===");
    console.log("User has:", actualLpBalance, "LP tokens");
    
    const reserveABefore = await getAccount(provider.connection, tokenReserveA);
    const reserveBBefore = await getAccount(provider.connection, tokenReserveB);
    const lpMintData = await getMint(provider.connection, lpMint);
    const totalLpSupply = Number(lpMintData.supply);
    
    console.log("Total LP supply:", totalLpSupply);
    console.log("Reserve A:", Number(reserveABefore.amount));
    console.log("Reserve B:", Number(reserveBBefore.amount));

    const userTokenABalanceBefore = Number(userTokenAAccount.amount);
    const userTokenBBalanceBefore = Number(userTokenBAccount.amount);
    
    const lpBurned = 4_545_000;
    const expectedAmountA = Math.floor((lpBurned * Number(reserveABefore.amount)) / totalLpSupply);
    const expectedAmountB = Math.floor((lpBurned * Number(reserveBBefore.amount)) / totalLpSupply);
    
    const tx = await program.methods
      .withdrawLiquidity(new anchor.BN(lpBurned), new anchor.BN(5_000), new anchor.BN(5_000))
      .accounts({
        mintA: mintA,
        mintB: mintB,
        pool: pool,
        lpMint: lpMint,
        reserveA: tokenReserveA,
        reserveB: tokenReserveB,
        userTokenA: userTokenAAccount.address,
        userTokenB: userTokenBAccount.address,
        userLpTokenVault: userLpTokenAccount,
        poolAuthority: poolAuthority,
        signer: signer,
        systemProgram: SYSTEM_PROGRAM_ID,
        tokenProgram: TOKEN_PROGRAM_ID,
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID
      })
      .rpc();
    
    console.log("Remove liquidity transaction signature", tx);
    
    const lpBalanceAfter = await getAccount(provider.connection, userLpTokenAccount);
    const reserveAAfter = await getAccount(provider.connection, tokenReserveA);
    const reserveBAfter = await getAccount(provider.connection, tokenReserveB);
    const userTokenAAccountData = await getAccount(provider.connection, userTokenAAccount.address);
    const userTokenBAccountData = await getAccount(provider.connection, userTokenBAccount.address);
    
    console.log("=== AFTER WITHDRAWAL ===");
    console.log("User LP tokens:", Number(lpBalanceAfter.amount));
    console.log("Reserve A:", Number(reserveAAfter.amount));
    console.log("Reserve B:", Number(reserveBAfter.amount));
    
    // Check LP tokens burned correctly
    expect(Number(lpBalanceAfter.amount)).to.equal(actualLpBalance - lpBurned);
    
    // Check reserves decreased by correct amounts
    expect(Number(reserveAAfter.amount)).to.equal(Number(reserveABefore.amount) - expectedAmountA);
    expect(Number(reserveBAfter.amount)).to.equal(Number(reserveBBefore.amount) - expectedAmountB);
    
    // Check user received correct amounts
    expect(Number(userTokenAAccountData.amount) - userTokenABalanceBefore).to.equal(expectedAmountA);
    expect(Number(userTokenBAccountData.amount) - userTokenBBalanceBefore).to.equal(expectedAmountB);
    
    // Verify slippage protection worked
    expect(expectedAmountA).to.be.greaterThanOrEqual(5_000);
    expect(expectedAmountB).to.be.greaterThanOrEqual(5_000);
  });

  it("Should fail when withdrawing 0 LP tokens", async () => {
    try {
      await program.methods
        .withdrawLiquidity(new anchor.BN(0), new anchor.BN(0), new anchor.BN(0))
        .accounts({
          mintA: mintA,
          mintB: mintB,
          pool: pool,
          lpMint: lpMint,
          reserveA: tokenReserveA,
          reserveB: tokenReserveB,
          userTokenA: userTokenAAccount.address,
          userTokenB: userTokenBAccount.address,
          userLpTokenVault: userLpTokenAccount,
          poolAuthority: poolAuthority,
          signer: signer,
          systemProgram: SYSTEM_PROGRAM_ID,
          tokenProgram: TOKEN_PROGRAM_ID,
          associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID
        })
        .rpc();
      expect.fail("Should have thrown an error");
    } catch (error) {
      expect(error.toString()).to.include("InsufficientFunds");
    }
  });

  it("Should fail when user has insufficient LP tokens", async () => {
    try {
      await program.methods
        .withdrawLiquidity(new anchor.BN(99999999999), new anchor.BN(0), new anchor.BN(0))
        .accounts({
          mintA: mintA,
          mintB: mintB,
          pool: pool,
          lpMint: lpMint,
          reserveA: tokenReserveA,
          reserveB: tokenReserveB,
          userTokenA: userTokenAAccount.address,
          userTokenB: userTokenBAccount.address,
          userLpTokenVault: userLpTokenAccount,
          poolAuthority: poolAuthority,
          signer: signer,
          systemProgram: SYSTEM_PROGRAM_ID,
          tokenProgram: TOKEN_PROGRAM_ID,
          associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID
        })
        .rpc();
      expect.fail("Should have thrown an error");
    } catch (error) {
      expect(error.toString()).to.include("InsufficientFunds");
    }
  });

  it("Should fail when slippage exceeded for token A", async () => {
    try {
      await program.methods
        .withdrawLiquidity(new anchor.BN(1000), new anchor.BN(999999999), new anchor.BN(0))
        .accounts({
          mintA: mintA,
          mintB: mintB,
          pool: pool,
          lpMint: lpMint,
          reserveA: tokenReserveA,
          reserveB: tokenReserveB,
          userTokenA: userTokenAAccount.address,
          userTokenB: userTokenBAccount.address,
          userLpTokenVault: userLpTokenAccount,
          poolAuthority: poolAuthority,
          signer: signer,
          systemProgram: SYSTEM_PROGRAM_ID,
          tokenProgram: TOKEN_PROGRAM_ID,
          associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID
        })
        .rpc();
      expect.fail("Should have thrown an error");
    } catch (error) {
      expect(error.toString()).to.include("SlippageExceeded");
    }
  });

  it("Should fail when slippage exceeded for token B", async () => {
    try {
      await program.methods
        .withdrawLiquidity(new anchor.BN(1000), new anchor.BN(0), new anchor.BN(999999999))
        .accounts({
          mintA: mintA,
          mintB: mintB,
          pool: pool,
          lpMint: lpMint,
          reserveA: tokenReserveA,
          reserveB: tokenReserveB,
          userTokenA: userTokenAAccount.address,
          userTokenB: userTokenBAccount.address,
          userLpTokenVault: userLpTokenAccount,
          poolAuthority: poolAuthority,
          signer: signer,
          systemProgram: SYSTEM_PROGRAM_ID,
          tokenProgram: TOKEN_PROGRAM_ID,
          associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID
        })
        .rpc();
      expect.fail("Should have thrown an error");
    } catch (error) {
      expect(error.toString()).to.include("SlippageExceeded");
    }
  });

  it("Should maintain correct ratio after multiple partial withdrawals", async () => {
    const lpBalanceBefore = await getAccount(provider.connection, userLpTokenAccount);
    const reserveABefore = await getAccount(provider.connection, tokenReserveA);
    const reserveBBefore = await getAccount(provider.connection, tokenReserveB);
    
    const ratioBefore = Number(reserveABefore.amount) / Number(reserveBBefore.amount);
    
    // First withdrawal
    await program.methods
      .withdrawLiquidity(new anchor.BN(100_000), new anchor.BN(0), new anchor.BN(0))
      .accounts({
        mintA: mintA,
        mintB: mintB,
        pool: pool,
        lpMint: lpMint,
        reserveA: tokenReserveA,
        reserveB: tokenReserveB,
        userTokenA: userTokenAAccount.address,
        userTokenB: userTokenBAccount.address,
        userLpTokenVault: userLpTokenAccount,
        poolAuthority: poolAuthority,
        signer: signer,
        systemProgram: SYSTEM_PROGRAM_ID,
        tokenProgram: TOKEN_PROGRAM_ID,
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID
      })
      .rpc();
    
    // Second withdrawal
    await program.methods
      .withdrawLiquidity(new anchor.BN(100_000), new anchor.BN(0), new anchor.BN(0))
      .accounts({
        mintA: mintA,
        mintB: mintB,
        pool: pool,
        lpMint: lpMint,
        reserveA: tokenReserveA,
        reserveB: tokenReserveB,
        userTokenA: userTokenAAccount.address,
        userTokenB: userTokenBAccount.address,
        userLpTokenVault: userLpTokenAccount,
        poolAuthority: poolAuthority,
        signer: signer,
        systemProgram: SYSTEM_PROGRAM_ID,
        tokenProgram: TOKEN_PROGRAM_ID,
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID
      })
      .rpc();
    
    const reserveAAfter = await getAccount(provider.connection, tokenReserveA);
    const reserveBAfter = await getAccount(provider.connection, tokenReserveB);
    const ratioAfter = Number(reserveAAfter.amount) / Number(reserveBAfter.amount);
    
    // Ratio should remain approximately the same (allow for small rounding differences)
    expect(Math.abs(ratioBefore - ratioAfter) / ratioBefore).to.be.lessThan(0.01); // Within 1%
  });

  it("Should withdraw all remaining liquidity successfully", async () => {
    const lpBalanceBefore = await getAccount(provider.connection, userLpTokenAccount);
    const allLpTokens = Number(lpBalanceBefore.amount);
    
    await program.methods
      .withdrawLiquidity(new anchor.BN(allLpTokens), new anchor.BN(0), new anchor.BN(0))
      .accounts({
        mintA: mintA,
        mintB: mintB,
        pool: pool,
        lpMint: lpMint,
        reserveA: tokenReserveA,
        reserveB: tokenReserveB,
        userTokenA: userTokenAAccount.address,
        userTokenB: userTokenBAccount.address,
        userLpTokenVault: userLpTokenAccount,
        poolAuthority: poolAuthority,
        signer: signer,
        systemProgram: SYSTEM_PROGRAM_ID,
        tokenProgram: TOKEN_PROGRAM_ID,
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID
      })
      .rpc();
    
    const lpBalanceAfter = await getAccount(provider.connection, userLpTokenAccount);
    expect(Number(lpBalanceAfter.amount)).to.equal(0);
  });
});

})
