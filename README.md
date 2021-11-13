# Rust-StockTradingSimulator
![diagram](diagram.svg)

## Ports
* `localhost:8080` - Adminer - manage database from browser
* `localhost:8081` - Webpage - client app
* `localhost:5432` - Database - manage database from code

## Start application in Docker
`docker-compose -f stack.yml up`

## Endpoints
`GET http://localhost:8081/stonkers`

## View DB in Adminer
`http://localhost:8080/`  
`username: postgres`

## Install Diesel CLI
`cargo install diesel_cli --no-default-features --features postgres`

## Install yew
* `rustup target add wasm32-unknown-unknown` rust compiles to wasm 
* `cargo install trunk wasm-bindgen-cli` trunk serve

### Unix Problems
In case of problems: `sudo apt install libpq-dev`


### Windows Setup
#### Installation of services
First, install Docker and PostgreSQL separately. 

```
https://www.docker.com/get-started
```

```
https://www.enterprisedb.com/downloads/postgres-postgresql-downloads
```

Next, add this to `config.toml`. For example, create it in `C:\Users\johnDoe\.cargo`.

```
[target.x86_64-pc-windows-msvc.pq]
rustc-link-search = ["C:\\Program Files\\PostgreSQL\\14\\lib"]
rustc-link-lib = ["libpq"]
```

Then run
```
cargo clean
```

Now restart pc and run final command.
```
cargo install diesel_cli --no-default-features --features postgres
```

If it still doesn't work, update appropriate environment variables. 

```
PATH should include postgresbin
LIB_PQ_DIR C:\Program Files\PostgreSQL\14\lib
DATABASE_URL postgres://postgres:example@localhost:5432/stocks
```

#### Creating the database

Most frustrating thing, is that postgres runs automatically
on background.


You can check that by typing

```
psql -h localhost -p 5432 -U postgres
```

You will be prompted with a password, if it doesn't work, you
also need to alter this file and change the verification to `trust` or `password` or `md5`.

```
C:\Program Files\PostgreSQL\14\data\pg_hba.conf
```

Next, continue to disable service "postgres" which runs automatically on 
background.

start Run (Win+R) and type

```
services.msc
```

Find postgres, set it to manual ("Ručně") and stop ("Zastavit") it.

Next, run the container.

```
docker-compose -f .\stack.yml up
```

Now, we should be on the other terminal run

```
diesel migration run
```

which should alter our tables, we should see the update on container window and also on Adminer.

## Create migration
`diesel migration generate <migration_name>`