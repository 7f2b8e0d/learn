use {
    anchor_lang::{
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
fn test_transfer_sol() {
    let program_id = transfer_sol::id();
    let payer = Keypair::new();
    let to = Keypair::new();
    let mut svm = LiteSVM::new();
    let bytes = include_bytes!(concat!(
        env!("CARGO_TARGET_TMPDIR"),
        "/../deploy/transfer_sol.so"
    ));
    svm.add_program(program_id, bytes).unwrap();
    svm.airdrop(&payer.pubkey(), 1_000_000_000).unwrap();

    let from_balance_before = svm.get_balance(&payer.pubkey()).unwrap();
    let to_balance_before = svm.get_balance(&to.pubkey()).unwrap_or(0);
    println!("from_balance_before: {}", from_balance_before);
    println!("to_balance_before: {}", to_balance_before);

    let instruction = Instruction::new_with_bytes(
        program_id,
        &transfer_sol::instruction::TransferSol {amount: 1_000_000}.data(),
        transfer_sol::accounts::TransferSol {
            from: payer.pubkey(),
            to: to.pubkey(),
            system_program: system_program::ID,
        }
        .to_account_metas(None),
    );

    let blockhash = svm.latest_blockhash();
    let msg = Message::new_with_blockhash(&[instruction], Some(&payer.pubkey()), &blockhash);
    let tx = VersionedTransaction::try_new(VersionedMessage::Legacy(msg), &[&payer]).unwrap();

    let res = svm.send_transaction(tx);
    assert!(res.is_ok());
    let from_balance_after = svm.get_balance(&payer.pubkey()).unwrap();
    let to_balance_after = svm.get_balance(&to.pubkey()).unwrap();
    println!("from_balance_after: {}", from_balance_after);
    println!("to_balance_after: {}", to_balance_after);
    assert_eq!(to_balance_after, to_balance_before + 1_000_000);
}
