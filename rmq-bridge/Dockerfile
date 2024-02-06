FROM rust:latest AS build

WORKDIR /build
COPY . .
RUN apt update && apt install cmake -y && cargo build --release
ADD https://raw.githubusercontent.com/vishnubob/wait-for-it/master/wait-for-it.sh /usr/local/bin/wait-for-it.sh
RUN chmod +x /usr/local/bin/wait-for-it.sh

FROM rust:slim-bookworm

WORKDIR /app
COPY --from=build /build/.env /build/target/release/rmq-bridge /app/

COPY --from=build /usr/local/bin/wait-for-it.sh /usr/local/bin/wait-for-it.sh
RUN chmod +x /usr/local/bin/wait-for-it.sh

CMD ["wait-for-it.sh", "rabbitmq:5672", "--", "./rmq-bridge"]