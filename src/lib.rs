use {
    solana_program::{
        account_info::{next_account_info, AccountInfo},
        entrypoint::ProgramResult,
        msg,
        program::invoke_signed,
        program_error::ProgramError,
        program_pack::Pack,
        pubkey::Pubkey,
    },
    spl_token::{instruction::transfer_checked, state::Mint},
};

solana_program::entrypoint!(process_instruction);
pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    _instruction_data: &[u8],
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();

    let source_info = next_account_info(account_info_iter)?;
    let mint_info = next_account_info(account_info_iter)?;
    let destination_info = next_account_info(account_info_iter)?;
    let authority_info = next_account_info(account_info_iter)?;
    let token_program_info = next_account_info(account_info_iter)?;

    let (expected_authority, bump_seed) = Pubkey::find_program_address(&[b"authority"], program_id);

    if expected_authority != *authority_info.key {
        return Err(ProgramError::InvalidSeeds);
    }

    let amount = 2000;

    let mint = Mint::unpack(&mint_info.try_borrow_data()?)?;

    let decimals = mint.decimals;

    // Invoke Transfer from spl_token
    msg!("Attempting to transfer {} tokens", amount);

    invoke_signed(
        &transfer_checked(
            token_program_info.key,
            source_info.key,
            mint_info.key,
            destination_info.key,
            authority_info.key,
            &[],
            amount,
            decimals,
        )
        .unwrap(),
        &[
            source_info.clone(),
            mint_info.clone(),
            destination_info.clone(),
            authority_info.clone(),
            token_program_info.clone(),
        ],
        &[&[b"authority", &[bump_seed]]],
    )
}
