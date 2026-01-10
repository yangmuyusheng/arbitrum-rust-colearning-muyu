use ethers::providers::{Http, Middleware, Provider};
use ethers::types::Address;
use ethers::utils::format_units;
use std::error::Error;

/// 查询指定地址在 Arbitrum 测试网的 ETH 余额
///
/// # 参数
/// * `address` - 要查询的以太坊地址（字符串格式，如 "0x..."）
///
/// # 返回
/// * `Result<String, Box<dyn Error>>` - 格式化后的余额（ETH 单位）
async fn get_balance(address: &str) -> Result<String, Box<dyn Error>> {
    // Arbitrum Sepolia 测试网 RPC URL
    let rpc_url = "https://Arbitrum-sepolia-rpc.publicnode.com";

    // 创建 HTTP Provider
    let provider = Provider::<Http>::try_from(rpc_url)?;

    // 解析地址
    let address: Address = address.parse()?;

    // 查询余额（返回 U256，单位为 wei）
    let balance = provider.get_balance(address, None).await?;

    // 将 wei 转换为 ETH（18 位小数）
    let balance_in_eth = format_units(balance, "ether")?;

    Ok(balance_in_eth)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
   
    let test_address = "0x51F14ab69C8f748F72b6DB1Aa66875faf7c24Bd2";

    println!("正在查询地址 {} 的余额...", test_address);

    match get_balance(test_address).await {
        Ok(balance) => {
            println!("余额: {} ETH", balance);
        }
        Err(e) => {
            eprintln!("查询余额失败: {}", e);
        }
    }

    Ok(())
}


