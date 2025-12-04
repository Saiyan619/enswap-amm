use anchor_lang::prelude::*;
use anchor_spl::token::{self, Burn, Mint, Token, TokenAccount, Transfer};
use anchor_spl::associated_token::AssociatedToken;
use crate::state::Pool;
use crate::errors::AmmError;

pub fn handler(ctx: Context<WithdrawLiquidity>, lp_tokens:u64, min_amount_a:u64, min_amount_b: u64) -> Result<()> {
        let pool = &mut ctx.accounts.pool;
        let total_lp_supply = ctx.accounts.lp_mint.supply;
        require!(lp_tokens > 0, AmmError::InsufficientFunds);
        require!(ctx.accounts.user_lp_token_vault.amount >= lp_tokens, AmmError::InsufficientFunds);
        //lp_tokens*reserve_a/total_lp_supply
        let amount_a = (lp_tokens as u128).checked_mul(ctx.accounts.reserve_a.amount as u128).unwrap().checked_div(total_lp_supply as u128).unwrap() as u64;
        let amount_b = (lp_tokens as u128).checked_mul(ctx.accounts.reserve_b.amount as u128).unwrap().checked_div(total_lp_supply as u128).unwrap() as u64;
        require!(amount_a >= min_amount_a, AmmError::SlippageExceeded);
        require!(amount_b >= min_amount_b, AmmError::SlippageExceeded);

        let ctx_program = ctx.accounts.token_program.to_account_info();
        let ctx_accounts = Burn{
            mint: ctx.accounts.lp_mint.to_account_info(),
            from: ctx.accounts.user_lp_token_vault.to_account_info(),
            authority: ctx.accounts.signer.to_account_info()
        };
        let cpi_ctx = CpiContext::new(ctx_program, ctx_accounts);
        token::burn(cpi_ctx, lp_tokens)?;

        let ctx_program = ctx.accounts.token_program.to_account_info();
        let ctx_accounts = Transfer{
            to: ctx.accounts.user_token_a.to_account_info(),
            from: ctx.accounts.reserve_a.to_account_info(),
            authority: ctx.accounts.pool_authority.to_account_info()
        };
        
        let authority_bump = pool.sign_authority_bump;
        let key = pool.key();
        let seeds = &[
            b"authority",
            key.as_ref(),
            &[authority_bump]
        ];
        let signer_seeds = &[&seeds[..]];

        let cpi_ctx = CpiContext::new_with_signer(ctx_program, ctx_accounts, signer_seeds);
        token::transfer(cpi_ctx, amount_a)?;

        let ctx_program = ctx.accounts.token_program.to_account_info();
        let ctx_accounts = Transfer{
            to: ctx.accounts.user_token_b.to_account_info(),
            from: ctx.accounts.reserve_b.to_account_info(),
            authority: ctx.accounts.pool_authority.to_account_info()
        };
        
        let cpi_ctx = CpiContext::new_with_signer(ctx_program, ctx_accounts, signer_seeds);
        token::transfer(cpi_ctx, amount_b)?;
        Ok(())
}

#[derive(Accounts)]
pub struct WithdrawLiquidity <'info>{
    #[account(address = pool.mint_a)]
    pub mint_a: Account<'info, Mint>,
    #[account(address = pool.mint_b)]
    pub mint_b: Account<'info, Mint>,
    #[account(mut, seeds = [b"pool", mint_a.key().as_ref(), mint_b.key().as_ref()], bump)]
    pub pool: Account<'info, Pool>,
    ///CHECK: pda used just for signing purposes
    #[account(mut, seeds=[b"authority", pool.key().as_ref()], bump)]
    pub pool_authority: UncheckedAccount<'info>,
    #[account(mut, address=pool.token_reserve_a, constraint = reserve_a.owner == pool_authority.key() @ AmmError::Unauthorized)]
    pub reserve_a: Account<'info, TokenAccount>,
    #[account(mut, address=pool.token_reserve_b, constraint = reserve_b.owner == pool_authority.key() @ AmmError::Unauthorized)]
    pub reserve_b: Account<'info, TokenAccount>,
    #[account(mut, seeds=[b"lp_mint", pool.key().as_ref()], bump)]
    pub lp_mint: Account<'info, Mint>,
    #[account(mut, associated_token::mint=lp_mint, associated_token::authority=signer)]
        pub user_lp_token_vault: Account<'info, TokenAccount>,
        #[account(mut,
        associated_token::mint=mint_a, associated_token::authority=signer)]
        pub user_token_a: Account<'info, TokenAccount>,
        #[account(mut,
        associated_token::mint=mint_b, associated_token::authority=signer)]
        pub user_token_b: Account<'info, TokenAccount>,
        #[account(mut)]
        signer: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}