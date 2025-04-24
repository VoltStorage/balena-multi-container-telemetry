use crate::util::config::{build_path, get_config, CONFIG_DIR};
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

pub fn get_collector_config() -> BalenaStatsCollectorConfig {
    get_config(build_path(vec![&CONFIG_DIR, "balena_stats_collector.config.json"]))
}