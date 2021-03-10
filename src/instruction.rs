use std::str::FromStr;

use borsh::BorshDeserialize;
use solana_program::{
    instruction::{AccountMeta, Instruction, InstructionError},
    pubkey::Pubkey,
    sysvar::rent,
};

#[derive(BorshDeserialize, Debug)]
pub struct GuardianThresholds {
    pub freeze: u8,
    pub unfreeze: u8,
    pub change_key: u8,
    pub adjust_daily_withdrawal_limit: u8,
}

#[derive(BorshDeserialize, Debug)]
pub enum Proposal {
    Freeze,
    Unfreeze,
    ChangeOwnerKey,
    AdjustDailyWithdrawalLimit { new_limit: u128 },
}

#[derive(BorshDeserialize)]
pub enum VaultInstruction {
    /// Initializes the vault with given parameters
    ///
    /// Expected accounts:
    ///
    /// 0. `[signer]` user account
    /// 1. `[writable]` the main sol_as_token token account of the vault
    /// 2. `[]` mint account for 2
    /// 3. `[]` pda that will own 3
    /// 4. `[writable]` vault state account
    /// 5. `[]` rent sysvar
    /// 6. `[]` token program
    /// 7... `[]` guardian addresses
    InitVault,
    /*     Propose(Proposal),
    Confirm(Proposal),
    Withdraw,
    AddGuardian,
    RemoveGuardian,
    DestroyVault */
}

pub fn create_init_vault_instruction(
    user_address: &Pubkey,
    sol_vault_address: &Pubkey,
    vault_state_address: &Pubkey,
) -> Result<Instruction, InstructionError> {
    let pda = Pubkey::try_find_program_address(
        &[user_address.as_ref(), sol_vault_address.as_ref()],
        &crate::id(),
    )
    .ok_or(InstructionError::InvalidSeeds)?
    .0;

    Ok(Instruction {
        program_id: crate::id(),
        data: vec![],
        accounts: vec![
            AccountMeta::new_readonly(*user_address, true),
            AccountMeta::new(*sol_vault_address, false),
            AccountMeta::new_readonly(
                Pubkey::from_str("So11111111111111111111111111111111111111112").unwrap(),
                false,
            ),
            AccountMeta::new_readonly(pda, false),
            AccountMeta::new(*vault_state_address, false),
            AccountMeta::new_readonly(rent::id(), false),
            AccountMeta::new_readonly(spl_token::id(), false),
        ],
    })
}

/* #[cfg(test)]
mod test {
    use {super::*, assert_matches::*, solana_program::pubkey::Pubkey};

    #[test]
    fn test_transaction() {
        let mut instruction_data = vec![];
        instruction_data.push(0); // tag
        instruction_data.append(&mut vec![10, 0, 0, 0, 0, 0, 0, 0]); // amount
        instruction_data.append(&mut vec![2, 3, 4, 2]); // guardian thresholds
        instruction_data.append(&mut vec![5, 0, 0, 0]); // guardians vec size
        let guardian_array = [Pubkey::new_unique().to_bytes(); 5].to_vec();
        let mut guardians_u8 = guardian_array.iter().flatten().cloned().collect();
        instruction_data.append(&mut guardians_u8); // guardians
        let init_vault_ix: VaultInstruction =
            BorshDeserialize::try_from_slice(&instruction_data[..]).unwrap();
        match init_vault_ix {
            VaultInstruction::InitVault {
                guardian_thresholds,
                guardians,
            } => {
                assert_matches!(guardian_thresholds.freeze, 2);
                assert_matches!(guardian_thresholds.unfreeze, 3);
                assert_matches!(guardian_thresholds.change_key, 4);
                assert_matches!(guardian_thresholds.adjust_daily_withdrawal_limit, 2);
                // addresses is in fact used but the compiler does not see it for some reason
                let _addresses = guardian_array
                    .into_iter()
                    .map(|v| Address::from(Pubkey::new_from_array(v)));
                assert_matches!(guardians, _addresses);
            }
        }
    }
}
 */
