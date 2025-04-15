use crate::collectors::balena_stats_collector::BalenaStatsCollector;
use crate::collectors::raw_stats_to_json_str::stdout_lines_to_json_array;
use crate::domain::ContainerStats;
use crate::parsers::balena_stats_json_parsers::parse;
use crate::COLLECTOR_CONFIG;
use std::fs;

pub struct BalenaStatsFileCollector;

impl BalenaStatsCollector for BalenaStatsFileCollector {
    fn collect(&self) -> anyhow::Result<Vec<ContainerStats>> {
        let output_as_json_array = collect_from_file()?;
        Ok(parse(&output_as_json_array))
    }
}

fn collect_from_file() -> anyhow::Result<String> {
    let contents = fs::read_to_string(COLLECTOR_CONFIG.clone().file_path)?;
    stdout_lines_to_json_array(contents.trim())
}
