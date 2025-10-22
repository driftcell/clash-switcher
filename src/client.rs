use crate::models::*;
use anyhow::{Context, ensure};

pub struct ClashClient {
    base_url: String,
}

impl ClashClient {
    pub fn new(base_url: String) -> Self {
        Self { base_url }
    }

    pub fn version(&self) -> anyhow::Result<Version> {
        ensure!(!self.base_url.is_empty());
        reqwest::blocking::get(format!("{}/version", self.base_url))
            .context("Failed to get the clash version due to the request")?
            .json::<Version>()
            .context("Failed to parse clash version")
    }

    pub fn get_proxies(&self) -> anyhow::Result<ProxiesResponse> {
        ensure!(!self.base_url.is_empty(), "Base URL is empty");
        reqwest::blocking::get(format!("{}/proxies", self.base_url))
            .context("Failed to get proxies")?
            .json::<ProxiesResponse>()
            .context("Failed to parse proxies response")
    }

    pub fn get_proxy(&self, name: &str) -> anyhow::Result<ProxyInfo> {
        ensure!(!self.base_url.is_empty(), "Base URL is empty");
        ensure!(!name.is_empty(), "Proxy name is empty");
        reqwest::blocking::get(format!("{}/proxies/{}", self.base_url, name))
            .context("Failed to get proxy info")?
            .json::<ProxyInfo>()
            .context("Failed to parse proxy info")
    }

    pub fn switch_proxy(&self, group: &str, proxy: &str) -> anyhow::Result<()> {
        ensure!(!self.base_url.is_empty(), "Base URL is empty");
        ensure!(!group.is_empty(), "Proxy group name is empty");
        ensure!(!proxy.is_empty(), "Proxy name is empty");

        let client = reqwest::blocking::Client::new();
        let response = client
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
        reqwest::blocking::get(format!("{}/configs", self.base_url))
            .context("Failed to get config")?
            .json::<Config>()
            .context("Failed to parse config")
    }

    pub fn set_mode(&self, mode: &str) -> anyhow::Result<()> {
        ensure!(!self.base_url.is_empty(), "Base URL is empty");
        ensure!(!mode.is_empty(), "Mode is empty");

        let client = reqwest::blocking::Client::new();
        let response = client
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
