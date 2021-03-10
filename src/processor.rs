use borsh::BorshDeserialize;
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    program::invoke,
    program_error::ProgramError,
    program_pack::Pack,
    pubkey::Pubkey,
    sysvar::{rent::Rent, Sysvar},
};

use spl_token::state::Account as TokenAccount;

use crate::serde::BorshPack;
use crate::state::Vault;
use crate::{instruction::VaultInstruction, state::Address};

struct TokenInitializeAccountParams<'a, 'b> {
    account: &'a AccountInfo<'b>,
    mint: &'a AccountInfo<'b>,
    owner: &'a AccountInfo<'b>,
    rent: &'a AccountInfo<'b>,
    token_program: &'a AccountInfo<'b>,
}

fn check_rent_exemption(rent: &Rent, account_info: &AccountInfo) -> ProgramResult {
    if rent.is_exempt(
        **account_info.try_borrow_lamports()?,
        account_info.try_borrow_data()?.len(),
    ) {
        Ok(())
    } else {
        Err(ProgramError::AccountNotRentExempt)
    }
}

fn spl_token_init_account(params: TokenInitializeAccountParams<'_, '_>) -> ProgramResult {
    let TokenInitializeAccountParams {
        account,
        mint,
        owner,
        rent,
        token_program,
    } = params;
    let ix = spl_token::instruction::initialize_account(
        token_program.key,
        account.key,
        mint.key,
        owner.key,
    )?;
    invoke(
        &ix,
        &[
            account.clone(),
            mint.clone(),
            owner.clone(),
            rent.clone(),
            token_program.clone(),
        ],
    )
}

pub struct Processor;

impl Processor {
    pub fn process(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        instruction_data: &[u8],
    ) -> ProgramResult {
        let instruction: VaultInstruction = BorshDeserialize::try_from_slice(instruction_data)
            .map_err(|_| ProgramError::InvalidInstructionData)?;
        match instruction {
            VaultInstruction::InitVault => Self::process_init_vault(accounts, program_id),
        }
    }

    fn process_init_vault(accounts: &[AccountInfo], program_id: &Pubkey) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        let user_info = next_account_info(account_info_iter)?;
        let sol_vault_info = next_account_info(account_info_iter)?;
        let sol_as_token_mint_info = next_account_info(account_info_iter)?;
        let pda_account_info = next_account_info(account_info_iter)?;
        let vault_state_info = next_account_info(account_info_iter)?;
        let rent_info = next_account_info(account_info_iter)?;
        let token_program_info = next_account_info(account_info_iter)?;

        if !user_info.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }

        if *sol_as_token_mint_info.key.to_string()
            != String::from("So11111111111111111111111111111111111111112")
        {
            return Err(ProgramError::InvalidAccountData);
        }

        let main_sol_vault_state = TokenAccount::unpack(*sol_vault_info.try_borrow_data()?)?;
        if main_sol_vault_state.mint != *sol_as_token_mint_info.key {
            return Err(ProgramError::InvalidAccountData);
        }

        let (pda, _bump_seed) = Pubkey::try_find_program_address(
            &[user_info.key.as_ref(), sol_vault_info.key.as_ref()],
            program_id,
        )
        .ok_or(ProgramError::InvalidArgument)?;

        if pda != *pda_account_info.key {
            return Err(ProgramError::InvalidAccountData);
        }

        spl_token_init_account(TokenInitializeAccountParams {
            account: sol_as_token_mint_info,
            mint: sol_as_token_mint_info,
            owner: pda_account_info,
            rent: rent_info,
            token_program: token_program_info,
        })?;

        let mut vault_state = Vault::unpack_unchecked(vault_state_info)?;
        if vault_state.is_initialized {
            return Err(ProgramError::AccountAlreadyInitialized);
        }

        let rent = Rent::from_account_info(rent_info)?;
        check_rent_exemption(&rent, vault_state_info)?;

        vault_state.owner_key = Address::from(user_info.key);
        vault_state.is_initialized = true;
        vault_state.is_frozen = false;
        vault_state.sol_vault_key = Address::from(sol_vault_info.key);
        let guardian_addresses: Vec<_> = account_info_iter.map(|a| Address::from(a.key)).collect();
        if guardian_addresses.len() < 4 {
            return Err(ProgramError::NotEnoughAccountKeys);
        }
        vault_state.guardians = guardian_addresses;
        // vault_state.tokens intentionally empty

        Ok(())
    }
}
