use anchor_lang::prelude::*;

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
