use steel::*;

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, TryFromPrimitive)]
pub enum GodlMintInstruction {
    Init = 0,
    MintGODL = 1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct Init {}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct MintGODL {
    pub amount: [u8; 8],
}

instruction!(GodlMintInstruction, Init);
instruction!(GodlMintInstruction, MintGODL);
