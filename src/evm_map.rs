use crate::cli::Chain;
use alloy::primitives::Address;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct EvmContract {
    address: Address,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct SpotToken {
    index: u64,
    #[serde(rename = "evmContract")]
    evm_contract: Option<EvmContract>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct SpotMeta {
    tokens: Vec<SpotToken>,
}

fn info_url(chain: Chain) -> &'static str {
    match chain {
        Chain::Mainnet => "https://api.hyperliquid.xyz/info",
        Chain::Testnet => "https://api.hyperliquid-testnet.xyz/info",
    }
}

async fn fetch_spot_meta(chain: Chain) -> Result<SpotMeta> {
    let url = info_url(chain);
    let client = reqwest::Client::new();
    let response = client.post(url).json(&serde_json::json!({"type": "spotMeta"})).send().await?;
    Ok(response.json().await?)
}

pub async fn erc20_contract_to_system_address(chain: Chain) -> Result<BTreeMap<Address, Address>> {
    let meta = fetch_spot_meta(chain).await?;
    let mut map = BTreeMap::new();
    for token in &meta.tokens {
        if let Some(evm_contract) = &token.evm_contract {
            let mut addr = [0u8; 20];
            addr[0] = 0x20;
            addr[12..20].copy_from_slice(token.index.to_be_bytes().as_ref());
            let addr = Address::from_slice(&addr);

            map.insert(evm_contract.address, addr);
        }
    }
    Ok(map)
}
