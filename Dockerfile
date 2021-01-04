FROM rust:1.49 as builder

RUN rustup target add wasm32-unknown-unknown

WORKDIR /workspace
COPY ./src /workspace
RUN cargo build --target=wasm32-unknown-unknown --release -p stat-filter

FROM envoyproxy/envoy-dev:0c9e7bfd887b9668507c7ab8e45c7100b7072b57
COPY --from=builder /workspace/target/wasm32-unknown-unknown/release/stat_filter.wasm /stat_filter.wasm
COPY ./envoy.yaml /etc/envoy/envoy.yaml