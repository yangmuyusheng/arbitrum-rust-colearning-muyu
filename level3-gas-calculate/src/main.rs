use ethers::providers::{Http, Middleware, Provider};
use ethers::types::U256;
use ethers::utils::format_units;
use std::error::Error;

// 基础 ETH 转账的 Gas 限额（行业通用值）
const BASIC_TRANSFER_GAS_LIMIT: u64 = 21000;

/// 获取 Arbitrum 测试网的实时 Gas 价格
///
/// # 返回
/// * `Result<U256, Box<dyn Error>>` - Gas 价格（单位：wei）
async fn get_gas_price() -> Result<U256, Box<dyn Error>> {
    // Arbitrum Sepolia 测试网 RPC URL
    let rpc_url = "https://sepolia-rollup.arbitrum.io/rpc";

    // 创建 HTTP Provider
    let provider = Provider::<Http>::try_from(rpc_url)?;

    // 获取当前 Gas 价格
    let gas_price = provider.get_gas_price().await?;

    Ok(gas_price)
}

/// 计算预估转账 Gas 费
///
/// # 参数
/// * `gas_limit` - Gas 限额（可选，默认使用基础转账的 21000）
///
/// # 返回
/// * `Result<(String, String, String), Box<dyn Error>>` - (Gas价格(Gwei), Gas限额, Gas费(ETH))
async fn calculate_gas_fee(gas_limit: Option<u64>) -> Result<(String, String, String), Box<dyn Error>> {
    // 获取实时 Gas 价格
    let gas_price = get_gas_price().await?;

    // 使用提供的 Gas 限额，或默认使用基础转账的 21000
    let gas_limit = gas_limit.unwrap_or(BASIC_TRANSFER_GAS_LIMIT);

    // 计算 Gas 费：Gas 价格 × Gas 限额
    let gas_fee = gas_price * U256::from(gas_limit);

    // 格式化输出
    let gas_price_gwei = format_units(gas_price, "gwei")?;
    let gas_fee_eth = format_units(gas_fee, "ether")?;

    Ok((
        gas_price_gwei,
        gas_limit.to_string(),
        gas_fee_eth,
    ))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("=== Arbitrum 测试网 Gas 费计算 ===\n");

    // 1. 获取实时 Gas 价格
    println!("正在获取实时 Gas 价格...");
    let gas_price = get_gas_price().await?;
    let gas_price_gwei = format_units(gas_price, "gwei")?;
    println!("当前 Gas 价格: {} Gwei", gas_price_gwei);
    println!("当前 Gas 价格 (wei): {}\n", gas_price);

    // 2. 计算基础转账的 Gas 费
    println!("--- 基础 ETH 转账 Gas 费计算 ---");
    let (price, limit, fee) = calculate_gas_fee(None).await?;
    println!("Gas 价格: {} Gwei", price);
    println!("Gas 限额: {}", limit);
    println!("预估 Gas 费: {} ETH\n", fee);

    Ok(())
}

