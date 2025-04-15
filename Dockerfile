FROM rust:latest AS builder

RUN apt update && apt upgrade -y
RUN apt install -y g++-aarch64-linux-gnu libc6-dev-arm64-cross cmake

RUN rustup target add aarch64-unknown-linux-gnu
RUN rustup toolchain install stable-aarch64-unknown-linux-gnu

WORKDIR /app

COPY . .

ENV CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER=aarch64-linux-gnu-gcc CC_aarch64_unknown_linux_gnu=aarch64-linux-gnu-gcc CXX_aarch64_unknown_linux_gnu=aarch64-linux-gnu-g++

RUN ["cargo", "build", "--release", "--target", "aarch64-unknown-linux-gnu"]


FROM debian:latest

RUN apt-get update && apt-get install -y \
    libssl-dev \
    jq

WORKDIR /app

COPY --from=builder /app/target/aarch64-unknown-linux-gnu/release/balena-multi-container-telemetry /app/balena-multi-container-telemetry

CMD ["./balena-multi-container-telemetry"]
