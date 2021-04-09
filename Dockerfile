FROM rust:1.50-alpine as builder

WORKDIR /API
COPY . .
RUN apk add --no-cache musl-dev
RUN cargo build --release --target-dir ./build

FROM alpine
WORKDIR /var/api/
COPY --from=builder /API/build/release/price-cryptocurrencies .
EXPOSE 8080
RUN ls
ENTRYPOINT ./price-cryptocurrencies
