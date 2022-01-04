# Rust-StockTradingSimulator
[![diagram](diagram.png)](http://localhost:8080/?pgsql=db&username=postgres&db=stocks&ns=public&schema=history%3A16.5278x9.0278_company%3A38.6806x19.7223_command%3A11.5278x17.9167_news%3A25.6945x18.5417_stock%3A26.6667x9.2361_stonker%3A32.9167x-2.1528)

## Installation
* Unix - follow instructions in this file
* [Windows Setup](Windows.md) - then follow these instructions

## Run the application
### Server
#### Docker Database
```
docker-compose -f .\stack.yml up
```
[adminer](http://localhost:8080)
username: postgres

Run new migrations
```
diesel migration run
```

If there is some problem you can always delete stocks database and create new within adminer like a so:
```sql
drop database stocks;
create database stocks
```

#### Server page
```
cargo run --bin server
```
[stonkers.json](http://localhost:8081/stonkers)

#### In case of problem
* Create a new file in project root called '.env'
* Add into file 'DATABASE_URL=postgres://postgres:example@localhost:5432/stocks'

### Client
#### First time setup
```
cargo install trunk wasm-bindgen-cli
rustup target add wasm32-unknown-unknown
```

#### Build only
```
cargo build --bin client --target wasm32-unknown-unknown
```

#### Run and watch
```
trunk --config ./client/trunk.toml serve
```
[webpage](http://localhost:5000/)

## Ports
* `localhost:8080` - Adminer - manage database from browser
* `localhost:8081` - Server side webpage full of json
* `localhost:5432` - Database - manage database from code
* `localhost:5000` - Webpage client app full of nice css

## Endpoints
### Stonker endpoints
- `GET http://localhost:8081/stonkers`
    - get all stonkers
- `GET http://localhost:8081/stonkers/{id}`
    - get stonker by id
- `GET http://localhost:8081/stonkers/{id}/overview`
    - get stonker overview by id
- `POST http://localhost:8081/stonkers`
    - create a stonker
    - Request Body: {name: string, balance: number}
- `GET http://localhost:8081/stonkers/{id}/stocks`
    - get stonker stocks

### Company endpoints
- `GET http://localhost:8081/companies`
    - get all companies
- `GET http://localhost:8081/companies/{id}`
    - get company by id
- `GET http://localhost:8081/companies/{id}/stocks`
    - get company stocks


### Stock endpoints
- `GET http://localhost:8081/stocks`
  - get all stocks
- `GET http://localhost:8081/stocks/{id}`
    - get stock by id
- `POST http://localhost:8081/stocks`
    - create a stock
    - Request Body: {stonker_id: string, company_id: number}

### News endpoints
- `GET http://localhost:8081/news`
    - get all stocks

## Install Diesel CLI
`cargo install diesel_cli --no-default-features --features postgres`

## Install yew
* `rustup target add wasm32-unknown-unknown` rust compiles to wasm
* `cargo install trunk wasm-bindgen-cli` trunk serve

### Unix Problems
In case of problems: `sudo apt install libpq-dev`
Alternatively: `sudo apt install libpq-dev libmysqlclient-dev`

## Create migration
`diesel migration generate <migration_name>`

## Deployment

### Database (Postgresql)
connection string: postgres://rust:<password>@pv287-stockss.postgres.database.azure.com/postgres?sslmode=require
If you want to access database directly, you need to add you IP addres in firewall rules


### Backend-server
stock-server:
- Compile server: `cargo build --bin server --release --features server_deps`
- Build docker image: `docker build --tag pv281.azurecr.io/stocks-server:<tag> .`
- Run the container: `docker run -p 8081:8081 pv281.azurecr.io/stocks-server:<tag>`
- Login to azure container registry: `docker login pv281.azurecr.io` (user name: `pv281`)
- Push image: `docker push pv281.azurecr.io/stocks-server:<tag>`
- Run the container in azure portal.
- The server should run at: `http://pv281-stock.germanywestcentral.azurecontainer.io:8081`

### Front-end
front-end:
- Compile server: `cargo build --bin client --target wasm32-unknown-unknown`
- Build docker image: `docker build -f Dockerfile-client --tag pv281.azurecr.io/stocks-server-client:<tag> .`
- Run the container: `docker push pv281.azurecr.io/stocks-server-client:<tag> `
- Login to azure container registry: `docker login pv281.azurecr.io` (user name: `pv281`)
- Push image: `docker push pv281.azurecr.io/stocks-server-client:<tag>`
- Run the container in azure portal.
- The server should run at: `http://pv281-stock-fe.germanywestcentral.azurecontainer.io:5000/`
