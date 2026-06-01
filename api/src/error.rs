use steel::*;

#[derive(Debug, Error, Clone, Copy, PartialEq, Eq, IntoPrimitive)]
#[repr(u32)]
pub enum GodlMintError {
    #[error("Max amount exceeded")]
    MaxAmountExceeded = 0,

    #[error("Mint frequency exceeded")]
    MintFrequencyExceeded = 1,
}

error!(GodlMintError);
