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

use crate::endpoints::company_endpoints::{get_companies, get_company, get_company_stocks};
use crate::endpoints::news_endpoints::get_news;
use crate::endpoints::stock_endpoints::{create_stock, get_stock, get_stocks, trade_stock, trade_stock_g};
use crate::endpoints::stonker_endpoints::{create_stonker, get_curr_stonker_stocks, get_stonker, get_stonker_overview, get_stonker_stocks, get_stonkers, login};
use actix_cors::Cors;
use actix_web::middleware::Logger;
use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use env_logger;
use crate::repos::Repo;

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
    let repo = Repo::new();
    let chat_server = Lobby::default().start(); //create and spin up a lobby
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(
                Cors::permissive()
            )
            .data(repo.clone())
            .data(chat_server.clone()) //register the lobby
            .service(login)
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
            .service(get_curr_stonker_stocks)
            .service(get_stonker_stocks)
            .service(get_news)
            .service(trade_stock)
            .service(trade_stock_g)
            .service(start_connection_route) //register our route. rename with "as" import or naming conflict
    })
    .bind(("0.0.0.0", 8081))?
    .run()
    .await
}