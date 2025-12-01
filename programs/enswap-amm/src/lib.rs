use anchor_lang::prelude::*;
use anchor_spl::{token};
use anchor_spl::{ associated_token::AssociatedToken };
use anchor_spl::token::{ TokenAccount, Transfer, Token, Mint, MintTo, Burn };
use std::cmp::min;
declare_id!("F4dfAy46FYa11G28VkreEirfqdfPw3ptgmd8rwQffsYS");

#[program]
pub mod enswap_amm {

    use super::*;

    pub fn initialize_pool(ctx: Context<Initialize>, fee: u16) -> Result<()> {
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

    pub fn add_liquidity(
        ctx: Context<AddLiquidity>,
        max_amount_a: u64,
        max_amount_b: u64,
        min_lp_tokens: u64
    ) -> Result<()> {
        let pool = &ctx.accounts.pool;
        // check if user has suffient funds
        require!(ctx.accounts.user_token_a.amount >= max_amount_a, AmmError::InsufficientFunds);
        require!(ctx.accounts.user_token_b.amount >= max_amount_b, AmmError::InsufficientFunds);
        require!(max_amount_a > 0 && max_amount_b > 0, AmmError::InsufficientFunds);
        let reserve_a = &ctx.accounts.token_reserve_a;
        let reserve_b = &ctx.accounts.token_reserve_b;
        let signer = &ctx.accounts.signer;
        const MINIMUM_LIQUIDITY: u64 = 1000;
        if reserve_a.amount == 0 && reserve_b.amount == 0 {
            let cpi_accounts_a = Transfer {
                from: ctx.accounts.user_token_a.to_account_info(),
                to: ctx.accounts.token_reserve_a.to_account_info(),
                authority: signer.to_account_info(),
            };
            let cpi_program = ctx.accounts.token_program.to_account_info();
            let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts_a);
            token::transfer(cpi_ctx, max_amount_a)?;

            let cpi_accounts_b = Transfer {
                from: ctx.accounts.user_token_b.to_account_info(),
                to: ctx.accounts.token_reserve_b.to_account_info(),
                authority: signer.to_account_info(),
            };
            let cpi_program = ctx.accounts.token_program.to_account_info();
            let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts_b);
            token::transfer(cpi_ctx, max_amount_b)?;

            let first_deposit_lp_mint = (max_amount_a as u128)
                .checked_mul(max_amount_b as u128)
                .unwrap()
                .isqrt() as u64;
            require!(
                first_deposit_lp_mint > MINIMUM_LIQUIDITY,
                AmmError::InsufficientTotalMintSupply
            );
            let authority_bumps = pool.sign_authority_bump;
            let pool_key = pool.key(); // Store the key first to avoid reference isssues
            let seeds = &[b"authority", pool_key.as_ref(), &[authority_bumps]];

            let signer_seeds = &[&seeds[..]];

            let cpi_program = ctx.accounts.token_program.to_account_info();
            let cpi_accounts = MintTo {
                mint: ctx.accounts.lp_mint.to_account_info(),
                to: ctx.accounts.user_lp_token_vault.to_account_info(),
                authority: ctx.accounts.pool_authority.to_account_info(),
            };

            let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);
            token::mint_to(cpi_ctx, first_deposit_lp_mint)?;

