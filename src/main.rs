//! Example of creating an HTTP provider using the `connect_http` method on the `ProviderBuilder`.
 
use alloy::providers::{Provider, ProviderBuilder}; 
use std::error::Error;
 
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Set up the HTTP transport which is consumed by the RPC client.
    let rpc_url = "https://Arbitrum-sepolia-rpc.publicnode.com".parse()?;
 
    // Create a provider with the HTTP transport using the `reqwest` crate.
    let provider = ProviderBuilder::new().connect_http(rpc_url); 

    let latest_block = provider.get_block_number().await?;
    println!("Latest block number: {latest_block}");
    println!("Hello web3");
    Ok(())
}