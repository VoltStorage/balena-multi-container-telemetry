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
        let unquoted_lines = remove_line_quotes(stdout.trim())?;
        stdout_lines_to_json_array(&unquoted_lines)
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        Err(anyhow!("{}", stderr))
    }
}

pub fn remove_line_quotes(stdout: &str) -> anyhow::Result<String> {
    let lines = stdout.split('\n').collect::<Vec<&str>>();
    let trimmed: Vec<&str> = lines
        .iter()
        .filter(|line| line.trim().len() > 0)
        .map(|line| &line[1..line.len() - 1]) // remove outer quotes
        .collect();
    let joined = trimmed.join("\n");
    Ok(joined)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_remove_line_quotes() {
        let stdout = include_str!("../../test-data/balena_cli_stats_stdout_quote_lines.txt");
        let expected = include_str!("../../test-data/balena_stats_stdout.txt");

        let actual = remove_line_quotes(stdout);

        assert_eq!(actual.unwrap(), expected)
    }
}