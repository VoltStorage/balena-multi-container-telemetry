use crate::collectors::balena_stats_collector::BalenaStatsCollector;
use crate::collectors::raw_stats_to_json_str::stdout_lines_to_json_array;
use crate::domain::ContainerStats;
use crate::parsers::balena_stats_json_parsers::parse;
use crate::COLLECTOR_CONFIG;
use anyhow::anyhow;
use std::process::Command;

pub struct BalenaStatsCliCollector;

impl BalenaStatsCollector for BalenaStatsCliCollector {
    fn collect(&self) -> anyhow::Result<Vec<ContainerStats>> {
        let output_as_json_array = collect_raw_from_cli()?;
        Ok(parse(&output_as_json_array))
    }
}

fn collect_raw_from_cli() -> anyhow::Result<String> {
    let output = Command::new(COLLECTOR_CONFIG.clone().cli_path)
        .arg("stats")
        .arg("--no-stream")
        .arg("--format")
        .arg("\"{{json .}}\"")
        .output()?;

    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        stdout_lines_to_json_array(stdout.trim())
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        Err(anyhow!("{}", stderr))
    }
}
