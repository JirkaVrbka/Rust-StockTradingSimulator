mod websockets;
use websockets::lobby::Lobby;
use websockets::start_connection::start_connection as start_connection_route;
use actix::Actor;
use actix_web::{App, HttpServer};

pub mod server_data;
use server_data::*;

#[macro_use]
extern crate diesel;
extern crate dotenv;
use std::sync::Arc;

use crate::endpoints::company_endpoints::{get_companies, get_company, get_company_stocks};
use crate::endpoints::news_endpoints::get_news;
use crate::endpoints::stock_endpoints::{create_stock, get_stock, get_stocks};
use crate::endpoints::stonker_endpoints::{
    create_stonker, get_stonker,get_stonker_overview, get_stonker_stocks, get_stonkers,
};
use crate::repos::company_repo::PostgresCompanyRepo;
use crate::repos::connection::establish_connection;
use crate::repos::news_repo::PostgresNewsRepo;
use crate::repos::stock_repo::PostgresStockRepo;
use crate::repos::stonker_repo::PostgresStonkerRepo;
use actix_cors::Cors;
use actix_web::middleware::Logger;
use actix_web::http::header;
use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use env_logger;

pub type PgPool = Pool<ConnectionManager<PgConnection>>;
pub type PgPooledConnection = PooledConnection<ConnectionManager<PgConnection>>;

fn init_logger() {
    std::env::set_var("RUST_LOG", "info");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    init_logger();

    let pool = match establish_connection() {
        Ok(p) => Arc::new(p),
        Err(_) => panic!("Cannot establish connection"),
    };
    let stonker_repo = PostgresStonkerRepo::new(pool.clone());
    let company_repo = PostgresCompanyRepo::new(pool.clone());
    let stock_repo = PostgresStockRepo::new(pool.clone());
    let news_repo = PostgresNewsRepo::new(pool.clone());
    let chat_server = Lobby::default().start(); //create and spin up a lobby
    println!("Utils: {}", utils::hello());
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
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
            .data(news_repo.clone())
            .data(chat_server.clone()) //register the lobby
            .service(get_stonkers)
            .service(get_stonker)
            .service(get_stonker_overview)
            .service(get_companies)
            .service(get_company)
            .service(create_stonker)
            .service(get_stocks)
            .service(get_stock)
            .service(create_stock)
            .service(get_company_stocks)
            .service(get_stonker_stocks)
            .service(get_news)
            .service(start_connection_route) //register our route. rename with "as" import or naming conflict
    })
    .bind(("0.0.0.0", 8081))?
    .run()
    .await
}