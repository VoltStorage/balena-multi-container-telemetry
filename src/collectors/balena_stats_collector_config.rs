use crate::util::config::get_config;
use serde;
use serde::Deserialize;

#[derive(PartialEq, Deserialize, Clone)]
pub enum CollectorType {
    CLI,
    FILE,
}

#[derive(Deserialize, Clone)]
pub struct BalenaStatsCollectorConfig {
    pub mode: CollectorType,
    pub cli_path: String,
    pub file_path: String,
    pub collection_interval_in_seconds: u64,
}

const DEFAULT_CONFIG_PATH: &str = "config/balena_stats_collector.config.json";

pub fn get_collector_config() -> BalenaStatsCollectorConfig {
    get_config(DEFAULT_CONFIG_PATH.to_string())
}