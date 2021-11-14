pub mod endpoints;
pub mod models;
pub mod repos;
pub mod schema;

#[macro_use]
extern crate diesel;
extern crate dotenv;
use std::sync::Arc;

use crate::endpoints::stonker_endpoints::{get_stonker, get_stonkers, create_stonker};
use crate::endpoints::stock_endpoints::{get_stocks, get_stock, create_stock};
use crate::endpoints::company_endpoints::{get_companies, get_company};
use crate::repos::company_repo::PostgresCompanyRepo;
use crate::repos::connection::establish_connection;
use crate::repos::stonker_repo::PostgresStonkerRepo;
use crate::repos::stock_repo::PostgresStockRepo;
use actix_web::{App, HttpServer};
use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};

pub type PgPool = Pool<ConnectionManager<PgConnection>>;
pub type PgPooledConnection = PooledConnection<ConnectionManager<PgConnection>>;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = Arc::new(establish_connection());
    let stonker_repo = PostgresStonkerRepo::new(pool.clone());
    let company_repo = PostgresCompanyRepo::new(pool.clone());
    let stock_repo = PostgresStockRepo::new(pool.clone());

    HttpServer::new(move || {
        App::new()
            .data(stonker_repo.clone())
            .data(company_repo.clone())
            .data(stock_repo.clone())
            .service(get_stonkers)
            .service(get_stonker)
            .service(get_companies)
            .service(get_company)
            .service(create_stonker)
            .service(get_stocks)
            .service(get_stock)
            .service(create_stock)
    })
    .bind(("127.0.0.1", 8081))?
    .run()
    .await
}
