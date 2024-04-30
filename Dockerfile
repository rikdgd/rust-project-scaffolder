FROM rust:latest AS final

WORKDIR /app

RUN apt update
RUN apt install nano

COPY . .

RUN cargo build