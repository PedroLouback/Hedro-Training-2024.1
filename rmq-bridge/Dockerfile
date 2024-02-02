FROM alpine AS build

WORKDIR /build

RUN apk add --no-cache rust cargo

RUN apk add --no-cache cmake pkgconf openssl-dev make

COPY . .
RUN cargo build --release

FROM alpine

RUN apk --no-cache add libgcc

WORKDIR /app
COPY .env ./
COPY --from=build /build/target/release/rmq-bridge ./

CMD ["sh", "-c", "./rmq-bridge"]