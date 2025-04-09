use crate::domain::ContainerStats;
use anyhow::anyhow;
use byte_unit::{Byte, ParseError};
use log::warn;
use serde::Deserialize;
use std::fmt::Debug;
use std::str::FromStr;

#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "PascalCase")]
struct ContainerStatsAsStrings {
    #[serde(rename = "BlockIO")]
    block_io: String,
    #[serde(rename = "CPUPerc")]
    cpu_perc: String,
    container: String,
    #[serde(rename = "ID")]
    id: String,
    mem_perc: String,
    mem_usage: String,
    name: String,
    #[serde(rename = "NetIO")]
    net_io: String,
    #[serde(rename = "PIDs")]
    pids: String,
}

pub fn parse(json_str: &str) -> Vec<ContainerStats> {
    let parsed = parse_raw(json_str).expect(&format!("Cannot parse raw json: {}", json_str));
    let mapped: Vec<ContainerStats> = parsed
        .into_iter()
        .map(|string_stats| map(string_stats))
        .filter_map(|parsed| parsed.ok())
        .collect();

    mapped
}

fn parse_raw(json_str: &str) -> Result<Vec<ContainerStatsAsStrings>, serde_json::Error> {
    let stats: Result<Vec<ContainerStatsAsStrings>, serde_json::Error> =
        serde_json::from_str(json_str);
    stats
}

fn map(stat_strings: ContainerStatsAsStrings) -> anyhow::Result<ContainerStats> {
    let [network_input_in_bytes, network_output_in_bytes] =
        parse_bytes_from_two_values(&stat_strings.net_io);
    let [mem_usage_in_bytes, mem_limit_in_bytes] =
        parse_bytes_from_two_values(&stat_strings.mem_usage);
    let [block_device_input_in_bytes, block_device_output_in_bytes] =
        parse_bytes_from_two_values(&stat_strings.block_io);

    let stats: ContainerStats = ContainerStats {
        container_name: stat_strings.name.clone(),
        container_id: stat_strings.id,
        container_id_short: stat_strings.container,
        service_name: parse_service_name(&stat_strings.name),
        amount_of_pids: parse_or_log(&stat_strings.pids),
        cpu_usage_in_percent: parse_or_log(&stat_strings.cpu_perc.replace("%", "")),
        mem_usage_in_percent: parse_or_log(&stat_strings.mem_perc.replace("%", "")),
        network_input: network_input_in_bytes.ok(),
        network_output: network_output_in_bytes.ok(),
        mem_usage: mem_usage_in_bytes.ok(),
        mem_limit: mem_limit_in_bytes.ok(),
        block_device_input: block_device_input_in_bytes.ok(),
        block_device_output: block_device_output_in_bytes.ok(),
    };

    Ok(stats)
}

fn parse_or_log<T: FromStr>(input: &str) -> Option<T>
where
    <T as FromStr>::Err: std::fmt::Display,
{
    input
        .parse::<T>()
        .map_err(|err| {
            warn!(
                "Could parse from: {original}, err: {err}",
                original = input,
                err = err
            )
        })
        .ok()
}

// see https://doc.rust-lang.org/book/ch04-03-slices.html#string-slices-as-parameters
fn parse_bytes_from_two_values(input: &str) -> [anyhow::Result<Byte>; 2] {
    let parsed_bytes = input
        .split("/")
        .map(|string| (string, parse_byte_from_str(string)))
        .map(|(original_string, parse_result)| match parse_result {
            Ok(byte) => Ok(byte),
            Err(err) => {
                warn!(
                    "Could not parse {} into Byte. Error: {:?}",
                    original_string, err
                );
                Err(err)
            }
        })
        .collect::<Vec<anyhow::Result<Byte, ParseError>>>();

    match parsed_bytes.as_slice() {
        [first, second] => [
            first.clone().map_err(|e| anyhow::Error::new(e)),
            second.clone().map_err(|e| anyhow::Error::new(e)),
        ],
        _ => {
            let error_msg = format!("Not exactly two Bytes found in string: {}", input);
            [Err(anyhow!(error_msg.clone())), Err(anyhow!(error_msg))]
        }
    }
}

