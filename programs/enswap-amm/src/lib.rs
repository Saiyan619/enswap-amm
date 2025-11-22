use anchor_lang::prelude::*;
use anchor_spl::token;
use anchor_spl::{associated_token::AssociatedToken};
use anchor_spl::token::{TokenAccount, Transfer, Token, Mint, MintTo, Burn};
use std::cmp::min;
declare_id!("F4dfAy46FYa11G28VkreEirfqdfPw3ptgmd8rwQffsYS");

#[program]
pub mod enswap_amm {
    use super::*;

    pub fn initialize_pool(ctx: Context<Initialize>, fee:u16) -> Result<()> {
        let pool = &mut ctx.accounts.pool;
        pool.fee_bps = fee;
        pool.mint_a = ctx.accounts.mint_a.key();
        pool.mint_b = ctx.accounts.mint_b.key();
        pool.token_reserve_a = ctx.accounts.token_reserve_a.key();
        pool.token_reserve_b = ctx.accounts.token_reserve_b.key();
        pool.lp_mint = ctx.accounts.lp_mint.key();
        pool.sign_authority_bump = ctx.bumps.pool_authority;
        Ok(())
    } 

    pub fn add_liquidity(ctx: Context<AddLiquidity>, max_amount_a:u64, max_amount_b:u64, min_lp_tokens: u64) -> Result<()>{
        let pool = &ctx.accounts.pool;
        // check if user has suffient funds
        require!(ctx.accounts.user_token_a.amount >= max_amount_a, AmmError::InsufficientFunds);
        require!(ctx.accounts.user_token_b.amount >= max_amount_b, AmmError::InsufficientFunds);
        let reserve_a = &ctx.accounts.token_reserve_a;
        let reserve_b = &ctx.accounts.token_reserve_b;
        let signer = &ctx.accounts.signer;
        const MINIMUM_LIQUIDITY: u64 = 1000;
        if reserve_a.amount == 0 && reserve_b.amount == 0{
            
            let cpi_accounts_a = Transfer{
                from: ctx.accounts.user_token_a.to_account_info(),
                to: ctx.accounts.token_reserve_a.to_account_info(),
                authority: signer.to_account_info()
            };
            let cpi_program = ctx.accounts.token_program.to_account_info();
            let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts_a);
            token::transfer(cpi_ctx, max_amount_a)?;

            let cpi_accounts_b = Transfer{
                from: ctx.accounts.user_token_b.to_account_info(),
                to: ctx.accounts.token_reserve_b.to_account_info(),
                authority: signer.to_account_info()
            };
            let cpi_program = ctx.accounts.token_program.to_account_info();
            let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts_b);
            token::transfer(cpi_ctx, max_amount_b)?;

            let first_deposit_lp_mint = (max_amount_a as u128).checked_mul(max_amount_b as u128).unwrap().isqrt() as u64;
            require!(first_deposit_lp_mint > MINIMUM_LIQUIDITY, AmmError::InsufficientTotalMintSupply);
            let authority_bumps = pool.sign_authority_bump;
            let pool_key = pool.key(); // Store the key first to avoid reference isssues
            let seeds = &[
                b"authority",
                pool_key.as_ref(),  
                &[authority_bumps]
                ];
                
                let signer_seeds = &[&seeds[..]];

                let cpi_program = ctx.accounts.token_program.to_account_info();
                let cpi_accounts = MintTo{
                    mint: ctx.accounts.lp_mint.to_account_info(),
                    to: ctx.accounts.user_lp_token_vault.to_account_info(),
                    authority: ctx.accounts.pool_authority.to_account_info()
                };

