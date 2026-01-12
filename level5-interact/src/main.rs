use ethers::prelude::*;
use ethers::abi::Abi;
use ethers::providers::{Http, Provider};
use ethers::types::Address;
use ethers::utils::format_units;
use std::error::Error;
use std::str::FromStr;
use std::sync::Arc;

const RPC_URL: &str = "https://sepolia-rollup.arbitrum.io/rpc";

// Arbitrum Sepolia 测试网上的 USDC 测试代币合约地址
const USDC_CONTRACT_ADDRESS: &str = "0x75faf114eafb1BDbe2F0316DF893fd58CE46AA4d";

// ERC20 标准 ABI
const ERC20_ABI: &str = r#"[
    {
        "constant": true,
        "inputs": [],
        "name": "name",
        "outputs": [{"name": "", "type": "string"}],
        "type": "function"
    },
    {
        "constant": true,
        "inputs": [],
        "name": "symbol",
        "outputs": [{"name": "", "type": "string"}],
        "type": "function"
    }
   
]"#;

/// 查询 ERC20 代币的基本信息
///
/// # 参数
/// * `contract_address` - 合约地址
///
/// # 返回
/// * `Result<(), Box<dyn Error>>` - 执行结果
async fn query_erc20_info(contract_address: &str) -> Result<(), Box<dyn Error>> {
    println!("=== Arbitrum 测试网合约交互演示 ===\n");

    // 1. 创建 Provider
    println!("1. 连接到 Arbitrum Sepolia 测试网...");
    let provider = Provider::<Http>::try_from(RPC_URL)?;
    let provider = Arc::new(provider);
    println!("✓ 连接成功\n");

    // 2. 解析合约地址
    println!("2. 加载合约...");
    let address = Address::from_str(contract_address)?;
    println!("✓ 合约地址: {}", address);

    // 3. 解析 ABI
    let abi: Abi = serde_json::from_str(ERC20_ABI)?;
    println!("✓ ABI 加载成功\n");

    // 4. 创建合约实例
    let contract = Contract::new(address, abi, provider.clone());
    println!("3. 合约实例已创建\n");

    // 5. 调用合约的只读方法
    println!("4. 查询合约信息...\n");

    // 查询代币名称
    println!("📝 调用 name() 方法...");
    let name: String = contract.method("name", ())?.call().await?;
    println!("✓ 代币名称: {}", name);

    // 查询代币符号
    println!("\n📝 调用 symbol() 方法...");
    let symbol: String = contract.method("symbol", ())?.call().await?;
    println!("✓ 代币符号: {}", symbol);

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("使用 Arbitrum Sepolia 测试网上的 USDC 测试代币\n");

    match query_erc20_info(USDC_CONTRACT_ADDRESS).await {
        Ok(_) => println!("\n✅ 查询成功！"),
        Err(e) => {
            eprintln!("\n❌ 查询失败: {}", e);
            std::process::exit(1);
        }
    }

    Ok(())
}


