ARG BUILD_FROM=homeassistant/amd64-base

FROM rust:latest as builder
WORKDIR /usr/src/myapp
COPY ./src/mqtt-async-client-rs .
RUN cargo install --no-default-features --example mqttc --path .

FROM ${BUILD_FROM}

COPY --from=builder /usr/local/cargo/bin/mqttc /usr/local/bin/mqttc
COPY rootfs /
