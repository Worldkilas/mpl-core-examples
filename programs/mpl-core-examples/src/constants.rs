use anchor_lang::prelude::*;

#[constant]
pub const SEED: &str = "anchor";

#[constant]
pub const MPL_CORE_ID: Pubkey =
    Pubkey::from_str_const("CoREENxT6tW1HoK8ypY1SxRMZTcVPm7R94rH4PZNhX7d");

#[constant]
pub const ONCHAIN_METAPLEX_ORACLE_PLUGIN: Pubkey =
    Pubkey::from_str_const("AwPRxL5f6GDVajyE1bBcfSWdQT58nWMoS36A1uFtpCZY");

#[constant]
pub const SPL_NOOP_PROGRAM: Pubkey =
    Pubkey::from_str_const("noopb9bkMVfRPU8AsbpTUg8AQkHtKwMYZiFUjNRtMmV");
