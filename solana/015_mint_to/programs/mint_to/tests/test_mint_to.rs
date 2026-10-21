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
use anchor_spl::token::TokenAccount;

#[test]
fn test_create_mint() {
    let program_id = mint_to::id();
    let payer = Keypair::new();
    let mint = Keypair::new();
    let mut svm = LiteSVM::new();
    let bytes = include_bytes!(concat!(
        env!("CARGO_TARGET_TMPDIR"),
        "/../deploy/mint_to.so"
    ));
    svm.add_program(program_id, bytes).unwrap();
    svm.airdrop(&payer.pubkey(), 1_000_000_000).unwrap();

    let instruction = Instruction::new_with_bytes(
        program_id,
        &mint_to::instruction::CreateMint {}.data(),
        mint_to::accounts::CreateMint {
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

    let token_account = Keypair::new();
    svm.airdrop(&token_account.pubkey(), 1_000_000_000).unwrap();

    let instruction = Instruction::new_with_bytes(
        program_id,
        &mint_to::instruction::CreateTokenAccount {}.data(),
        mint_to::accounts::CreateTokenAccount {
            payer: payer.pubkey(),
            mint: mint.pubkey(),
            token_account: token_account.pubkey(),
            token_program: token::ID,
            system_program: system_program::ID,
        }
        .to_account_metas(None),
    );

    let blockhash = svm.latest_blockhash();
    let msg = Message::new_with_blockhash(&[instruction], Some(&payer.pubkey()), &blockhash);
    let tx = VersionedTransaction::try_new(VersionedMessage::Legacy(msg), &[&payer, &token_account]).unwrap();

    let res = svm.send_transaction(tx);
    assert!(res.is_ok());

    //获取代币相关数据
    let account = svm.get_account(&token_account.pubkey()).unwrap();
    let mut data: &[u8] = &account.data;
    let token_account_state = TokenAccount::try_deserialize(&mut data).unwrap();

    assert_eq!(token_account_state.owner, payer.pubkey());
    assert_eq!(token_account_state.amount, 0);
    assert_eq!(token_account_state.mint, mint.pubkey());

     let instruction = Instruction::new_with_bytes(
        program_id,
        &mint_to::instruction::MintTo {amount: 100}.data(),
        mint_to::accounts::MintTo {
            payer: payer.pubkey(),
            mint: mint.pubkey(),
            token_account: token_account.pubkey(),
            token_program: token::ID,
        }
        .to_account_metas(None),
    );

    let blockhash = svm.latest_blockhash();
    let msg = Message::new_with_blockhash(&[instruction], Some(&payer.pubkey()), &blockhash);
    let tx = VersionedTransaction::try_new(VersionedMessage::Legacy(msg), &[&payer]).unwrap();

    let res = svm.send_transaction(tx);
    assert!(res.is_ok());

    //获取代币相关数据
    let account = svm.get_account(&token_account.pubkey()).unwrap();
    let mut data: &[u8] = &account.data;
    let token_account_state = TokenAccount::try_deserialize(&mut data).unwrap();

    assert_eq!(token_account_state.amount, 100);
}