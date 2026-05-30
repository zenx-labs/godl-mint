use serde::{Deserialize, Serialize};
use steel::*;

use super::GodlAccount;

/// Account which has the mint authority for the GODL token.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Pod, Zeroable, Serialize, Deserialize)]
pub struct Authority {
    /// The slot of the last mint.
    pub last_mint_at: u64,
}

account!(GodlAccount, Authority);
