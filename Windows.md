# Windows Setup
## Installation of services
First, install Docker and PostgreSQL separately. 

```
https://www.docker.com/get-started
```

```
https://www.enterprisedb.com/downloads/postgres-postgresql-downloads
```

## Disable Postgres

Postgres runs automatically on background.

You can check that by typing

```
psql -h localhost -p 5432 -U postgres
```

If no command was found, add `D:\Programs\PostgreSQL\14\bin` to your PATH.

You will be prompted with a password, if it doesn't work, you also need to alter this file a
nd change the verification of `IPv6 local connections` from `scram-sha-256` to `trust` or `password`.

```
C:\Program Files\PostgreSQL\14\data\pg_hba.conf
```

To show all running tables run this command
```
\list
```

Next, continue to disable service "postgres" which runs automatically on 
background.

start Run (Win+R) and type

```
services.msc
```

Find postgres, set it to manual ("Ručně") and stop ("Zastavit") it.

## Environment

Next, add create `config.toml` (**not CARGO.TOML**) in `C:\Users\johnDoe\.cargo`. 

```
[target.x86_64-pc-windows-msvc.pq]
rustc-link-search = ["C:\\Program Files\\PostgreSQL\\14\\lib"]
rustc-link-lib = ["libpq"]
```

Then run
```
cargo clean
```

Add this to your path

```
PATH should include postgresbin
LIB_PQ_DIR C:\Program Files\PostgreSQL\14\lib
DATABASE_URL postgres://postgres:example@localhost:5432/stocks
```

## Create database
Now **restart pc** and run the container.

```
docker-compose -f .\stack.yml up
```

Now go to adminer `http://localhost:8080/` and create database called `stocks`

And now the final command
```
cargo install diesel_cli --no-default-features --features postgres
diesel migration run
```

Which should alter our tables, we should see the update on container window and also on Adminer.

## Webpages
```
cargo install trunk wasm-bindgen-cli
rustup target add wasm32-unknown-unknown
```