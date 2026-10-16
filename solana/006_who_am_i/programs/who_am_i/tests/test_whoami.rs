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
fn test_who_am_i() {
    //获取程序id
    let program_id = who_am_i::id();
    //新建付款人
    let payer = Keypair::new();
    let alice = Keypair::new();
    //新建虚拟机
    let mut svm = LiteSVM::new();
    //读入编译好的智能合约文件
    let bytes = include_bytes!(concat!(
        env!("CARGO_TARGET_TMPDIR"),
        "/../deploy/who_am_i.so"
    ));
    //添加智能合约到虚拟机
    svm.add_program(program_id, bytes).unwrap();
    //给付款人空投1 SOL
    svm.airdrop(&payer.pubkey(), 1_000_000_000).unwrap();

    //生成指令
    let inspect_ix = Instruction::new_with_bytes(
        program_id,
        &who_am_i::instruction::Whoami {}.data(),
        who_am_i::accounts::WhoAmI {user: alice.pubkey()}.to_account_metas(None),
    );

    //获取最后区块hash
    let blockhash = svm.latest_blockhash();
    //生成消息
    let msg = Message::new_with_blockhash(&[inspect_ix,], Some(&payer.pubkey()), &blockhash);
    //付款人签名，生成交易
    let tx = VersionedTransaction::try_new(VersionedMessage::Legacy(msg), &[&payer, &alice]).unwrap();
    //执行并 获取交易信息
    let meta = svm.send_transaction(tx).unwrap();
    println!("{:#?}", meta.logs);
    //拼接对比日志字符串
    let log_owner_str = format!("Program log: user: {}", alice.pubkey());
    //检查日志是否正确
    assert!(meta.logs.iter().any(|log| *log == log_owner_str));
}
