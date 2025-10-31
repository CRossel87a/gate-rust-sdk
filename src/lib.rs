pub mod futures;

use reqwest::Client;
use anyhow::Result;
use reqwest::header::{HeaderMap, HeaderValue};
use hmac::{Hmac, Mac};
use sha2::{Sha512, Digest};
use anyhow::anyhow;
use std::time::{SystemTime, UNIX_EPOCH};

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

    /// https://github.com/gateio/gateapi-python/blob/master/gate_api/api_client.py
    pub fn sign_request(&self, request: String) -> anyhow::Result<String> {
        let secret_key = self.api_secret.as_ref().ok_or_else(|| anyhow!("Missing secret key"))?;
        let mut signed_key = Hmac::<Sha512>::new_from_slice(secret_key.as_bytes())?;
        signed_key.update(request.as_bytes());
        // Convert HMAC result to hex string, then Base64-encode the hex string bytes
        let hexdigest = hex::encode(signed_key.finalize().into_bytes());
        Ok(hexdigest)
    }

    pub async fn get_request(&self, endpoint: &str) -> anyhow::Result<reqwest::Response> {
        let api_key = self.api_key.as_ref().ok_or_else(|| anyhow!("Missing api key"))?;

        let timestamp = get_timestamp();

        let mut headers = HeaderMap::new();
        
        headers.insert("KEY", HeaderValue::from_str(&api_key)?);
        headers.insert("Timestamp", HeaderValue::from_str(&timestamp)?);
        headers.insert("Content-Type", HeaderValue::from_static("application/json"));

        let url = format!("{LIVE_URL}{endpoint}");
        
        // Hash the payload (empty for GET requests)
        let mut hasher = Sha512::new();
        hasher.update(b"");
        let hashed_payload = hex::encode(hasher.finalize());
        
        // The URL in the signature must include the full path with /api/v4
        let full_path = format!("/api/v4{}", endpoint);
        
        // Format: {method}\n{url}\n{query_string}\n{hashed_payload}\n{timestamp}
        let prehash = format!("GET\n{}\n\n{}\n{}", full_path, hashed_payload, timestamp);
  
        // Debug output
        //eprintln!("Endpoint: {}", endpoint);
        //eprintln!("Full path for signature: {}", full_path);
        //eprintln!("Timestamp: {}", timestamp);
        //eprintln!("Hashed payload: {}", hashed_payload);
        //eprintln!("Prehash string:\n{}", prehash);
        
        let hexdigest = self.sign_request(prehash)?;
        //eprintln!("Signature: {}", hexdigest);
        
        headers.insert("SIGN", HeaderValue::from_str(&hexdigest)?);

        let resp = self.client.get(url).headers(headers).send().await?;

        Ok(resp)
    }
}

pub fn get_timestamp() -> String {
    let duration = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    duration.as_secs().to_string()
}