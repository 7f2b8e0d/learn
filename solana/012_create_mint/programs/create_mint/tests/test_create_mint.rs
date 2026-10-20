use {
    anchor_lang::{
        solana_program::{instruction::Instruction, system_program},
        AccountDeserialize, InstructionData, ToAccountMetas,
    },
    anchor_spl::token::{self, Mint},
    litesvm::LiteSVM,
    solana_keypair::Keypair,
    solana_message::{Message, VersionedMessage},
    solana_signer::Signer,
    solana_transaction::versioned::VersionedTransaction,
};

#[test]
fn test_create_mint() {
    let program_id = create_mint::id();
    let payer = Keypair::new();
    let mint = Keypair::new();
    let mut svm = LiteSVM::new();
    let bytes = include_bytes!(concat!(
        env!("CARGO_TARGET_TMPDIR"),
        "/../deploy/create_mint.so"
    ));
    svm.add_program(program_id, bytes).unwrap();
    svm.airdrop(&payer.pubkey(), 1_000_000_000).unwrap();

    let instruction = Instruction::new_with_bytes(
        program_id,
        &create_mint::instruction::CreateMint {}.data(),
        create_mint::accounts::CreateMint {
            payer: payer.pubkey(),
            mint: mint.pubkey(),
            token_program: token::ID,
            system_program: system_program::ID,
        }
        .to_account_metas(None),
    );

    let blockhash = svm.latest_blockhash();
    let msg = Message::new_with_blockhash(&[instruction], Some(&payer.pubkey()), &blockhash);
    let tx = VersionedTransaction::try_new(VersionedMessage::Legacy(msg), &[&payer, &mint]).unwrap();

    let res = svm.send_transaction(tx);
    assert!(res.is_ok());

    //获取代币相关数据
    let account = svm.get_account(&mint.pubkey()).unwrap();
    let mut data: &[u8] = &account.data;
    let mint_state = Mint::try_deserialize(&mut data).unwrap();

    //对比小数精度
    assert_eq!(mint_state.decimals, 9);
    //对比发行量
    assert_eq!(mint_state.supply, 0);
    //对比代币发行者的公钥账户是不是payer的
    assert_eq!(mint_state.mint_authority.unwrap(), payer.pubkey());
}