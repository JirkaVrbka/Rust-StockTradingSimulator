pub mod endpoints;
pub mod models;
pub mod repos;
pub mod schema;

#[macro_use]
extern crate diesel;
extern crate dotenv;
use crate::endpoints::stonker_endpoints::get_stonkers;
use crate::models::stonker::Stonker;
use crate::repos::connection::establish_connection;
use crate::repos::stonker_repo::PostgresStonkerRepo;
use actix_web::{web, App, HttpServer};
use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};

pub type PgPool = Pool<ConnectionManager<PgConnection>>;
pub type PgPooledConnection = PooledConnection<ConnectionManager<PgConnection>>;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = establish_connection();
    let stonker_repo = PostgresStonkerRepo::new(pool);

    HttpServer::new(move || App::new().data(stonker_repo.clone()).service(get_stonkers))
        .bind(("127.0.0.1", 8081))?
        .run()
        .await
}
