mod init;
mod mint_godl;

use init::*;
use mint_godl::*;

use godl_mint_api::instruction::*;
use solana_security_txt::security_txt;
use steel::*;

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    data: &[u8],
) -> ProgramResult {
    let (ix, data) = parse_instruction(&godl_mint_api::ID, program_id, data)?;

    match ix {
        GodlMintInstruction::Init => process_init(accounts, data)?,
        GodlMintInstruction::MintGODL => process_mint_godl(accounts, data)?,
    }

    Ok(())
}

entrypoint!(process_instruction);

security_txt! { 
    name: "GODL Mint",
    project_url: "https://godl.supply",
    contacts: "email:bootapollo@pm.me,telegram:bootapollo",
    policy: "https://github.com/zenx-labs/godl-mint/blob/main/SECURITY.md",
    preferred_languages: "en",
    source_code: "https://github.com/zenx-labs/godl-mint"
}
