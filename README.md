# Balena Multi-Container Telemetry

This program provides insights into CPU, memory, network, and I/O metrics for all containers running in a Balena
multi-container/service setup.

It collects metrics from CLI commands (`balena stats`) output or a file and exports the parsed metrics to MQTT topics.

## Deployment, Configuration, and Execution

Two modes are supported for collecting metrics:

- `CLI`: Intended for direct deployment on the Balena Host OS.
- `FILE`: Can be used either on the Balena Host OS or inside a container. For an example how to run it as a container,
  refer to `docker-compose.yaml`.

For configuration details of collectors, parsers, and exporters, see the following sections.

### Collectors

Configuration is done in `config/balena_stats_collector.config.json`; see `balena_stats_collector.example-config.json`
as an example:

- `collection_interval_in_seconds`: Defines the interval in seconds at which the collection is started.
- `mode`: `CLI` or `FILE`; for details, see below.

#### CLI

Internally, this application executes the CLI command `balena stats` (or `docker stats`; see `cli_path` in
`balena_stats_collector.config.json`). To avoid forwarding the Balena/Docker socket into a container, the only supported
mode is running directly on the Balena Host OS.

#### FILE

In this mode, the application collects stats from a file at the path defined via `file_path` in
`balena_stats_collector.config.json`.

When running inside a container, the basic idea is to provide access to the referenced file via a mount into the
container.

To update the contents of the file regularly with the output from `balena stats`, for example, a cron job on the Balena
Host OS could be executed. The file format needs to match the output from
`balena stats --no-stream --format {{json .}}`.

### Exporters

The MQTT exporter is configured via `config/mqtt.config.json` (see `mqtt.example-config.json` as a reference).

- `broker_url`: e.g., `tcp://localhost:1883`; only unauthenticated connections are supported at the moment.
- `device_id`: Used as a fallback value if `BALENA_DEVICE_UUID` is not available.
- `root_topic_template`: `root/{device_id}/telemetry/system/{service_name}`; defines the root topic onto which the
  metrics are published. `service_name` will be extracted from the `NAME` column of `balena stats`. This root topic will
  be extended by metric names like `cpu_usage_in_percent` or `memory_usage_in_percent`.

## Contributing and Building

To build the application binary for Balena-supported devices, an example setup for `aarch64` is defined in the
`Dockerfile` and `docker-compose.yaml` within `cross-compile`.

### Container Image Build
To recreate the application binary and build a docker container image from it, use the `Dockerfile` in the root directory.

Here is an example of how to build for `arm64`, which is used on Balena:

```sh
docker buildx build --platform linux/arm64 -t balena-multi-container-telemetry:arm64 .
```