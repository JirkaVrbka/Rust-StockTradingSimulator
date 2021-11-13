pub mod models;
pub mod schema;

#[macro_use]
extern crate diesel;
extern crate dotenv;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool, PoolError, PooledConnection};
use dotenv::dotenv;
use std::env;

use crate::models::stonker::Stonker;

pub type PgPool = Pool<ConnectionManager<PgConnection>>;
pub type PgPooledConnection = PooledConnection<ConnectionManager<PgConnection>>;

fn init_pool(database_url: &str) -> Result<PgPool, PoolError> {
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::builder().build(manager)
}

pub fn establish_connection() -> PgPool {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL")
         .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url));

    init_pool(&database_url).expect("Failed to create pool")
}

pub fn test_connection() {
    use self::schema::stonker::dsl::*;
    let connection = establish_connection().get().expect("Pool failed");
    let results = stonker
        .filter(name.eq(String::from("stonker")))
        .limit(5)
        .load::<Stonker>(&connection)
        .expect("Error loading stonkers");

    println!("Displaying {} stonkers", results.len());
    for entity in results {
        println!("{}", entity.id);
    }
}

fn main() {
    test_connection();
}
