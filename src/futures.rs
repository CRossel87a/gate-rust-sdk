use anyhow::anyhow;
use serde::{Deserialize, Serialize};


use crate::*;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct FuturesContractInfo {
    pub name: String,
    pub mark_price: String,
    pub funding_rate_indicative: String,
    pub funding_offset: u64,
    pub in_delisting: bool,
    pub funding_interval: u64,
    pub funding_impact_value: String,
    pub funding_rate: String,
    pub quanto_multiplier: String
}


#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct FuturesAccount {
    pub currency: String,
    pub total: String,
    pub unrealised_pnl: String,
    pub cross_margin_balance: String
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct FuturesPosition {
    pub contract: String,
    pub size: i64,
    pub value: String,
    pub margin: String,
    pub entry_price: String,
    pub liq_price: String,
    pub mark_price: String
}

impl FuturesContractInfo {
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
    pub async fn futures_contracts(&self, settle_coin: &str) -> Result<Vec<FuturesContractInfo>> {

        let url = format!("{}/futures/{}/contracts", LIVE_URL, settle_coin.to_lowercase());
        let resp = self.client.get(url).send().await?.error_for_status()?;
        let text = resp.text().await?;

        serde_json::from_str::<Vec<FuturesContractInfo>>(&text)
            .map_err(|err| {
                anyhow!("Err: {} text: {}", err, text)
            })
    }

    /// https://www.gate.com/docs/developers/apiv4/en/#get-futures-account
    pub async fn futures_account(&self, settle_coin: &str) -> Result<FuturesAccount> {
        let endpoint = format!("/futures/{}/accounts", settle_coin.to_lowercase());
        
        let resp = self.get_request(&endpoint).await?;
        let status = resp.status();
        let text = resp.text().await?;

        if !status.is_success() {
            eprintln!("Response body: {}", text);
            return Err(anyhow!("HTTP status {} for url", status));
        }
        
        serde_json::from_str::<FuturesAccount>(&text)
            .map_err(|err| {
                anyhow!("Err: {} text: {}", err, text)
            })
    }

    /// https://www.gate.com/docs/developers/apiv4/en/#get-user-position-list
    pub async fn futures_positions(&self, settle_coin: &str) -> Result<Vec<FuturesPosition>> {
        let endpoint = format!("/futures/{}/positions", settle_coin.to_lowercase());
        
        let resp = self.get_request(&endpoint).await?;
        let status = resp.status();
        let text = resp.text().await?;
        
        if !status.is_success() {
            eprintln!("Response body: {}", text);
            return Err(anyhow!("HTTP status {} for url", status));
        }
        
        serde_json::from_str::<Vec<FuturesPosition>>(&text)
            .map_err(|err| {
                anyhow!("Err: {} text: {}", err, text)
            })
    }
}