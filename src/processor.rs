use borsh::BorshDeserialize;
use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, msg, program_error::ProgramError,
    pubkey::Pubkey,
};

use crate::instruction::VaultInstruction;

pub struct Processor;

impl Processor {
    pub fn process(
        _program_id: &Pubkey,
        _accounts: &[AccountInfo],
        instruction_data: &[u8],
    ) -> ProgramResult {
        let instruction: VaultInstruction = BorshDeserialize::try_from_slice(instruction_data)
            .map_err(|_| ProgramError::InvalidInstructionData)?;
        match instruction {
            VaultInstruction::InitVault {
                amount,
                guardian_thresholds,
                guardians,
            } => {
                msg!(
                    "amount: {:?}, guardian_tresholds: {:?}, guardians: {:?}",
                    amount,
                    guardian_thresholds,
                    guardians
                );
            }
        }
        Ok(())
    }
}
