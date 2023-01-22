[![Rust](https://github.com/dialMForMonkey/api-price-cryptocurrencies/actions/workflows/rust.yml/badge.svg?branch=main)](https://github.com/dialMForMonkey/api-price-cryptocurrencies/actions/workflows/rust.yml)
[![Docker Image CI](https://github.com/dialMForMonkey/api-price-cryptocurrencies/actions/workflows/docker-image.yml/badge.svg)](https://github.com/dialMForMonkey/api-price-cryptocurrencies/actions/workflows/docker-image.yml)
# Api Price Cryptocurrences

This api get price of Cryptocurrences, from mercadobitcoin

This repository only for study, but if you want run this repo, execute down steps and have fun.

``` sh
docker-compose up 

curl --location --request GET 'http://localhost:8080/v1/money'
# or 
curl --location --request GET 'http://localhost:8080/v1/money?money=btc'

# or 
docker pull dialmformonkey/api-price-cryptocurrencies

```
