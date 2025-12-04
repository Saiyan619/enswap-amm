use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount, Transfer};
use anchor_spl::associated_token::AssociatedToken;
use crate::state::Pool;
use crate::errors::AmmError;

  pub fn handler(ctx: Context<Swap>, amount_in: u64, min_amount_out: u64) -> Result<()> {
        require!(ctx.accounts.user_src_token_acc.amount >= amount_in, AmmError::InsufficientFunds);
        require!(amount_in > 0, AmmError::InsufficientFunds);
        let pool = &mut ctx.accounts.pool;
        let fee_percent = pool.fee_bps as u128;
        // calculate the fees and sutract from amount_in
        // calculation
        // e.g 3% - 0.03 - fee , 2sol - amount_in
        // 1 - 0.03 = 0.97 (1 meaning 100 and 0.03 meaning 3%)
        // let amount_in_post_fee = 2(sol) * 0.97 = 1.94(sol)
        let actual_amount_in_percent = (10000u128).checked_sub(fee_percent).unwrap();
        let amount_in_post_fee = (amount_in as u128)
            .checked_mul(actual_amount_in_percent as u128)
            .unwrap()
            .checked_div(10000u128)
            .unwrap() as u64;

        let reserve_src = &mut ctx.accounts.token_reserve_src;
        let reserve_dst = &mut ctx.accounts.token_reserve_dst;
        // // constant product formula A * B = K
        // Before swap:
        // k = reserve_in * reserve_out

        // // After swap:
        // new_reserve_in = reserve_in + amount_in_with_fee
        let new_reserve_dst = reserve_dst.amount.checked_add(amount_in_post_fee).unwrap();
        // new_reserve_out = ?

        // // Keep k constant:
        // new_reserve_in * new_reserve_out = k

        // // So:
        // new_reserve_out = k / new_reserve_in
        //                 = (reserve_in * reserve_out) / (reserve_in + amount_in_with_fee)

        // let new_reserve_out = (reserve_src.amount as u128).checked_mul(reserve_dst.amount as u128).unwrap().checked_div(new_reserve_in as u128).unwrap() as u64;

        // // Final formula:
        // amount_out = (reserve_out * amount_in_with_fee) / (reserve_in + amount_in_with_fee)
        let calculated_amount_out = (reserve_src.amount as u128)
            .checked_mul(amount_in_post_fee as u128)
            .unwrap()
            .checked_div(new_reserve_dst as u128)
            .unwrap() as u64;

        //  check if the slippage condition is met i.e is the calculated_amount_out >= min_amount_out
         require!(reserve_src.amount >= calculated_amount_out, AmmError::InsufficientFunds);
         require!(calculated_amount_out >= min_amount_out, AmmError::SlippageExceeded);
        
        // fifth - transfer tokens
        let ctx_program = ctx.accounts.token_program.to_account_info();
        let ctx_accounts = Transfer {
            to: ctx.accounts.token_reserve_dst.to_account_info(),
            from: ctx.accounts.user_src_token_acc.to_account_info(),
            authority: ctx.accounts.signer.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(ctx_program, ctx_accounts);
        token::transfer(cpi_ctx, amount_in)?;

        let authority_bump = pool.sign_authority_bump;
        let pool_key = pool.key(); // Store the key first to avoid reference issues
        let seeds = &[b"authority", pool_key.as_ref(), &[authority_bump]];
        let signer_seeds = &[&seeds[..]];
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_accounts = Transfer {
            to: ctx.accounts.user_dst_token_acc.to_account_info(),
            from: ctx.accounts.token_reserve_src.to_account_info(),
            authority: ctx.accounts.pool_authority.to_account_info(),
        };
        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);
        token::transfer(cpi_ctx, calculated_amount_out)?;

        Ok(())
}


#[derive(Accounts)]
pub struct Swap<'info> {
    #[account(address = pool.mint_a)]
    pub mint_a: Account<'info, Mint>,
    #[account(address = pool.mint_b)]
    pub mint_b: Account<'info, Mint>,
    #[account(mut, seeds = [b"pool", mint_a.key().as_ref(), mint_b.key().as_ref()], bump)]
    pub pool: Account<'info, Pool>,
    #[account(mut,
        constraint = token_reserve_src.key() == pool.token_reserve_a || token_reserve_src.key() == pool.token_reserve_b @ AmmError::InvalidReserve,
        constraint = token_reserve_src.key() != token_reserve_dst.key() @ AmmError::SameReserve,
        constraint = token_reserve_src.owner == pool_authority.key() @ AmmError::Unauthorized,
        constraint = token_reserve_src.mint == user_dst_token_acc.mint @ AmmError::InvalidMint)]
    pub token_reserve_src: Account<'info, TokenAccount>,
    #[account(mut,
        constraint = token_reserve_dst.key() == pool.token_reserve_a || token_reserve_dst.key() == pool.token_reserve_b @ AmmError::InvalidReserve,
        constraint = token_reserve_dst.key() != token_reserve_src.key() @ AmmError::SameReserve,
        constraint = token_reserve_dst.owner == pool_authority.key() @ AmmError::Unauthorized,
        constraint = token_reserve_dst.mint == user_src_token_acc.mint @ AmmError::InvalidMint)]
    pub token_reserve_dst: Account<'info, TokenAccount>,
    #[account(mut,
        constraint = user_src_token_acc.key() != user_dst_token_acc.key() @ AmmError::SameAccount,
        constraint = user_src_token_acc.owner == signer.key() @ AmmError::Unauthorized)]
    pub user_src_token_acc: Account<'info, TokenAccount>,
    #[account(mut,
        constraint = user_dst_token_acc.key() != user_src_token_acc.key() @ AmmError::SameAccount,
        constraint = user_dst_token_acc.owner == signer.key() @ AmmError::Unauthorized)]
    pub user_dst_token_acc: Account<'info, TokenAccount>,
    ///CHECK: pda used just for signing purposes
    #[account(seeds = [b"authority", pool.key().as_ref()], bump)]
    pub pool_authority: UncheckedAccount<'info>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}
