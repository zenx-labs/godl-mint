use solana_program::pubkey::{pubkey, Pubkey};

/// The decimal precision of the GODL token.
/// There are 100 billion indivisible units per GODL (called "grams").
pub const TOKEN_DECIMALS: u8 = 11;

/// One GODL token, denominated in indivisible units.
pub const ONE_GODL: u64 = 10u64.pow(TOKEN_DECIMALS as u32);

/// The maximum amount of GODL tokens that can be minted per request.
pub const MAX_MINT_AMOUNT: u64 = ONE_GODL * 50;

/// The number of slots required between mint requests.
pub const MIN_SLOTS_BETWEEN_MINT: u64 = 125;

/// The seed of the automation account PDA.
pub const AUTHORITY: &[u8] = b"authority";

/// The address of the mint account.
pub const MINT_ADDRESS: Pubkey = pubkey!("GodL6KZ9uuUoQwELggtVzQkKmU1LfqmDokPibPeDKkhF");

/// The treasury address allowed to request a mint.
pub const TREASURY_ADDRESS: Pubkey = pubkey!("5epGzdW6veQwLQiQs1L45uUQ8jdSLQHWL8RbC7uTWVY3");
