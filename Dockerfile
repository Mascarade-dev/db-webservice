FROM rust:1.95 as build-env
WORKDIR /app
COPY . /app
RUN rustup target add x86_64-unknown-linux-musl && \
    apt-get update && apt-get install -y musl-tools && \
    cargo build --release --target x86_64-unknown-linux-musl

FROM gcr.io/distroless/cc-debian12
COPY --from=build-env /app/target/x86_64-unknown-linux-musl/release/db-webservice /
ENTRYPOINT ["./db-webservice"]

