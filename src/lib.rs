pub mod futures;

use reqwest::Client;
use anyhow::Result;

// https://www.gate.com/docs/developers/apiv4/#gate-api-v4-105-20

pub const LIVE_URL: &str = "https://api.gateio.ws/api/v4";


pub struct GateSDK {
    client: reqwest::Client,
    api_key: Option<String>,
    api_secret: Option<String>,
}


impl GateSDK {
    pub fn new(api_key: String, api_secret: String) -> Self {
        Self { 
            client: Client::new(),
            api_key: Some(api_key),
            api_secret: Some(api_secret),
         }
    }

    pub fn new_readonly() -> Self {
        Self { 
            client: Client::new(),
            api_key: None,
            api_secret: None,
         }
    }
}