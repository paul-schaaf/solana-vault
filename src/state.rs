use solana_program::pubkey::Pubkey;

use borsh::{BorshDeserialize, BorshSchema, BorshSerialize};
use std::collections::HashMap;
use std::{io, io::Write};

use borsh::schema::{Declaration, Definition};
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
