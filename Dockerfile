FROM rust:latest

RUN USER=root cargo new --bin nfl-rushing
WORKDIR /nfl-rushing

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
COPY ./benches ./benches
COPY ./tests ./tests

RUN cargo build --release
RUN rm src/*.rs

COPY ./src ./src
COPY ./rushing.json ./rushing.json

RUN cargo build --release