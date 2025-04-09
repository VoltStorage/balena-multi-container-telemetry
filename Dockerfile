FROM debian:latest

ARG BINARY_PATH

RUN apt-get update && apt-get install -y \
    libssl-dev \
    jq

WORKDIR /app

COPY $BINARY_PATH /app

CMD ["./balena-multi-container-telemetry"]
