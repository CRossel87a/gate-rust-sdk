use std::collections::HashMap;

use gate_rust_sdk::GateSDK;
use anyhow::{Result, ensure};

use std::env;

fn unlock_keys() -> anyhow::Result<(String, String)>{
    let key: String = env::var("gate_key")?;
    let secret: String = env::var("gate_secret")?;
    Ok((key, secret))
}

#[tokio::test]
async fn test_futures_info() -> Result<()>{
    let sdk = GateSDK::new_readonly();
    let cis = sdk.future_contracts("usdt").await?;
    let json = serde_json::to_string_pretty(&cis)?;
    std::fs::write("temp/futures_info.json", &json)?;

    let mut funding_rates: HashMap<String, f64> = HashMap::default();

    for c in cis {
        let funding_rate = c.annualized_funding_rate()?;
        funding_rates.insert(c.name, funding_rate);
    }
    ensure!(!funding_rates.is_empty());

    let json = serde_json::to_string_pretty(&funding_rates)?;
    std::fs::write("temp/futures_funding_rates.json", &json)?;

    Ok(())
}

#[tokio::test]
async fn test_futures_account() -> Result<()>{
    let (key, secret) = unlock_keys()?;
    let sdk = GateSDK::new(key, secret);
    let account = sdk.future_account("usdt").await?;
    dbg!(account);
    Ok(())
}