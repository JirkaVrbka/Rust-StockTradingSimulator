pub mod server_data;
use server_data::*;
pub mod utils;

#[macro_use]
extern crate diesel;
extern crate dotenv;
use std::sync::Arc;

use crate::endpoints::company_endpoints::{get_companies, get_company, get_company_stocks};
use crate::endpoints::stock_endpoints::{create_stock, get_stock, get_stocks};
use crate::endpoints::stonker_endpoints::{
    create_stonker, get_stonker, get_stonker_stocks, get_stonkers,
};
use crate::repos::company_repo::PostgresCompanyRepo;
use crate::repos::connection::establish_connection;
use crate::repos::stock_repo::PostgresStockRepo;
use crate::repos::stonker_repo::PostgresStonkerRepo;
use actix_cors::Cors;
use actix_web::{http::header, App, HttpServer};
use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};

pub type PgPool = Pool<ConnectionManager<PgConnection>>;
pub type PgPooledConnection = PooledConnection<ConnectionManager<PgConnection>>;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = match establish_connection() {
        Ok(p) => Arc::new(p),
        Err(_) => panic!("Cannot establish connection"),
    };
    let stonker_repo = PostgresStonkerRepo::new(pool.clone());
    let company_repo = PostgresCompanyRepo::new(pool.clone());
    let stock_repo = PostgresStockRepo::new(pool.clone());
    println!("Utils: {}", utils::hello());
    HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::default()
                    .allowed_origin("http://localhost:5000")
                    .allowed_methods(vec!["GET", "POST", "DELETE", "PUT"])
                    .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
                    .allowed_header(header::CONTENT_TYPE)
                    .supports_credentials()
                    .max_age(3600),
            )
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
            .service(get_company_stocks)
            .service(get_stonker_stocks)
    })
    .bind(("0.0.0.0", 8081))?
    .run()
    .await
}
