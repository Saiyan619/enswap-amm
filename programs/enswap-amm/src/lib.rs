use anchor_lang::prelude::*;
mod state;
mod errors;
mod instructions;
use instructions::*;
declare_id!("F4dfAy46FYa11G28VkreEirfqdfPw3ptgmd8rwQffsYS");

#[program]
pub mod enswap_amm {

    use super::*;

    pub fn initialize_pool(ctx: Context<Initialize>, fee: u16) -> Result<()> {
      instructions::initialize_pool::handler(ctx, fee)
    }

    pub fn add_liquidity(ctx: Context<AddLiquidity>, max_amount_a: u64, max_amount_b: u64, min_lp_tokens: u64
    ) -> Result<()> {
       instructions::add_liquidity::handler(ctx, max_amount_a, max_amount_b, min_lp_tokens)
    }

    pub fn swap(ctx: Context<Swap>, amount_in: u64, min_amount_out: u64) -> Result<()> {
        instructions::swap::handler(ctx, amount_in, min_amount_out)
    }

    pub fn withdraw_liquidity(ctx: Context<WithdrawLiquidity>, lp_tokens:u64, min_amount_a:u64, min_amount_b: u64) -> Result<()> {
       instructions::withdraw_liquidity::handler(ctx, lp_tokens, min_amount_a, min_amount_b)
    }
    
}