                let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);
                token::mint_to(cpi_ctx, first_deposit_lp_mint)?;

                let cpi_program = ctx.accounts.token_program.to_account_info();
                let cpi_accounts = Burn{
                    mint:ctx.accounts.lp_mint.to_account_info(),
                    from: ctx.accounts.user_lp_token_vault.to_account_info(),
                    authority: signer.to_account_info()
                };
                let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
                token::burn(cpi_ctx, MINIMUM_LIQUIDITY)?;

        }else{

///// Updated version
 // Calculate LP based on both tokens
// let lp_from_a = (max_max_amount_a * total_lp_supply) / reserve_a;
// let lp_from_b = (max_max_amount_b * total_lp_supply) / reserve_b;

// // Take the minimum
// let lp_to_mint = min(lp_from_a, lp_from_b);

// // Now calculate ACTUAL amounts to transfer (reverse calculation)
// let actual_max_amount_a = (lp_to_mint * reserve_a) / total_lp_supply;
// let actual_max_amount_b = (lp_to_mint * reserve_b) / total_lp_supply;

// // Transfer only actual amounts (not max amounts!)
// transfer(user_token_a -> reserve_a, actual_max_amount_a);
// transfer(user_token_b -> reserve_b, actual_max_amount_b);

            let total_lp_supply = ctx.accounts.lp_mint.supply;

         let lp_from_a = (max_amount_a as u128).checked_mul(total_lp_supply as u128).unwrap().checked_div(reserve_a.amount as u128).unwrap() as u64;
         let lp_from_b = (max_amount_b as u128).checked_mul(total_lp_supply as u128).unwrap().checked_div(reserve_b.amount as u128).unwrap() as u64;
         let lp_to_mint = min(lp_from_a, lp_from_b);

          let actual_amount_a = (lp_to_mint as u128).checked_mul(reserve_a.amount as u128).unwrap().checked_div(total_lp_supply as u128).unwrap() as u64;
         let actual_amount_b = (lp_to_mint as u128).checked_mul(reserve_b.amount as u128).unwrap().checked_div(total_lp_supply as u128).unwrap() as u64;
         
         require!(lp_to_mint >= min_lp_tokens, AmmError::SlippageExceeded);
        //  To prevent someone depositing 1 token and getting 0 actual transfer.
         require!(actual_amount_a > 0 && actual_amount_b > 0, AmmError::AmountTooSmall);
        require!(actual_amount_a <= max_amount_a, AmmError::TooMuchRequired);
require!(actual_amount_b <= max_amount_b, AmmError::TooMuchRequired);

         let cpi_program = ctx.accounts.token_program.to_account_info();

         let cpi_accounts = Transfer{
            from: ctx.accounts.user_token_a.to_account_info(),
            to: ctx.accounts.token_reserve_a.to_account_info(),
            authority: signer.to_account_info()
         };

         let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

         token::transfer(cpi_ctx, actual_amount_a)?;

          let cpi_accounts = Transfer{
            from: ctx.accounts.user_token_b.to_account_info(),
            to: ctx.accounts.token_reserve_b.to_account_info(),
            authority: signer.to_account_info()
         };

         let cpi_program = ctx.accounts.token_program.to_account_info();
         let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
         token::transfer(cpi_ctx, actual_amount_b)?;

          let authority_bump = pool.sign_authority_bump;
         let pool_key = pool.key(); // Store the key first to avoid reference issues
         let seeds = &[
            b"authority",
            pool_key.as_ref(),
            &[authority_bump]
         ];
         let signer_seeds = &[&seeds[..]];

         let cpi_program = ctx.accounts.token_program.to_account_info();
         let cpi_accounts = MintTo{
            mint: ctx.accounts.lp_mint.to_account_info(),
            to: ctx.accounts.user_lp_token_vault.to_account_info(),
            authority: ctx.accounts.pool_authority.to_account_info()
         };

         let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);
         token::mint_to(cpi_ctx, lp_to_mint)?;

        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize <'info> {
    pub mint_a: Account<'info, Mint>,
    pub mint_b: Account<'info, Mint>,
    #[account(init, 
        payer = signer, 
        space = 8 + 2 + 32 + 32 + 32 + 32 + 32 + 1,
        seeds = [b"pool", mint_a.key().as_ref(), mint_b.key().as_ref()],
        bump
    )]
    pub pool: Account<'info, Pool>, 
    #[account(init, payer=signer, seeds=[b"lp_mint", pool.key().as_ref()], bump, mint::decimals=6, mint::authority=pool_authority)]
    pub lp_mint: Account<'info, Mint>,
    #[account(init,
         payer = signer, 
         token::mint=mint_a, 
         token::authority=pool_authority,
         seeds = [b"reserve_a", pool.key().as_ref()],
         bump
        )]
    pub token_reserve_a: Account<'info, TokenAccount>,
    #[account(init, 
        payer = signer, 
        token::mint=mint_b, 
        token::authority=pool_authority,
        seeds = [b"reserve_b", pool.key().as_ref()],
        bump
    )]
    pub token_reserve_b: Account<'info, TokenAccount>,
    /// CHECK: PDA derived from seeds, no data needed
    #[account(seeds = [b"authority", pool.key().as_ref()],bump)]
    pub pool_authority: UncheckedAccount<'info>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>

}
 
