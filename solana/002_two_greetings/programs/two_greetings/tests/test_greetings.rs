use {
    anchor_lang::{
        solana_program::{instruction::Instruction},
        InstructionData, ToAccountMetas,
    },
    litesvm::LiteSVM,
    solana_keypair::Keypair,
    solana_message::{Message, VersionedMessage},
    solana_signer::Signer,
    solana_transaction::versioned::VersionedTransaction,
};

#[test]
fn test_greetings() {
    //获取程序id
    let program_id = two_greetings::id();
    //新建付款人
    let payer = Keypair::new();
    //新建虚拟机
    let mut svm = LiteSVM::new();
    //读入编译好的智能合约文件
    let bytes = include_bytes!(concat!(
        env!("CARGO_TARGET_TMPDIR"),
        "/../deploy/two_greetings.so"
    ));
    //添加智能合约到虚拟机
    svm.add_program(program_id, bytes).unwrap();
    //给付款人空投1 SOL
    svm.airdrop(&payer.pubkey(), 1_000_000_000).unwrap();

    //生成指令
    let hello_ix = Instruction::new_with_bytes(
        program_id,
        &two_greetings::instruction::Hello {}.data(),
        two_greetings::accounts::Hello {}.to_account_metas(None),
    );

    //生成指令
    let bye_ix = Instruction::new_with_bytes(
        program_id,
        &two_greetings::instruction::Bye {}.data(),
        two_greetings::accounts::Bye {}.to_account_metas(None),
    );

    //获取最后区块hash
    let blockhash = svm.latest_blockhash();
    //生成消息
    let msg = Message::new_with_blockhash(&[hello_ix, bye_ix], Some(&payer.pubkey()), &blockhash);
    //付款人签名，生成交易
    let tx = VersionedTransaction::try_new(VersionedMessage::Legacy(msg), &[&payer]).unwrap();
    //执行并 获取交易信息
    let meta = svm.send_transaction(tx).unwrap();
    //检查日志是否正确
    assert!(meta.logs.iter().any(|log| log == "Program log: Hello, world!"));
    assert!(meta.logs.iter().any(|log| log == "Program log: Bye, world!"));
}
