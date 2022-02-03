# 開発環境
FROM rust:1.58.1 as develop-stage
WORKDIR /app
RUN cargo install cargo-watch
COPY ./ ./


FROM rust:1.58.1-slim-bullseye AS production
WORKDIR /app
COPY ./Cargo.toml ./Cargo.toml
RUN mkdir ./src
RUN echo "fn main(){}" > ./src/main.rs
RUN cargo build --release
RUN rm -f ./target/release/deps/hello_actix*
COPY ./src ./src
RUN cargo build --release
RUN mv ./target/release/hello-actix /usr/local/bin
RUN rm -rf ./src ./target
# 外部ファイルとして読み込む系は以下で
COPY ./quizzes.json ./quizzes.json
EXPOSE 3000
CMD ["hello-actix"]