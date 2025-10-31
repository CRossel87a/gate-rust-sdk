use anyhow::anyhow;
use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ContractInfo {
    pub name: String,
    pub mark_price: String,
    pub funding_rate_indicative: String,
    pub funding_offset: u64,
    pub in_delisting: bool,
    pub funding_interval: u64,
    pub funding_impact_value: String,
    pub funding_rate: String,
}

impl ContractInfo {
    pub fn annualized_funding_rate(&self) -> Result<f64> {
        let hours_per_interval: f64 = self.funding_interval as f64 / 3600.0;
        let intervals_per_day: f64 = 24.0 / hours_per_interval;
        let intervals_per_year: f64 = intervals_per_day * 365.0;
        let funding_rate: f64 = self.funding_rate.parse()?;
        Ok(funding_rate * intervals_per_year)
    }
}

impl GateSDK {

    /// https://www.gate.com/docs/developers/apiv4/#query-all-futures-contracts
    pub async fn future_contracts(&self, settle_coin: &str) -> Result<Vec<ContractInfo>> {

        let url = format!("{}/futures/{}/contracts", LIVE_URL, settle_coin.to_lowercase());
        let resp = self.client.get(url).send().await?.error_for_status()?;
        let text = resp.text().await?;

        serde_json::from_str::<Vec<ContractInfo>>(&text)
            .map_err(|err| {
                anyhow!("Err: {} text: {}", err, text)
            })
    }
}