use anyhow::{anyhow, Result};
use async_trait::async_trait;
use reqwest::Client;
use serde_json::Value;
use std::path::Path;
use tracing::debug;

use super::backend::ClnBackend;

pub struct RestBackend {
    client: Client,
    base_url: String,
    rune: String,
}

impl RestBackend {
    pub async fn new(url: &str, rune: &str, ca_cert_path: Option<&str>) -> Result<Self> {
        let mut builder = Client::builder()
            .danger_accept_invalid_certs(true);

        if let Some(ca_path) = ca_cert_path {
            if Path::new(ca_path).exists() {
                let pem = std::fs::read(ca_path)?;
                let cert = reqwest::Certificate::from_pem(&pem)?;
                builder = builder.add_root_certificate(cert);
                debug!("Loaded CA cert from {}", ca_path);
            }
        }

        let client = builder.build()?;

        Ok(Self {
            client,
            base_url: url.trim_end_matches('/').to_string(),
            rune: rune.to_string(),
        })
    }
}

#[async_trait]
impl ClnBackend for RestBackend {
    async fn call(&self, method: &str, params: Value) -> Result<Value> {
        let url = format!("{}/v1/{}", self.base_url, method);
        debug!("REST call: {} with params: {}", url, params);

        let resp = self
            .client
            .post(&url)
            .header("Rune", &self.rune)
            .json(&params)
            .send()
            .await?;

        let status = resp.status();
        let body = resp.text().await?;

        if !status.is_success() {
            return Err(anyhow!("REST call {} failed ({}): {}", method, status, body));
        }

        let value: Value = serde_json::from_str(&body)
            .map_err(|e| anyhow!("Failed to parse REST response for {}: {} — body: {}", method, e, body))?;

        Ok(value)
    }
}
