use std::collections::HashMap;

use gate_rust_sdk::GateSDK;
use anyhow::{Result, ensure};

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