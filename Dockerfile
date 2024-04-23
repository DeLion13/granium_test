# Dockerfile

FROM rust:latest AS builder

# Set environment variables
ENV SERVER_PORT=8080

WORKDIR /usr/src/app

COPY . .

RUN cargo install --path .

CMD ["granium"]
