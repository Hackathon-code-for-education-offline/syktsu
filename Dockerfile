FROM rust:1-bullseye

WORKDIR /APP
COPY . .

RUN cargo install trunk && 