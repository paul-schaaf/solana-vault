pub mod error;
pub mod instruction;
pub mod processor;
pub mod serde;
pub mod state;

solana_program::declare_id!("SRW333GPvbdGVxr1b333ZbsiW5xWH25efTNsLJA8knL"); // TODO: create real id

#[cfg(not(feature = "no-entrypoint"))]
pub mod entrypoint;
