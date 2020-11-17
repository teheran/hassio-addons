FROM rust:latest as builder
WORKDIR /usr/src/myapp
COPY ./mqtt-async-client-rs .
RUN cargo install --no-default-features --example mqttc --path .

FROM debian:buster-slim
COPY --from=builder /usr/local/cargo/bin/mqttc /usr/local/bin/mqttc
CMD ["mqttc"]