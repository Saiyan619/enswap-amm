use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};
use crate::state::Pool;
use crate::errors::AmmError;

 pub fn handler(ctx: Context<Initialize>, fee: u16) -> Result<()> {
        require!(fee <= 10000, AmmError::InvalidFee);
        let pool = &mut ctx.accounts.pool;
        pool.fee_bps = fee;
        pool.mint_a = ctx.accounts.mint_a.key();
        pool.mint_b = ctx.accounts.mint_b.key();
        pool.token_reserve_a = ctx.accounts.token_reserve_a.key();
        pool.token_reserve_b = ctx.accounts.token_reserve_b.key();
        pool.lp_mint = ctx.accounts.lp_mint.key();
        pool.sign_authority_bump = ctx.bumps.pool_authority;
        msg!("pool initialized succesfully");
        msg!("mint_a: {}", pool.mint_a);
        msg!("mint_b: {}", pool.mint_b);
        msg!("reserve_a: {}", pool.token_reserve_a);
        msg!("reserve_b: {}", pool.token_reserve_b);
        msg!("lp_mint: {}", pool.lp_mint);
        msg!("fee_bps: {}", pool.fee_bps);
        msg!("sign_authority_bump: {}", pool.sign_authority_bump);
        Ok(())
}



#[derive(Accounts)]
pub struct     Initialize<'info> {
    pub mint_a: Account<'info, Mint>,
    pub mint_b: Account<'info, Mint>,
    #[account(
        init,
        payer = signer,
        space = 8 + 2 + 32 + 32 + 32 + 32 + 32 + 1,
        seeds = [b"pool", mint_a.key().as_ref(), mint_b.key().as_ref()],
        bump
    )]
    pub pool: Account<'info, Pool>,
    #[account(
        init,
        payer = signer,
        seeds = [b"lp_mint", pool.key().as_ref()],
        bump,
        mint::decimals = 6,
        mint::authority = pool_authority
    )]
    pub lp_mint: Account<'info, Mint>,
    #[account(
        init,
        payer = signer,
        token::mint = mint_a,
        token::authority = pool_authority,
        seeds = [b"reserve_a", pool.key().as_ref()],
        bump
    )]
    pub token_reserve_a: Account<'info, TokenAccount>,
    #[account(
        init,
        payer = signer,
        token::mint = mint_b,
        token::authority = pool_authority,
        seeds = [b"reserve_b", pool.key().as_ref()],
        bump
    )]
    pub token_reserve_b: Account<'info, TokenAccount>,
    /// CHECK: PDA derived from seeds, no data needed
    #[account(seeds = [b"authority", pool.key().as_ref()], bump)]
    pub pool_authority: UncheckedAccount<'info>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}
