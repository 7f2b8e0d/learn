
use {
    anchor_lang::{
        prelude::Pubkey,
        solana_program::{instruction::Instruction, system_program},
        InstructionData, ToAccountMetas,
    },
    litesvm::LiteSVM,
    solana_keypair::Keypair,
    solana_message::{Message, VersionedMessage},
    solana_signer::Signer,
    solana_transaction::versioned::VersionedTransaction,
};

#[test]
fn test_withdraw() {
    let program_id = vault_back::id();
    let user = Keypair::new();
    let vault = Pubkey::find_program_address(
        &[vault_back::constants::VAULT_SEED, user.pubkey().as_ref()],
        &program_id,
    )
    .0;
    let mut svm = LiteSVM::new();
    let bytes = include_bytes!(concat!(
        env!("CARGO_TARGET_TMPDIR"),
        "/../deploy/vault_back.so"
    ));
    svm.add_program(program_id, bytes).unwrap();
    svm.airdrop(&user.pubkey(), 1_000_000_000).unwrap();
    svm.airdrop(&vault, 1_000_000_000).unwrap();

    let instruction = Instruction::new_with_bytes(
        program_id,
        &vault_back::instruction::Withdraw {amount: 100}.data(),
        vault_back::accounts::Withdraw {
            user: user.pubkey(),
            vault,
            system_program: system_program::ID,
        }
        .to_account_metas(None),
    );

    let blockhash = svm.latest_blockhash();
    let msg = Message::new_with_blockhash(&[instruction], Some(&user.pubkey()), &blockhash);
    let tx = VersionedTransaction::try_new(VersionedMessage::Legacy(msg), &[&user]).unwrap();

    let res = svm.send_transaction(tx);
    assert!(res.is_ok());

    let vault_account = svm.get_account(&vault).unwrap();
    let user_account = svm.get_account(&user.pubkey()).unwrap();
    let mut vault_data: &[u8] = &vault_account.data;
    let mut user_data: &[u8] = &user_account.data;
    println!("vault: {}", vault_account.lamports);
    println!("user: {}", user_account.lamports);
}
