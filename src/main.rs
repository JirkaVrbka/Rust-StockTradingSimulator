pub mod models;
pub mod repos;
pub mod schema;

#[macro_use]
extern crate diesel;
extern crate dotenv;
use actix_web::HttpRequest;
use crate::models::stonker::Stonker;
use crate::repos::connection::establish_connection;
use crate::repos::stonker_repo::PostgresStonkerRepo;
use crate::repos::stonker_repo::StonkerRepo;
use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

pub type PgPool = Pool<ConnectionManager<PgConnection>>;
pub type PgPooledConnection = PooledConnection<ConnectionManager<PgConnection>>;

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



async fn get_stonkers(req: HttpRequest) -> impl Responder {
    format!("Hello")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/stonkers", web::get().to(get_stonkers))
    })
    .bind(("127.0.0.1", 8081))?
    .run()
    .await
}
