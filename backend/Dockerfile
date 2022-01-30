# 開発環境
FROM rust:1.58.1 as develop-stage
WORKDIR /app
RUN cargo install cargo-watch
COPY . .

# ビルド環境
FROM rust:1.58.1 AS builder
WORKDIR /app
COPY Cargo.toml Cargo.toml
RUN mkdir src
RUN echo "fn main(){}" > src/main.rs
RUN cargo build --release
COPY ./src ./src
RUN rm -f target/release/deps/hello_actix*
RUN cargo build --release

# 本番環境
FROM rust:1.58.1-slim-bullseye
COPY --from=builder /app/target/release/hello-actix /usr/local/bin/hello-actix
EXPOSE 3000
CMD ["hello-actix"]
