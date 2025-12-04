use anchor_lang::prelude::*;

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