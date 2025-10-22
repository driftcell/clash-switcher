use crate::models::*;
use anyhow::{Context, ensure};
use reqwest::blocking::Client;
use reqwest::header::{AUTHORIZATION, HeaderMap, HeaderValue};

pub struct ClashClient {
    base_url: String,
    client: Client,
}

impl ClashClient {
    pub fn new(base_url: String, secret: Option<String>) -> Self {
        let mut headers = HeaderMap::new();

        if let Some(secret) = secret {
            if !secret.is_empty() {
                if let Ok(value) = HeaderValue::from_str(&format!("Bearer {}", secret)) {
                    headers.insert(AUTHORIZATION, value);
                }
            }
        }

        let client = Client::builder()
            .default_headers(headers)
            .build()
            .expect("Failed to build HTTP client");

        Self { base_url, client }
    }

    pub fn version(&self) -> anyhow::Result<Version> {
        ensure!(!self.base_url.is_empty());
        self.client
            .get(format!("{}/version", self.base_url))
            .send()
            .context("Failed to get the clash version due to the request")?
            .json::<Version>()
            .context("Failed to parse clash version")
    }

    pub fn get_proxies(&self) -> anyhow::Result<ProxiesResponse> {
        ensure!(!self.base_url.is_empty(), "Base URL is empty");
        self.client
            .get(format!("{}/proxies", self.base_url))
            .send()
            .context("Failed to get proxies")?
            .json::<ProxiesResponse>()
            .context("Failed to parse proxies response")
    }

    pub fn get_proxy(&self, name: &str) -> anyhow::Result<ProxyInfo> {
        ensure!(!self.base_url.is_empty(), "Base URL is empty");
        ensure!(!name.is_empty(), "Proxy name is empty");
        self.client
            .get(format!("{}/proxies/{}", self.base_url, name))
            .send()
            .context("Failed to get proxy info")?
            .json::<ProxyInfo>()
            .context("Failed to parse proxy info")
    }

    pub fn switch_proxy(&self, group: &str, proxy: &str) -> anyhow::Result<()> {
        ensure!(!self.base_url.is_empty(), "Base URL is empty");
        ensure!(!group.is_empty(), "Proxy group name is empty");
        ensure!(!proxy.is_empty(), "Proxy name is empty");

        let response = self
            .client
            .put(format!("{}/proxies/{}", self.base_url, group))
            .json(&SwitchRequest {
                name: proxy.to_string(),
            })
            .send()
            .context("Failed to switch proxy")?;

        if response.status().is_success() {
            Ok(())
        } else {
            anyhow::bail!(
                "Failed to switch proxy: {} - {}",
                response.status(),
                response.text().unwrap_or_default()
            )
        }
    }

    pub fn get_config(&self) -> anyhow::Result<Config> {
        ensure!(!self.base_url.is_empty(), "Base URL is empty");
        self.client
            .get(format!("{}/configs", self.base_url))
            .send()
            .context("Failed to get config")?
            .json::<Config>()
            .context("Failed to parse config")
    }

    pub fn set_mode(&self, mode: &str) -> anyhow::Result<()> {
        ensure!(!self.base_url.is_empty(), "Base URL is empty");
        ensure!(!mode.is_empty(), "Mode is empty");

        let response = self
            .client
            .patch(format!("{}/configs", self.base_url))
            .json(&ModeRequest {
                mode: mode.to_string(),
            })
            .send()
            .context("Failed to set mode")?;

        if response.status().is_success() {
            Ok(())
        } else {
            anyhow::bail!(
                "Failed to set mode: {} - {}",
                response.status(),
                response.text().unwrap_or_default()
            )
        }
    }
}
