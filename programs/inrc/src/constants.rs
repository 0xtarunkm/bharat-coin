use anchor_lang::prelude::*;

#[constant]
pub const SOL_USD_FEED_ID: &str =
    "0xef0d8b6fda2ceba41da15d4095d1da392a0d2f8ed0c6c7bc0f4cfac8c280b56d";
pub const SEED_CONFIG_ACCOUNT: &[u8] = b"config";
pub const SEED_MINT_ACCOUNT: &[u8] = b"mint";
pub const SEED_COLLATERAL_ACCOUNT: &[u8] = b"collateral_treasury";
pub const SEED_SOL_ACCOUNT: &[u8] = b"sol";

pub const ANCHOR_DISCRIMINATOR: usize = 8;
pub const MINT_DECIMAL: u8 = 6;
pub const MAX_AGE: u64 = 100;
pub const PRICE_FEED_DECIMAL_ADJUSTMENT: u128 = 10;
