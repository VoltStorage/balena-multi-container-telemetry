# Balena Multi-Container Telemetry

This program provides CPU, memory, network, and I/O metrics for all containers running in a Balena multi-container
setup. It collects metrics from CLI commands (`balena stats`) or a file and exports them to MQTT topics.

# Latest Docker Image

You can find the latest Docker image on Docker Hub:

[https://img.shields.io/badge/Docker%20Hub-Visit%20Repository-blue](https://hub.docker.com/r/voltstorage/balena-multi-container-telemetry)


## Deployment, Configuration, and Execution

Two modes for collecting metrics:

- `CLI`: For direct deployment on Balena Host OS or mounting the docker/balena socket.
- `FILE`: Can be used on Balena Host OS or inside a container. See `docker-compose.yaml` for an example.

For configuration details of collectors, parsers, and exporters, see the following sections.

### General Configuration

All configuration files for collectors, exporters, and logs should be in one `config` directory. By default, it points
to the relative "config/" directory but can be overridden via `CONFIG_DIR` env var.

### Collectors

Configuration is in `config/balena_stats_collector.config.json`; see `balena_stats_collector.example-config.json`:

- `collection_interval_in_seconds`: Interval in seconds for starting collection.
- `mode`: `CLI` or `FILE`; see below.

#### CLI

This application executes `balena stats` (or `docker stats`; see `cli_path` in `balena_stats_collector.config.json`).
Mount the Balena/Docker socket into the container or execute directly on Balena Host OS.

For setup via Balena `docker-compose.yaml` using the [label
`io.balena.features.balena-socket: 1`](https://docs.balena.io/reference/supervisor/docker-compose/#labels).

To manually mount the docker socket as a volume, use `-v /var/run/docker.sock:/var/run/docker.sock`.

#### FILE

In FILE mode, the application collects stats from a file defined by `file_path` in `balena_stats_collector.config.json`.

When running inside a container, mount the referenced file into the container.

To update file contents regularly with output from `balena stats`, execute a cron job on Balena Host OS. The file format
must match output from `balena stats --no-stream --format {{json .}}`.

### Exporters

Configure MQTT exporter via `config/mqtt.config.json` (see `mqtt.example-config.json`).

- `broker_url`: e.g., `tcp://localhost:1883`; only unauthenticated connections supported currently.
- `device_id`: Identifier for a device
- `unit`: Identifier of a unit
- `root_topic_template`: Defines root topic for publishing metrics. Default:
  `root/{device_id}/telemetry/system/{service_name}`. Extracts `service_name` from `NAME` column of
  `balena stats`. Extends root topic with metric names like `cpu_usage_in_percent` or `memory_usage_in_percent`.

## Contributing and Building

To build application binary for Balena-supported devices, see example setup for `aarch64` within cross-compile; usage:
`docker-compose -f cross-compile/docker-compose.yaml up --build`

### Container Image Build

To recreate application binary and build docker container image, use Dockerfile in root directory.

Example build for `arm64` used on Balena:

```sh
docker buildx build --platform linux/arm64 -t balena-multi-container-telemetry:arm64 .
```