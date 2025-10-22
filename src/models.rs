use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
pub struct Version {
    pub meta: bool,
    pub version: String,
}

#[derive(Deserialize, Debug)]
pub struct Config {
    pub mode: String,
}

#[derive(Deserialize, Debug)]
pub struct ProxiesResponse {
    pub proxies: HashMap<String, ProxyInfo>,
}

#[derive(Deserialize, Debug)]
pub struct ProxyInfo {
    #[serde(rename = "type")]
    pub proxy_type: String,
    #[serde(default)]
    pub all: Vec<String>,
    #[serde(default)]
    pub now: String,
    #[serde(default)]
    pub history: Vec<HistoryItem>,
}

#[derive(Deserialize, Debug)]
pub struct HistoryItem {
    pub time: String,
    pub delay: u32,
}

#[derive(Serialize)]
pub struct SwitchRequest {
    pub name: String,
}

#[derive(Serialize)]
pub struct ModeRequest {
    pub mode: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version_deserialize() {
        let json = r#"{"meta":true,"version":"1.18.0"}"#;
        let version: Version = serde_json::from_str(json).unwrap();
        assert_eq!(version.version, "1.18.0");
        assert!(version.meta);
    }

    #[test]
    fn test_proxy_info_deserialize() {
        let json = r#"{
            "type": "Selector",
            "all": ["DIRECT", "REJECT"],
            "now": "DIRECT"
        }"#;
        let info: ProxyInfo = serde_json::from_str(json).unwrap();
        assert_eq!(info.proxy_type, "Selector");
        assert_eq!(info.all.len(), 2);
        assert_eq!(info.now, "DIRECT");
    }

    #[test]
    fn test_proxy_info_defaults() {
        let json = r#"{"type": "Direct"}"#;
        let info: ProxyInfo = serde_json::from_str(json).unwrap();
        assert_eq!(info.proxy_type, "Direct");
        assert!(info.all.is_empty());
        assert!(info.now.is_empty());
        assert!(info.history.is_empty());
    }

    #[test]
    fn test_config_deserialize() {
        let json = r#"{
            "port": 7890,
            "socks-port": 7891,
            "redir-port": 0,
            "allow-lan": true,
            "mode": "Rule",
            "log-level": "info"
        }"#;
        let config: Config = serde_json::from_str(json).unwrap();
        assert_eq!(config.mode, "Rule");
    }

    #[test]
    fn test_switch_request_serialize() {
        let req = SwitchRequest {
            name: "DIRECT".to_string(),
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("DIRECT"));
        assert!(json.contains("name"));
    }

    #[test]
    fn test_mode_request_serialize() {
        let req = ModeRequest {
            mode: "Global".to_string(),
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("Global"));
        assert!(json.contains("mode"));
    }

    #[test]
    fn test_history_item_deserialize() {
        let json = r#"{
            "time": "2024-01-01T00:00:00Z",
            "delay": 150
        }"#;
        let item: HistoryItem = serde_json::from_str(json).unwrap();
        assert_eq!(item.time, "2024-01-01T00:00:00Z");
        assert_eq!(item.delay, 150);
    }

    #[test]
    fn test_proxies_response_deserialize() {
        let json = r#"{
            "proxies": {
                "DIRECT": {
                    "type": "Direct"
                },
                "GLOBAL": {
                    "type": "Selector",
                    "all": ["DIRECT"],
                    "now": "DIRECT"
                }
            }
        }"#;
        let response: ProxiesResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.proxies.len(), 2);
        assert!(response.proxies.contains_key("DIRECT"));
        assert!(response.proxies.contains_key("GLOBAL"));
    }
}
