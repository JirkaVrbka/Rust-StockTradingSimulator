#[macro_use]
use crate::Stonker;
use crate::repos::stonker_repo::StonkerRepo;
use crate::PostgresStonkerRepo;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool, PoolError, PooledConnection};
use dotenv::dotenv;
use std::env;

pub type PgPool = Pool<ConnectionManager<PgConnection>>;
pub type PgPooledConnection = PooledConnection<ConnectionManager<PgConnection>>;

fn init_pool(database_url: &str) -> Result<PgPool, PoolError> {
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::builder().build(manager)
}

pub fn establish_connection() -> PgPool {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url));

    init_pool(&database_url).expect("Failed to create pool")
}

pub async fn test_connection() {
    let pool = establish_connection();
    let stonker_repo = PostgresStonkerRepo::new(pool);
    let stonkers: Vec<Stonker> = stonker_repo
        .get_stonkers()
        .await
        .expect("Fetching stonkers failed");
    println!("Displaying {} stonkers", stonkers.len());
    for entity in stonkers {
        println!("id: {} name: {}", entity.id, entity.name);
    }
}
