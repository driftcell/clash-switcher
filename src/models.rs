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
