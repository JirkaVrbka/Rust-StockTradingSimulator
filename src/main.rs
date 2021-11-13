pub mod models;
pub mod repos;
pub mod schema;
pub mod endpoints;

#[macro_use]
extern crate diesel;
extern crate dotenv;
use crate::endpoints::stonker_endpoints::get_stonkers;
use crate::models::stonker::Stonker;
use crate::repos::connection::establish_connection;
use crate::repos::stonker_repo::PostgresStonkerRepo;
use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use actix_web::{web, App, HttpServer};

pub type PgPool = Pool<ConnectionManager<PgConnection>>;
pub type PgPooledConnection = PooledConnection<ConnectionManager<PgConnection>>;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = establish_connection();
    let stonker_repo = PostgresStonkerRepo::new(pool);

    HttpServer::new(move || {
        App::new()
            .data(stonker_repo.clone())
            .route("/stonkers", web::get().to(get_stonkers))
    })
    .bind(("127.0.0.1", 8081))?
    .run()
    .await
}
