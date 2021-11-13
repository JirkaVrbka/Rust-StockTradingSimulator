# Rust-StockTradingSimulator
![diagram](diagram.svg)

## Start application in Docker
`docker-compose -f stack.yml up`

## View DB in Adminer
`http://localhost:8080/`  
`username: postgres`

## Install Diesel CLI
`cargo install diesel_cli`
`cargo install diesel_cli --no-default-features --features postgres`

### Unix Problems
In case of problems: `sudo apt install libpq-dev`

### Windows Problems
First, install Docker and PostgreSQL separately. 

```
https://www.docker.com/get-started
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

## Create migration
`diesel migration generate <migration_name>`