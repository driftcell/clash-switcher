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

        if let Some(secret) = secret
            && !secret.is_empty()
            && let Ok(value) = HeaderValue::from_str(&format!("Bearer {}", secret))
        {
            headers.insert(AUTHORIZATION, value);
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_creation_without_secret() {
        let client = ClashClient::new("http://localhost:9090".to_string(), None);
        assert_eq!(client.base_url, "http://localhost:9090");
    }

    #[test]
    fn test_client_creation_with_secret() {
        let client = ClashClient::new(
            "http://localhost:9090".to_string(),
            Some("test-secret".to_string()),
        );
        assert_eq!(client.base_url, "http://localhost:9090");
    }

    #[test]
    fn test_client_creation_with_empty_secret() {
        let client = ClashClient::new("http://localhost:9090".to_string(), Some("".to_string()));
        assert_eq!(client.base_url, "http://localhost:9090");
    }

    #[test]
    fn test_client_with_custom_url() {
        let client = ClashClient::new("http://example.com:9090".to_string(), None);
        assert_eq!(client.base_url, "http://example.com:9090");
    }

    #[test]
    fn test_version_with_empty_url() {
        let client = ClashClient::new("".to_string(), None);
        let result = client.version();
        assert!(result.is_err());
    }

    #[test]
    fn test_get_proxies_with_empty_url() {
        let client = ClashClient::new("".to_string(), None);
        let result = client.get_proxies();
        assert!(result.is_err());
    }

    #[test]
    fn test_get_proxy_with_empty_name() {
        let client = ClashClient::new("http://localhost:9090".to_string(), None);
        let result = client.get_proxy("");
        assert!(result.is_err());
    }

    #[test]
    fn test_switch_proxy_with_empty_group() {
        let client = ClashClient::new("http://localhost:9090".to_string(), None);
        let result = client.switch_proxy("", "proxy");
        assert!(result.is_err());
    }

    #[test]
    fn test_switch_proxy_with_empty_proxy() {
        let client = ClashClient::new("http://localhost:9090".to_string(), None);
        let result = client.switch_proxy("group", "");
        assert!(result.is_err());
    }

    #[test]
    fn test_set_mode_with_empty_mode() {
        let client = ClashClient::new("http://localhost:9090".to_string(), None);
        let result = client.set_mode("");
        assert!(result.is_err());
    }
}
