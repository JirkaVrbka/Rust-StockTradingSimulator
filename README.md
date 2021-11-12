# Rust-StockTradingSimulator

## Start application in Docker
`docker-compose -f stack.yml up`

## View DB in Adminer
`http://localhost:8080/`  
`username: postgres`

## Install Diesel CLI
`cargo install diesel_cli`
`cargo install diesel_cli --no-default-features --features postgres`

In case of problems: `sudo apt install libpq-dev`

## Create migration
`diesel migration generate <migration_name>`