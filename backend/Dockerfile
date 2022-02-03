# 開発環境
FROM rust:1.58.1 as develop-stage
WORKDIR /app
RUN cargo install cargo-watch

FROM rust:1.58.1-slim-bullseye AS production
WORKDIR /app
COPY ./Cargo.toml ./Cargo.toml
COPY ./backend/Cargo.toml ./backend/Cargo.toml
COPY ./frontend/Cargo.toml ./frontend/Cargo.toml
RUN mkdir -p ./backend/src
RUN echo "fn main(){}" > ./backend/src/main.rs
COPY ./quiz_json ./quiz_json
COPY ./backend/src ./backend/src
COPY ./frontend/src ./frontend/src
COPY ./backend/static ./backend/static
RUN cargo build --release -p rustknock-backend
RUN rm -f ./target/release/deps/rustknock-backend*
RUN cargo build --release -p rustknock-backend
RUN mv ./target/release/rustknock-backend /usr/local/bin
# 外部ファイルとして読み込む系は以下で
COPY ./quizzes.json ./quizzes.json
EXPOSE 3000
CMD ["rustknock-backend"]