use crate::domain::ContainerStats;
use crate::util::config::{build_path, get_config, CONFIG_DIR};
use anyhow::anyhow;
use lazy_static::lazy_static;
use log::error;
use paho_mqtt as mqtt;
use paho_mqtt::Client;
use serde::Deserialize;
use std::string::ToString;
use std::time::Duration;

#[derive(Clone, Debug, PartialEq)]
struct MqttMessage {
    topic: String,
    value: f32,
}

#[derive(Clone, Deserialize, Debug, PartialEq)]
struct MqttConfig {
    broker_url: String,
    root_topic_template: String,
    device_id: String,
    unit: String,
}

lazy_static! {
    static ref CONFIG: MqttConfig = get_config(build_path(vec![&CONFIG_DIR, "mqtt.config.json"]));
}

lazy_static! {
    static ref CLIENT: Client = build_client_and_connect(CONFIG.clone());
}

pub fn export(stats: Vec<ContainerStats>) {
    map_to_mqtt_messages(stats)
        .into_iter()
        .for_each(|message| publish(message));
}

fn publish(message: MqttMessage) {
    let payload: String = "{\"value\": ".to_string() + &message.value.to_string() + "}";
    let msg = mqtt::Message::new(message.topic, payload, 0);
    CLIENT
        .publish(msg.clone())
        .unwrap_or_else(|err| error!("Publishing of msg {} failed! Because of {}", msg, err))
}

fn map_to_mqtt_messages(stats: Vec<ContainerStats>) -> Vec<MqttMessage> {
    stats
        .iter()
        .flat_map(|stat| map_to_mqtt_message(stat, &CONFIG))
        .collect()
}

fn map_to_mqtt_message(stats: &ContainerStats, config: &MqttConfig) -> Vec<MqttMessage> {
    let base_topic = &config.root_topic_template
        .replace("{device_id}", &config.device_id)
        .replace("{unit}", &config.unit)
        .replace("{service_name}", &stats.service_name);

    [
        build_messsage(
            &(base_topic.clone() + "/memory_usage_in_percent"),
            stats.mem_usage_in_percent,
        ),
        build_messsage(
            &(base_topic.clone() + "/cpu_usage_in_percent"),
            stats.cpu_usage_in_percent,
        ),
    ]
        .into_iter()
        .filter_map(|result| result.ok())
        .collect()
}

fn build_messsage(topic: &str, value: Option<f32>) -> anyhow::Result<MqttMessage> {
    value
        .ok_or(anyhow!("Value not available"))
        .map(|val| MqttMessage {
            topic: String::from(topic),
            value: val,
        })
}

fn build_client_and_connect(config: MqttConfig) -> Client {
    let client_options = mqtt::CreateOptionsBuilder::new()
        .server_uri(config.broker_url)
        .finalize();
    let client = Client::new(client_options).expect("Error during client creation");
    let connection_options = mqtt::ConnectOptionsBuilder::new()
        .clean_session(true)
        .automatic_reconnect(Duration::from_secs(1), Duration::from_secs(30))
        .finalize();

    client
        .connect(connection_options)
        .expect("Failed to connect to broker");

    client
}

#[cfg(test)]
mod tests {
    use super::*;
    use byte_unit::{Byte, Unit};

    #[test]
    fn should_map_to_mqtt_message() {
        let input = ContainerStats {
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
        let expected = [
            MqttMessage {
                topic: "root/d35a7ea843c61c723a12f19a41c26ef1/telemetry/my-unit/b/memory_usage_in_percent"
                    .to_string(),
                value: input.mem_usage_in_percent.unwrap(),
            },
            MqttMessage {
                topic: "root/d35a7ea843c61c723a12f19a41c26ef1/telemetry/my-unit/b/cpu_usage_in_percent"
                    .to_string(),
                value: input.cpu_usage_in_percent.unwrap(),
            },
        ]
            .to_vec();
        let config: MqttConfig = get_config(build_path(vec!["test-data/config/mqtt.config.json"]));

        let actual = map_to_mqtt_message(&input, &config);

        assert_eq!(actual, expected)
    }

    #[test]
    fn should_get_config() {
        let test_config_path = build_path(vec!["test-data/config/mqtt.config.json"]);

        let actual: MqttConfig = get_config(test_config_path);

        assert_eq!(actual.broker_url, "tcp://localhost:1883");
        assert_eq!(actual.device_id, "d35a7ea843c61c723a12f19a41c26ef1");
        assert_eq!(actual.unit, "my-unit");
        assert_eq!(
            actual.root_topic_template,
            "root/{device_id}/telemetry/{unit}/{service_name}"
        );
    }

    #[test]
    #[ignore] // Manual test to running MQTT broker
    fn should_build_a_client_and_connect() {
        assert_eq!(CLIENT.is_connected(), true);

        let test_message = MqttMessage {
            topic:
            "root/d35a7ea843c61c723a12f19a41c26ef1/telemetry/system/b/memory/usage_in_percent"
                .to_string(),
            value: 12.57,
        };
        publish(test_message)
    }
}
