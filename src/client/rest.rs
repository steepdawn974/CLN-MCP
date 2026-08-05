use anyhow::{anyhow, Result};
use async_trait::async_trait;
use reqwest::Client;
use serde_json::Value;
use std::path::Path;
use std::time::Duration;
use tracing::debug;

use super::backend::ClnBackend;

pub struct RestBackend {
    client: Client,
    base_url: String,
    rune: String,
}

impl RestBackend {
    pub async fn new(url: &str, rune: &str, ca_cert_path: Option<&str>) -> Result<Self> {
        // Generous timeout: some RPC calls (pay, fundchannel, close) are long-running
        let mut builder = Client::builder().timeout(Duration::from_secs(120));

        if let Some(ca_path) = ca_cert_path {
            if !Path::new(ca_path).exists() {
                return Err(anyhow!("CA cert file not found: {}", ca_path));
            }
            let pem = tokio::fs::read(ca_path).await?;
            let cert = reqwest::Certificate::from_pem(&pem)?;
            builder = builder.add_root_certificate(cert);
            debug!("Loaded CA cert from {}", ca_path);
        } else {
            // clnrest uses self-signed certs by default; only skip verification
            // when the user has not provided a CA cert
            builder = builder.danger_accept_invalid_certs(true);
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
            return Err(anyhow!(
                "REST call {} failed ({}): {}",
                method,
                status,
                body
            ));
        }

        let value: Value = serde_json::from_str(&body).map_err(|e| {
            anyhow!(
                "Failed to parse REST response for {}: {} — body: {}",
                method,
                e,
                body
            )
        })?;

        Ok(value)
    }
}