#[derive(Accounts)]
pub struct AddLiquidity <'info>{
  #[account(address = pool.mint_a)]
pub mint_a: Account<'info, Mint>,

#[account(address = pool.mint_b)]
pub mint_b: Account<'info, Mint>,

#[account(
    mut,
    seeds = [b"pool", mint_a.key().as_ref(), mint_b.key().as_ref()],
    bump
)]
pub pool: Account<'info, Pool>,
    #[account(mut, associated_token::mint=pool.mint_a, associated_token::authority=signer)]
    pub user_token_a: Account<'info, TokenAccount>,
    #[account(mut, associated_token::mint=pool.mint_b, associated_token::authority=signer)]
    pub user_token_b: Account<'info, TokenAccount>,
    #[account(mut,
         seeds = [b"reserve_a", pool.key().as_ref()],
         bump
        )]
    pub token_reserve_a: Account<'info, TokenAccount>,
    #[account(mut,
         seeds = [b"reserve_b", pool.key().as_ref()],
         bump
        )]
    pub token_reserve_b: Account<'info, TokenAccount>,
    #[account(mut, seeds=[b"lp_mint", pool.key().as_ref()], bump)]
    pub lp_mint: Account<'info, Mint>,
    //Using associated token account for this vault because this vault is created/used by the user/client 
    //hence, associated_token::authority=signer 
    #[account(init_if_needed, 
        payer=signer,
        associated_token::mint=lp_mint, 
        associated_token::authority=signer)]
    pub user_lp_token_vault: Account<'info, TokenAccount>,
    ///CHECK: pda used just per signing purposes
    #[account(seeds = [b"authority", pool.key().as_ref()],bump)]
    pub pool_authority: UncheckedAccount<'info>,
    #[account(
        init_if_needed,
        payer = signer,
        seeds = [b"locked_lp", pool.key().as_ref()],
        bump,
        token::mint = lp_mint,
        token::authority = pool_authority  // No one can transfer from this
    )]
    pub locked_lp_vault: Account<'info, TokenAccount>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}


// Common Data Sizes:
//     Pubkey = 32 bytes
//     u64 = 8 bytes
//     u32 = 4 bytes
//     u16 = 2 bytes
//     u8 = 1 byte
//     bool = 1 byte

#[account]
pub struct Pool {
    pub fee_bps: u16,
    pub mint_a: Pubkey,
    pub mint_b: Pubkey,
    pub token_reserve_a: Pubkey,
    pub token_reserve_b: Pubkey,
    pub lp_mint: Pubkey,
    pub sign_authority_bump: u8
}

#[error_code]
pub enum AmmError{
    #[msg("Invalid mint for this account")]
    InvalidMint,
    #[msg("Insufficient Funds")]
    InsufficientFunds,
    #[msg("Invalid Funds Ratio")]
    InvalidRatio,
    #[msg("Insuffiecient Total Mints Available")]
    InsufficientTotalMintSupply,
    #[msg("Slippage Limit Exceeded")]
    SlippageExceeded,
    #[msg("Amount to small to mint any LP tokens")]
    AmountTooSmall,
    #[msg("Required amount exceeds maximum allowed")]
    TooMuchRequired
}