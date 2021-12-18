pub mod company_repo;
pub mod news_repo;
pub mod stock_repo;
pub mod stonker_repo;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool, PoolError, PooledConnection};
use dotenv::dotenv;
use std::env;
use std::sync::Arc;
use anyhow::Context;

pub type PgPool = Pool<ConnectionManager<PgConnection>>;
pub type PgPooledConnection = PooledConnection<ConnectionManager<PgConnection>>;

#[derive(std::clone::Clone)]
pub struct Repo {
    pg_pool: Arc<PgPool>
}

impl Repo {
    fn init_pool(database_url: &str) -> Result<PgPool, PoolError> {
        let manager = ConnectionManager::<PgConnection>::new(database_url);
        Pool::builder().build(manager)
    }

    fn establish_connection() -> anyhow::Result<PgPool> {
        dotenv().expect("Env file was not loaded correctly");
        let database_url = env::var("DATABASE_URL")?;
        PgConnection::establish(&database_url)?;
        Ok(Repo::init_pool(&database_url)?)
    }

    pub fn new() -> Self {
        let pg_pool = match Repo::establish_connection() {
            Ok(p) => Arc::new(p),
            Err(_) => panic!("Cannot establish connection"),
        };
        Self { pg_pool }
    }

    pub fn connect(&self) -> anyhow::Result<PgPooledConnection> {
        self
            .pg_pool
            .get()
            .context("500::::Cannot get connection from pool")
    }
}