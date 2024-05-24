# If you're reading this, I just use this to build a testing container.
# Don't want to accidentally clutter my filesystem...
FROM rust:latest AS final

WORKDIR /app

RUN apt update
RUN apt install nano

COPY . .

RUN cargo build