FROM rust:1.50-slim

EXPOSE 8080

WORKDIR /API
COPY . .
## adicionar um gerenciado da api no futuro
RUN cargo build --release --target-dir ./build

ENTRYPOINT build/release/price-cryptocurrencies
