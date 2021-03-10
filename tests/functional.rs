use solana_program::pubkey::Pubkey;
use solana_program::{
    hash::Hash, instruction::InstructionError, program_pack::Pack, sysvar, sysvar::rent::Rent,
};
use solana_program_test::{processor, BanksClient, ProgramTest};

use solana_sdk::{
    signature::{Keypair, Signer},
    transaction::Transaction,
};

fn program_test() -> ProgramTest {
    let mut pc = ProgramTest::new(
        "SRW",
        Pubkey::new_unique(),
        processor!(SRW::processor::Processor::process),
    );

    // Dial down the BPF compute budget to detect if the program gets bloated in the future
    pc.set_bpf_compute_max_units(50_000);

    pc
}

async fn make_account_rent_exempt(
    payer: &Keypair,
    account_address: &Pubkey,
    size: usize,
    recent_blockhash: &Hash,
    banks_client: &mut BanksClient,
    rent: &Rent,
) {
    let transfer_rent_ix = solana_program::system_instruction::transfer(
        &payer.pubkey(),
        account_address,
        rent.minimum_balance(size),
    );

    let transfer_tx = Transaction::new_signed_with_payer(
        &[transfer_rent_ix],
        Some(&payer.pubkey()),
        &[payer],
        *recent_blockhash,
    );

    banks_client.process_transaction(transfer_tx).await.unwrap();
}

#[tokio::test]
async fn test_init_vault() -> Result<(), InstructionError> {
    let sol_vault_keypair = Keypair::new();
    let vault_state_keypair = Keypair::new();

    let (mut banks_client, payer, recent_blockhash) = program_test().start().await;
    let rent = banks_client.get_rent().await.unwrap();

    make_account_rent_exempt(
        &payer,
        &sol_vault_keypair.pubkey(),
        spl_token::state::Account::LEN,
        &recent_blockhash,
        &mut banks_client,
        &rent,
    )
    .await;

    let create_sol_vault_ix = solana_program::system_instruction::create_account(
        &payer.pubkey(),
        &sol_vault_keypair.pubkey(),
        rent.minimum_balance(spl_token::state::Account::LEN),
        spl_token::state::Account::LEN as u64,
        &spl_token::id(),
    );

    let init_vault_ix = SRW::instruction::create_init_vault_instruction(
        &payer.pubkey(),
        &sol_vault_keypair.pubkey(),
        &vault_state_keypair.pubkey(),
    )?;

    let transaction = Transaction::new_signed_with_payer(
        &[create_sol_vault_ix, init_vault_ix],
        Some(&payer.pubkey()),
        &[&payer, &sol_vault_keypair],
        recent_blockhash,
    );

    banks_client.process_transaction(transaction).await.unwrap();

    Ok(())
}
