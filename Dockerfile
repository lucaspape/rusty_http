FROM rust:latest

RUN USER=root cargo new --bin rusty_http
WORKDIR /rusty_http

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

RUN cargo build --release
RUN rm src/*.rs

COPY ./src ./src

RUN rm ./target/release/deps/rusty_http*
RUN cargo install --path .

CMD ["rusty_http"]