            let cpi_program = ctx.accounts.token_program.to_account_info();
            let cpi_accounts = Burn {
                mint: ctx.accounts.lp_mint.to_account_info(),
                from: ctx.accounts.user_lp_token_vault.to_account_info(),
                authority: signer.to_account_info(),
            };
            let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
            token::burn(cpi_ctx, MINIMUM_LIQUIDITY)?;
        } else {
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

            let lp_from_a = (max_amount_a as u128)
                .checked_mul(total_lp_supply as u128)
                .unwrap()
                .checked_div(reserve_a.amount as u128)
                .unwrap() as u64;
            let lp_from_b = (max_amount_b as u128)
                .checked_mul(total_lp_supply as u128)
                .unwrap()
                .checked_div(reserve_b.amount as u128)
                .unwrap() as u64;
            let lp_to_mint = min(lp_from_a, lp_from_b);

            let actual_amount_a = (lp_to_mint as u128)
                .checked_mul(reserve_a.amount as u128)
                .unwrap()
                .checked_div(total_lp_supply as u128)
                .unwrap() as u64;
            let actual_amount_b = (lp_to_mint as u128)
                .checked_mul(reserve_b.amount as u128)
                .unwrap()
                .checked_div(total_lp_supply as u128)
                .unwrap() as u64;

            require!(lp_to_mint >= min_lp_tokens, AmmError::SlippageExceeded);
            //  To prevent someone depositing 1 token and getting 0 actual transfer.
            require!(actual_amount_a > 0 && actual_amount_b > 0, AmmError::AmountTooSmall);
            require!(actual_amount_a <= max_amount_a, AmmError::TooMuchRequired);
            require!(actual_amount_b <= max_amount_b, AmmError::TooMuchRequired);

            let cpi_program = ctx.accounts.token_program.to_account_info();

            let cpi_accounts = Transfer {
                from: ctx.accounts.user_token_a.to_account_info(),
                to: ctx.accounts.token_reserve_a.to_account_info(),
                authority: signer.to_account_info(),
            };

            let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

            token::transfer(cpi_ctx, actual_amount_a)?;

            let cpi_accounts = Transfer {
                from: ctx.accounts.user_token_b.to_account_info(),
                to: ctx.accounts.token_reserve_b.to_account_info(),
                authority: signer.to_account_info(),
            };

            let cpi_program = ctx.accounts.token_program.to_account_info();
            let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
            token::transfer(cpi_ctx, actual_amount_b)?;

            let authority_bump = pool.sign_authority_bump;
            let pool_key = pool.key(); // Store the key first to avoid reference issues
            let seeds = &[b"authority", pool_key.as_ref(), &[authority_bump]];
            let signer_seeds = &[&seeds[..]];

            let cpi_program = ctx.accounts.token_program.to_account_info();
            let cpi_accounts = MintTo {
                mint: ctx.accounts.lp_mint.to_account_info(),
                to: ctx.accounts.user_lp_token_vault.to_account_info(),
                authority: ctx.accounts.pool_authority.to_account_info(),
            };

            let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);
            token::mint_to(cpi_ctx, lp_to_mint)?;
        }
        Ok(())
    }

    pub fn swap(ctx: Context<Swap>, amount_in: u64, min_amount_out: u64) -> Result<()> {
        
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

    pub fn withdraw_liquidity(ctx: Context<WithdrawLiquidity>, lp_tokens:u64, min_amount_a:u64, min_amount_b: u64) -> Result<()> {
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
    
}

#[derive(Accounts)]
pub struct Initialize<'info> {
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

#[derive(Accounts)]
pub struct AddLiquidity<'info> {
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
    #[account(
        init_if_needed,
        payer = signer,
        associated_token::mint = lp_mint,
        associated_token::authority = signer
    )]
    pub user_lp_token_vault: Account<'info, TokenAccount>,
    ///CHECK: pda used just per signing purposes
    #[account(seeds = [b"authority", pool.key().as_ref()], bump)]
    pub pool_authority: UncheckedAccount<'info>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
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
        constraint = token_reserve_src.mint == user_src_token_acc.mint @ AmmError::InvalidMint)]
    pub token_reserve_src: Account<'info, TokenAccount>,
    #[account(mut,
        constraint = token_reserve_dst.key() == pool.token_reserve_a || token_reserve_dst.key() == pool.token_reserve_b @ AmmError::InvalidReserve,
        constraint = token_reserve_dst.key() != token_reserve_src.key() @ AmmError::SameReserve,
        constraint = token_reserve_dst.owner == pool_authority.key() @ AmmError::Unauthorized,
        constraint = token_reserve_dst.mint == user_dst_token_acc.mint @ AmmError::InvalidMint)]
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
    #[account(mut,
        constraint = user_lp_token_vault.owner == signer.key() @ AmmError::Unauthorized,
        constraint = user_lp_token_vault.mint == lp_mint.key() @ AmmError::InvalidMint)]
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
    pub sign_authority_bump: u8,
}

#[error_code]
pub enum AmmError {
    #[msg("Extremely excessive fee amount")]
    InvalidFee,
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
    TooMuchRequired,
    #[msg("Unauthorized Access")]
    Unauthorized,
    #[msg("Invalid Reserve Account")]
    InvalidReserve,
    #[msg("Source and Destination reserve accounts cannot be the same")]
    SameReserve,
    #[msg("Source and Destination user accounts cannot be the same")]
    SameAccount,
}
