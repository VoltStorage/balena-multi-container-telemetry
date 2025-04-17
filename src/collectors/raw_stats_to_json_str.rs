pub fn stdout_lines_to_json_array(stdout: &str) -> anyhow::Result<String> {
    let lines = stdout.split('\n').collect::<Vec<&str>>();
    let trimmed: Vec<&str> = lines
        .into_iter()
        .filter(|line| line.trim().len() > 0)
        .collect();
    let joined = trimmed.join(",\n");
    Ok(format!("[{}]", joined))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_convert_stdout_lines_to_json_array() {
        let stdout = include_str!("../../test-data/balena_stats_stdout.txt");
        let expected = include_str!("../../test-data/balena_stats_stdout.json");

        let actual = stdout_lines_to_json_array(stdout);

        assert_eq!(actual.unwrap(), expected)
    }
}