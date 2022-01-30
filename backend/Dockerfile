# 開発環境
FROM rust:1.58.1 as develop-stage
WORKDIR /app
RUN cargo install cargo-watch
COPY . .

# ビルド環境
FROM develop-stage as build-stage
RUN cargo build --release

# 本番環境
FROM rust:1.58.1-slim-bullseye
COPY --from=build-stage /app/target/release/hello-actix .
EXPOSE 8088
CMD ["/usr/local/bin/hello-actix"]
