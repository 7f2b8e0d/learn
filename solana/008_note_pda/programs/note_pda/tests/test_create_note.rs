
use {
    anchor_lang::{
        prelude::Pubkey,
        solana_program::{instruction::Instruction, system_program},
        AccountDeserialize, InstructionData, ToAccountMetas,
    },
    litesvm::LiteSVM,
    solana_keypair::Keypair,
    solana_message::{Message, VersionedMessage},
    solana_signer::Signer,
    solana_transaction::versioned::VersionedTransaction,
};

#[test]
fn test_initialize() {
    let program_id = note_pda::id();
    let author = Keypair::new();
    let counter = Pubkey::find_program_address(
        &[note_pda::constants::NOTE_SEED, author.pubkey().as_ref()],
        &program_id,
    )
    .0;
    let mut svm = LiteSVM::new();
    let bytes = include_bytes!(concat!(
        env!("CARGO_TARGET_TMPDIR"),
        "/../deploy/note_pda.so"
    ));
    svm.add_program(program_id, bytes).unwrap();
    svm.airdrop(&author.pubkey(), 1_000_000_000).unwrap();

    let instruction = Instruction::new_with_bytes(
        program_id,
        &note_pda::instruction::CreateNote {message: "...这是笔记".to_string()}.data(),
        note_pda::accounts::CreateNote {
            author: author.pubkey(),
            note: counter,
            system_program: system_program::ID,
        }
        .to_account_metas(None),
    );

    let blockhash = svm.latest_blockhash();
    let msg = Message::new_with_blockhash(&[instruction], Some(&author.pubkey()), &blockhash);
    let tx = VersionedTransaction::try_new(VersionedMessage::Legacy(msg), &[&author]).unwrap();

    let res = svm.send_transaction(tx);
    assert!(res.is_ok());

    let counter_account = svm.get_account(&counter).unwrap();
    let mut data: &[u8] = &counter_account.data;
    let counter_state = note_pda::state::Note::try_deserialize(&mut data).unwrap();
    println!("{}", counter_state.author);
    println!("{}", counter_state.message);
    assert_eq!(counter_state.author, author.pubkey());
    assert_eq!(counter_state.message, "...这是笔记".to_string());
}
