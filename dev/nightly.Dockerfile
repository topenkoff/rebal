FROM rustlang/rust:nightly-bullseye-slim as builder
WORKDIR /usr/src/rebal
RUN mkdir src
RUN echo "fn main() {}" > src/main.rs
COPY Cargo.toml Cargo.lock .
RUN cargo build --release --all-features
COPY . .
ARG FEATURES
RUN cargo build --release --features ${FEATURES}

FROM debian:bullseye-slim
COPY --from=builder /usr/src/rebal/target/release/rebal /usr/local/bin/rebal
COPY --from=builder /usr/src/rebal/dev/config.yml /usr/local/bin/config.yml
EXPOSE 5000

CMD ["rebal"]
