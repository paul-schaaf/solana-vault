use borsh::BorshDeserialize;

use crate::state::Address;

#[derive(BorshDeserialize, Debug)]
pub struct GuardianThresholds {
    pub freeze: u8,
    pub unfreeze: u8,
    pub change_key: u8,
    pub adjust_daily_widthdrawal_limit: u8,
}

#[derive(BorshDeserialize)]
pub enum VaultInstruction {
    /// Initializes the vault with given parameters
    ///
    /// Expected accounts:
    ///
    /// 0. `[signer]` vault owner
    /// 1. `[writable]` token account that tokens will be transferred from to the vault
    /// 2. `[writable]` the token account of the vault that tokens will be transferred to
    InitVault {
        /// initial amount to be transferred to vault
        amount: u64,
        guardian_thresholds: GuardianThresholds,
        guardians: Vec<Address>,
    },
}

#[cfg(test)]
mod test {
    use {
        super::*,
        assert_matches::*,
        solana_program::pubkey::Pubkey
    };

    #[test]
    fn test_transaction() {
        let mut instruction_data = vec![];
        instruction_data.push(0); // tag
        instruction_data.append(&mut vec![10, 0, 0, 0, 0, 0, 0, 0]); // amount
        instruction_data.append(&mut vec![2, 3, 4, 2]); // guardian thresholds
        instruction_data.append(&mut vec![5, 0, 0, 0]); // guardians vec size
        let guardian_array = [Pubkey::new_unique().to_bytes(); 5].to_vec();
        let mut guardians_u8 = guardian_array
            .iter()
            .flatten()
            .cloned()
            .collect();
        instruction_data.append(&mut guardians_u8); // guardians
        let init_vault_ix: VaultInstruction =
            BorshDeserialize::try_from_slice(&instruction_data[..]).unwrap();
        match init_vault_ix {
            VaultInstruction::InitVault {
                amount,
                guardian_thresholds,
                guardians,
            } => {
                assert_matches!(amount, 10);
                assert_matches!(guardian_thresholds.freeze, 2);
                assert_matches!(guardian_thresholds.unfreeze, 3);
                assert_matches!(guardian_thresholds.change_key, 4);
                assert_matches!(guardian_thresholds.adjust_daily_widthdrawal_limit, 2);
                // addresses is in fact used but the compiler does not see it for some reason
                let _addresses = guardian_array.into_iter().map(|v| {Address::from(Pubkey::new_from_array(v))});
                assert_matches!(guardians, _addresses);
            }
        }
    }
}
