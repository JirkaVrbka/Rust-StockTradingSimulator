use diesel::dsl::Find;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::query_dsl::LoadQuery;
use diesel::query_dsl::methods::{FindDsl, LimitDsl};
use diesel::r2d2::{ConnectionManager, Pool, PoolError, PooledConnection};
use dotenv::dotenv;
use std::env;
use std::sync::Arc;
use anyhow::Context;
pub type PgPool = Pool<ConnectionManager<PgConnection>>;
pub type PgPooledConnection = PooledConnection<ConnectionManager<PgConnection>>;
use diesel::helper_types::Limit;

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
            Err(e) => panic!("Cannot establish connection: {}", e),
        };
        Self { pg_pool }
    }

    pub fn connect(&self) -> anyhow::Result<PgPooledConnection> {
        self
            .pg_pool
            .get()
            .context("500::::Cannot get connection from pool")
    }

    // table is immediately used as result, therefore its type is restricted only by LoadQuery
    pub fn all<Model, Table>(connection: &PgPooledConnection, table: Table, what: &str) -> anyhow::Result<Vec<Model>>
      where Table: LoadQuery<PgPooledConnection, Model> {
        table
            .load::<Model>(&connection)
            .context(format!("404::::Could not find {}", what))
    }

    // first is table search through using i32, therefore Table has bound FindDsl<i32>,
    // then it is limited, therefore Find<Table, i32> has bound LimitDsl,
    // lastly there's result, therefore Limit<Find<Table, i32>> has bound LoadQuery<PgPooledConnection, Model>
    pub fn find<Model, Table>(connection: &PgPooledConnection, table: Table, id: i32, what: &str) -> anyhow::Result<Model>
      where Table: FindDsl<i32>,
            Find<Table, i32>: LimitDsl,
            Limit<Find<Table, i32>>: LoadQuery<PgPooledConnection, Model> {
        table
            .find(id)
            .limit(1) // same as first()
            .get_result::<Model>(connection)
            .context(format!("404::::Could not find {} with id {}", what, id))
    }
}