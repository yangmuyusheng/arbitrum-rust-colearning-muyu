use ethers::prelude::*;
use ethers::providers::{Http, Middleware, Provider};
use ethers::signers::{LocalWallet, Signer};
use ethers::types::{Address, TransactionRequest, U256};
use ethers::utils::{format_units, parse_ether};
use std::error::Error;
use std::str::FromStr;

// 基础 ETH 转账的 Gas 限额（行业通用值）
const BASIC_TRANSFER_GAS_LIMIT: u64 = 300000;
const RPC_URL: &str = "https://sepolia-rollup.arbitrum.io/rpc";

/// 获取 Arbitrum 测试网的实时 Gas 价格
///
/// # 参数
/// * `provider` - Provider 引用
///
/// # 返回
/// * `Result<U256, Box<dyn Error>>` - Gas 价格（单位：wei）
async fn get_gas_price(provider: &Provider<Http>) -> Result<U256, Box<dyn Error>> {
    let gas_price = provider.get_gas_price().await?;
    Ok(gas_price)
}

/// 验证地址格式是否正确
///
/// # 参数
/// * `address` - 地址字符串
///
/// # 返回
/// * `Result<Address, Box<dyn Error>>` - 解析后的地址
fn validate_address(address: &str) -> Result<Address, Box<dyn Error>> {
    let addr = Address::from_str(address)?;
    Ok(addr)
}

/// 查询地址余额
///
/// # 参数
/// * `provider` - Provider 引用
/// * `address` - 要查询的地址
///
/// # 返回
/// * `Result<U256, Box<dyn Error>>` - 余额（wei）
async fn get_balance(provider: &Provider<Http>, address: Address) -> Result<U256, Box<dyn Error>> {
    let balance = provider.get_balance(address, None).await?;
    Ok(balance)
}

/// 执行 ETH 转账
///
/// # 参数
/// * `private_key` - 私钥（从环境变量读取）
/// * `to_address` - 接收地址
/// * `amount_eth` - 转账金额（ETH）
///
/// # 返回
/// * `Result<TxHash, Box<dyn Error>>` - 交易哈希
async fn transfer_eth(
    private_key: &str,
    to_address: &str,
    amount_eth: &str,
) -> Result<TxHash, Box<dyn Error>> {
    println!("\n=== 开始转账流程 ===\n");

    // 1. 创建 Provider
    println!("1. 连接到 Arbitrum Sepolia 测试网...");
    let provider = Provider::<Http>::try_from(RPC_URL)?;
    println!("✓ 连接成功\n");

    // 2. 从私钥创建钱包
    println!("2. 加载钱包...");
    let wallet: LocalWallet = private_key.parse()?;
    let from_address = wallet.address();
    println!("✓ 发送地址: {}", from_address);

    // 3. 验证接收地址
    println!("\n3. 验证接收地址...");
    let to_address = validate_address(to_address)?;
    println!("✓ 接收地址: {}", to_address);

    // 4. 检查发送地址余额
    println!("\n4. 检查发送地址余额...");
    let balance = get_balance(&provider, from_address).await?;
    let balance_eth = format_units(balance, "ether")?;
    println!("✓ 当前余额: {} ETH", balance_eth);

    // 5. 解析转账金额
    let amount = parse_ether(amount_eth)?;
    println!("\n5. 转账金额: {} ETH ({} wei)", amount_eth, amount);

    // 6. 获取实时 Gas 价格
    println!("\n6. 获取实时 Gas 价格...");
    let gas_price = get_gas_price(&provider).await?;
    let gas_price_gwei = format_units(gas_price, "gwei")?;
    println!("✓ 当前 Gas 价格: {} Gwei", gas_price_gwei);

    // 7. 计算 Gas 费
    let gas_limit = U256::from(BASIC_TRANSFER_GAS_LIMIT);
    let gas_fee = gas_price * gas_limit;
    let gas_fee_eth = format_units(gas_fee, "ether")?;
    println!("✓ Gas 限额: {}", BASIC_TRANSFER_GAS_LIMIT);
    println!("✓ 预估 Gas 费: {} ETH", gas_fee_eth);

    // 8. 验证余额是否足够（金额 + Gas 费）
    let total_required = amount + gas_fee;
    if balance < total_required {
        return Err(format!(
            "余额不足！需要 {} ETH（转账 {} + Gas 费 {}），但只有 {} ETH",
            format_units(total_required, "ether")?,
            amount_eth,
            gas_fee_eth,
            balance_eth
        )
        .into());
    }
    println!("✓ 余额充足");

    // 9. 创建客户端（将钱包和 provider 绑定）
    println!("\n7. 准备交易...");
    let chain_id = provider.get_chainid().await?;
    let client = SignerMiddleware::new(provider.clone(), wallet.with_chain_id(chain_id.as_u64()));

    // 10. 构建交易
    let tx = TransactionRequest::new()
        .to(to_address)
        .value(amount)
        .gas(gas_limit)
        .gas_price(gas_price);

    println!("✓ 交易已构建");

    // 11. 签名并发送交易
    println!("\n8. 签名并发送交易...");
    let pending_tx = client.send_transaction(tx, None).await?;
    let tx_hash = pending_tx.tx_hash();
    println!("✓ 交易已发送！");
    println!("✓ 交易哈希: {:?}", tx_hash);

    // 12. 等待交易确认
    println!("\n9. 等待交易确认...");
    let receipt = pending_tx.await?;

    match receipt {
        Some(receipt) => {
            println!("✓ 交易已确认！");
            println!("  - 区块号: {:?}", receipt.block_number);
            println!("  - Gas 使用: {:?}", receipt.gas_used);
            println!("  - 状态: {:?}", receipt.status);
        }
        None => {
            println!("⚠ 交易已发送，但未收到确认收据");
        }
    }

    println!("\n=== 转账完成 ===");
    Ok(tx_hash)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("=== Arbitrum 测试网 ETH 转账工具 ===");

    // 从环境变量读取私钥（安全实践）
    dotenv::dotenv().ok(); // 加载 .env 文件（如果存在）

    let private_key = std::env::var("PRIVATE_KEY").unwrap_or_else(|_| {
        eprintln!("\n错误: 未找到 PRIVATE_KEY 环境变量！");
        eprintln!("\n请通过以下方式之一设置私钥:");
        eprintln!("1. 创建 .env 文件，添加: PRIVATE_KEY=your_private_key_here");
        eprintln!("2. 在命令行设置: set PRIVATE_KEY=your_private_key_here (Windows)");
        eprintln!("3. 在命令行设置: export PRIVATE_KEY=your_private_key_here (Unix/Linux/Mac)");
        eprintln!("\n⚠ 警告: 请勿将私钥硬编码在代码中！\n");
        std::process::exit(1);
    });

    // 接收地址（可以改成从命令行参数或环境变量读取）
    let to_address = std::env::var("TO_ADDRESS").unwrap_or_else(|_| {
        // 默认测试地址（可以替换）
        "0x741CD80d41eDE318feD4010E296704a061f4115a".to_string()
    });

    // 转账金额（ETH）
    let amount = std::env::var("AMOUNT").unwrap_or_else(|_| "0.001".to_string());

    // 执行转账
    match transfer_eth(&private_key, &to_address, &amount).await {
        Ok(tx_hash) => {
            println!("\n✅ 转账成功！");
            println!("交易哈希: {:?}", tx_hash);
            println!("\n查看交易: https://sepolia.arbiscan.io/tx/{:?}", tx_hash);
        }
        Err(e) => {
            eprintln!("\n❌ 转账失败: {}", e);
            std::process::exit(1);
        }
    }

    Ok(())
}

