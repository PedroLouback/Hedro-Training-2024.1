FROM rust:latest AS build

WORKDIR /build
COPY . .
RUN apt-get update && \
    apt-get install -y cmake protobuf-compiler && \
    cargo build --release

FROM rust:slim-bookworm

RUN apt-get update && apt-get install -y awscli

WORKDIR /app
COPY --from=build /build/.env /build/target/release/grpc /app/

CMD ["./grpc"]