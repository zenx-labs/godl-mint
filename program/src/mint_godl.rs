use godl_mint_api::prelude::*;
use spl_token::amount_to_ui_amount;
use steel::*;

/// Mints GODL to the treasury.
pub fn process_mint_godl(accounts: &[AccountInfo<'_>], data: &[u8]) -> ProgramResult {
    // Parse data.
    let args = MintGODL::try_from_bytes(data)?;
    let amount = u64::from_le_bytes(args.amount);

    // Load accounts.
    let clock = Clock::get()?;
    let [signer_info, authority_info, mint_info, to_info, token_program] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };
    signer_info.is_signer()?.has_address(&TREASURY_ADDRESS)?;
    let authority = authority_info
        .has_seeds(&[AUTHORITY], &godl_mint_api::ID)?
        .as_account_mut::<Authority>(&godl_mint_api::ID)?;
    mint_info.has_address(&MINT_ADDRESS)?.as_mint()?;
    to_info.as_associated_token_account(&TREASURY_ADDRESS, &MINT_ADDRESS)?;
    token_program.is_program(&spl_token::ID)?;

    // Check max mint amount.
    if amount > MAX_MINT_AMOUNT {
        return Err(trace(
            format!(
                "Cannot mint more than max allowed: {} > {}",
                amount_to_ui_amount(amount, TOKEN_DECIMALS),
                amount_to_ui_amount(MAX_MINT_AMOUNT, TOKEN_DECIMALS)
            )
            .as_str(),
            GodlMintError::MaxAmountExceeded.into(),
        ));
    }

    // Check mint frequency.
    let slots_since = clock.slot.saturating_sub(authority.last_mint_at);
    if slots_since < MIN_SLOTS_BETWEEN_MINT {
        return Err(trace(
            format!(
                "Must wait at least {} slots between mints: {} slots remaining",
                MIN_SLOTS_BETWEEN_MINT,
                MIN_SLOTS_BETWEEN_MINT.saturating_sub(slots_since)
            )
            .as_str(),
            GodlMintError::MintFrequencyExceeded.into(),
        ));
    }

    // Update timestamp.
    authority.last_mint_at = clock.slot;

    // Mint tokens to the treasury.
    mint_to_signed(
        mint_info,
        to_info,
        authority_info,
        token_program,
        amount,
        &[AUTHORITY],
    )?;

    Ok(())
}
