FROM rust:latest AS final

WORKDIR /app

COPY . .

RUN cargo build