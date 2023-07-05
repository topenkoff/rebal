FROM rust:latest as builder
WORKDIR /usr/src/rebal
RUN mkdir src
RUN echo "fn main() {}" >> src/main.rs
COPY Cargo.toml Cargo.lock .
RUN cargo install --path .
COPY . .
RUN cargo install --path .

FROM debian:bullseye-slim
COPY --from=builder /usr/local/cargo/bin/rebal /usr/local/bin/rebal
COPY --from=builder /usr/src/rebal/dev/config.yml /usr/local/bin/config.yml
EXPOSE 5000

CMD ["rebal"]
