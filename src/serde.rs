use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::AccountInfo, program_error::ProgramError, program_pack::IsInitialized,
};

use std::ops::DerefMut;

/// Safely and efficiently (de)serialize account state
pub trait BorshPack: Sized + BorshDeserialize + BorshSerialize {
    /// The length, in bytes, of the packed representation
    const LEN: usize;

    /// Get the packed length
    fn get_packed_len() -> usize {
        Self::LEN
    }

    /// Unpack from account and check if initialized
    fn unpack(account_info: &AccountInfo) -> Result<Self, ProgramError>
    where
        Self: IsInitialized,
    {
        let state = Self::unpack_unchecked(account_info)?;
        if state.is_initialized() {
            Ok(state)
        } else {
            Err(ProgramError::UninitializedAccount)
        }
    }

    /// Unpack from account without checking if initialized
    fn unpack_unchecked(account_info: &AccountInfo) -> Result<Self, ProgramError> {
        Self::try_from_slice(&account_info.data.borrow())
            .map_err(|_| ProgramError::InvalidAccountData)
    }

    /// Pack into account with length check
    fn pack(&self, account_info: &AccountInfo) -> Result<(), ProgramError> {
        if account_info.try_borrow_data()?.len() != Self::LEN {
            return Err(ProgramError::InvalidAccountData);
        }
        self.pack_into_account_info(account_info)
    }

    /// Pack into account without length check
    fn pack_into_account_info(&self, account_info: &AccountInfo) -> Result<(), ProgramError> {
        BorshSerialize::serialize(self, account_info.try_borrow_mut_data()?.deref_mut())
            .map_err(|_| ProgramError::AccountDataTooSmall)
    }
}
