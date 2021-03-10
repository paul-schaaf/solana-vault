use borsh::schema::{Declaration, Definition};
use borsh::{BorshDeserialize, BorshSchema, BorshSerialize};
use solana_program::{program_option::COption, pubkey::Pubkey};

use std::collections::HashMap;
use std::io::{self, Write};

use crate::serde::BorshPack;

/// Wrapper around `solana_program::pubkey::Pubkey` so it can implement `BorshSerialize` etc.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Address(Pubkey);

impl From<Address> for Pubkey {
    fn from(address: Address) -> Self {
        address.0
    }
}

impl AsRef<Pubkey> for Address {
    fn as_ref(&self) -> &Pubkey {
        &self.0
    }
}

impl AsMut<Pubkey> for Address {
    fn as_mut(&mut self) -> &mut Pubkey {
        &mut self.0
    }
}

impl From<Pubkey> for Address {
    fn from(pubkey: Pubkey) -> Self {
        Self(pubkey)
    }
}

impl From<&Pubkey> for Address {
    fn from(pubkey: &Pubkey) -> Self {
        Self(*pubkey)
    }
}

impl BorshSerialize for Address {
    fn serialize<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        BorshSerialize::serialize(&self.0.to_bytes(), writer)
    }
}

impl BorshDeserialize for Address {
    fn deserialize(buf: &mut &[u8]) -> io::Result<Self> {
        Ok(Self(Pubkey::new_from_array(BorshDeserialize::deserialize(
            buf,
        )?)))
    }
}

impl BorshSchema for Address {
    fn add_definitions_recursively(definitions: &mut HashMap<Declaration, Definition>) {
        Self::add_definition(
            Self::declaration(),
            Definition::Struct {
                fields: borsh::schema::Fields::UnnamedFields(vec![
                    <[u8; 32] as BorshSchema>::declaration(),
                ]),
            },
            definitions,
        );
        <[u8; 32] as BorshSchema>::add_definitions_recursively(definitions);
    }

    fn declaration() -> Declaration {
        "Address".to_string()
    }
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct Vault {
    pub is_initialized: bool,
    pub owner_key: Address,
    pub sol_vault_key: Address,
    pub sol_vault_daily_withdrawal_limit: Option<u128>,
    pub guardians: Vec<Address>,
    pub is_frozen: bool,
    pub tokens: Vec<Token>,
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct Token {
    mint: Address,
    daily_withdrawal_limit: Option<u128>,
}

impl BorshPack for Vault {
    /// 1 (is_initialized) + 32 (owner key) + 32 (main sol vault key) + 129 (1 (option byte) + 128 (sol vault limit))
    /// + (4(borsh vector length header) + 640(20 guardians)) + 1 (is_frozen)
    /// + (4(borsh vector length header) + 960 (30 mints) + 30 (borsh option bytes) + 3840 (30 u128s))
    const LEN: usize = 5544;
}

/*
TODO: FUTURE
pub struct Guardian {
    name: String,
    key: Address,
} */