fn parse_byte_from_str(input: &str) -> Result<Byte, ParseError> {
    Byte::parse_str(input.trim(), true)
}

fn parse_service_name(container_name: &str) -> String {
    let mut parts: Vec<&str> = container_name.split("_").collect();
    if parts.len() > 2 {
        parts.truncate(parts.len() - 3);
    }

    let service_name = parts.join("_");

    service_name
}

#[cfg(test)]
mod tests {
    use super::*;
    use byte_unit::Unit;
    use rstest::rstest;

    fn setup_test_data() -> ContainerStatsAsStrings {
        ContainerStatsAsStrings {
            block_io: String::from("0B / 0B"),
            cpu_perc: String::from("1.75%"),
            container: String::from("4889ab0711ac"),
            id: String::from("4889ab0711ac17500a5de1c3d50eb6bfbc7eb367d349797f9fe8dcb92fd9e914"),
            mem_perc: String::from("31.12%"),
            mem_usage: String::from("318.6MiB / 1GiB"),
            name: String::from("b_10800414_3361262_f54e4ffc136d1344ee98993b36b9deeb"),
            net_io: String::from("541MB / 680MB"),
            pids: String::from("26"),
        }
    }

    #[test]
    fn it_parses_from_json_array() {
        let test_json = include_str!("../../test-data/balena_stats_example.json");
        let first_expected = setup_test_data();

        let actual = parse_raw(test_json);

        let success_result = actual.ok().unwrap();
        assert_eq!(success_result.len(), 8);
        let first = success_result.first().unwrap();
        assert_eq!(first, &first_expected);
    }

    #[test]
    fn it_maps_from_string_to_stats() {
        let input = setup_test_data();
        let expected = ContainerStats {
            container_id: String::from(
                "4889ab0711ac17500a5de1c3d50eb6bfbc7eb367d349797f9fe8dcb92fd9e914",
            ),
            container_id_short: String::from("4889ab0711ac"),
            container_name: String::from("b_10800414_3361262_f54e4ffc136d1344ee98993b36b9deeb"),
            service_name: "b".to_string(),
            cpu_usage_in_percent: Some(1.75),
            mem_usage_in_percent: Some(31.12),
            mem_usage: Byte::from_f64_with_unit(318.6, Unit::MiB),
            mem_limit: Byte::from_i64_with_unit(1, Unit::GiB),
            network_input: Byte::from_i64_with_unit(541, Unit::MB),
            network_output: Byte::from_i64_with_unit(680, Unit::MB),
            block_device_input: Byte::from_i64_with_unit(0, Unit::B),
            block_device_output: Byte::from_i64_with_unit(0, Unit::B),
            amount_of_pids: Some(26),
        };

        let actual = map(input);

        assert_eq!(actual.unwrap(), expected)
    }

    #[rstest]
    #[case("1e+03MB")]
    #[case("1e+03MB")]
    fn parse_byte_from_str_should_result_in_parse_error(#[case] input: &str) {
        let actual = parse_byte_from_str(input);

        assert!(
            matches!(actual.unwrap_err(), ParseError::Unit(_)),
            "Error is not of type Unit"
        );
    }

    #[rstest]
    #[case::zero_bytes("0B", Byte::from_i64_with_unit(0, Unit::B).unwrap())]
    #[case::mib_decimal_points("318.6MiB", Byte::from_f64_with_unit(318.6, Unit::MiB).unwrap())]
    #[case::gib_int("1GiB", Byte::from_i64_with_unit(1, Unit::GiB).unwrap())]
    #[case::mb_int("541MB", Byte::from_i64_with_unit(541, Unit::MB).unwrap())]
    fn parse_byte_from_str_should_result_in_ok(#[case] input: &str, #[case] expected: Byte) {
        let actual = parse_byte_from_str(input).unwrap();

        assert_eq!(actual, expected);
    }
}
