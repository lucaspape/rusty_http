FROM rust:latest

RUN apt-get update && apt-get install libfcgi-bin -y

RUN USER=root cargo new --bin rusty_http
WORKDIR /rusty_http

COPY ./Cargo.toml ./Cargo.toml

RUN cargo build --release
RUN rm src/*.rs

COPY ./src ./src

RUN rm ./target/release/deps/rusty_http*
RUN cargo install --path .

USER www-data

CMD ["rusty_http"]