use bs58;
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    commitment_config::CommitmentConfig,
    signature::{Keypair, Signer},
    system_instruction,
    transaction::Transaction,
};
use tokio::time::sleep;
use std::{fs::File, time::Duration};
use std::io::Write;


#[tokio::main]

async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _ = create_multiple_wallets(5).await;  // 使用 .await 调用异步函数
    airdrop().await?;  // 使用 .await，并处理错误
    check_balance().await?;  // 使用 .await，并处理错误
    transfer().await?;  // 使用 .await，并处理错误

    Ok(())
}

//1.创建i数量的钱包
async fn create_multiple_wallets(num: usize) -> Result<(), Box<dyn std::error::Error>>{
    for i in 0..num {
        // 创建一个新的钱包（Keypair）
        let keypair = Keypair::new();

        // 打印钱包的公钥和私钥
        println!("Wallet {}:", i + 1);
        println!("Public Key: {}", keypair.pubkey());
        println!(
            "Private Key (Base58): {:?}",
            bs58::encode(&keypair.to_bytes()).into_string()
        );
        println!("-------------------------------");

        // 可以将私钥存储到文件中，进行备份
        let mut file =
            File::create(format!("wallet_{}.json", i + 1)).expect("Unable to create file");
        let keypair_data = format!(
            "{{\"publicKey\": \"{}\", \"privateKey\": \"{:?}\"}}",
            keypair.pubkey(),
            bs58::encode(&keypair.to_bytes()).into_string()
        );
        file.write_all(keypair_data.as_bytes())
            .expect("Unable to write data");
    }
    Ok(())
}

// 2.领取空投
async fn airdrop() -> Result<(), Box<dyn std::error::Error>> {
    // 创建一个密钥对
    let keypair = Keypair::new();
    let pubkey = keypair.pubkey();

    // 连接到 Solana 网络
    let rpc_url = "https://api.devnet.solana.com"; // 这里使用 Devnet 网络
    let client = RpcClient::new_with_commitment(rpc_url.to_string(), CommitmentConfig::confirmed());

    // 请求空投
    sleep(Duration::from_secs(5)).await; 
    let airdrop_amount = 1_000_000_000; // 1 SOL, 空投的单位是 lamports
    client.request_airdrop(&pubkey, airdrop_amount)?;

    println!("请求空投成功，钱包地址: {}", pubkey);

    Ok(())
}

//3.查看余额

async fn check_balance() -> Result<(), Box<dyn std::error::Error>> {
    // 创建一个密钥对
    let keypair = Keypair::new();
    let pubkey = keypair.pubkey();

    // 连接到 Solana 网络
    let rpc_url = "https://api.devnet.solana.com"; // 这里使用 Devnet 网络
    let client = RpcClient::new_with_commitment(rpc_url.to_string(), CommitmentConfig::confirmed());

    // 获取余额
    let balance = client.get_balance(&pubkey)?;
    println!("钱包余额: {} lamports", balance);

    Ok(())
}

// 4.转账
async fn transfer() -> Result<(), Box<dyn std::error::Error>> {
    // 创建发送方和接收方的密钥对
    let from_keypair = Keypair::new();
    let from_pubkey = from_keypair.pubkey();
    let to_keypair = Keypair::new();
    let to_pubkey = to_keypair.pubkey();

    // 连接到 Solana 网络
    let rpc_url = "https://api.devnet.solana.com"; // 这里使用 Devnet 网络
    let client = RpcClient::new_with_commitment(rpc_url.to_string(), CommitmentConfig::confirmed());

    sleep(Duration::from_secs(5)).await; 
    // 请求空投到发送方钱包
    let airdrop_amount = 1_000_000_000; // 1 SOL
    client.request_airdrop(&from_pubkey, airdrop_amount)?;

    // 获取发送方和接收方的余额
    let from_balance_before = client.get_balance(&from_pubkey)?;
    let to_balance_before = client.get_balance(&to_pubkey)?;

    println!("发送方余额: {} lamports", from_balance_before);
    println!("接收方余额: {} lamports", to_balance_before);

    // 创建转账交易
    let transfer_amount = 500_000_000; // 转账 0.5 SOL
    let transfer_ix = system_instruction::transfer(&from_pubkey, &to_pubkey, transfer_amount);
    let recent_blockhash = client.get_latest_blockhash()?;
    let mut transaction = Transaction::new_with_payer(&[transfer_ix], Some(&from_pubkey));
    transaction.sign(&[&from_keypair], recent_blockhash);

    // 发送交易
    let signature = client.send_and_confirm_transaction(&transaction)?;
    println!("交易成功，签名: {}", signature);

    // 获取余额
    let from_balance_after = client.get_balance(&from_pubkey)?;
    let to_balance_after = client.get_balance(&to_pubkey)?;

    println!("发送方余额: {} lamports", from_balance_after);
    println!("接收方余额: {} lamports", to_balance_after);

    Ok(())
}


