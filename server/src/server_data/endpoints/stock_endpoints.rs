use std::future::Future;
use std::ops::Deref;
use actix::FinishStream;
use crate::models::stock::NewStock;
use crate::server_data::endpoints::ApiError;
use crate::server_data::repos::Repo;
use crate::server_data::repos::stock_repo::StockRepo;
use actix_web::web;
use actix_web::{get, post, HttpResponse, Result};
use chrono::Utc;
use crate::models::command::{CommandTypes, NewCommand};
use utils::json::{CommandCreateJson, CommandTypesJSON};

#[get("/stocks")]
pub async fn get_stocks(repo: web::Data<Repo>) -> Result<HttpResponse> {
    let stocks_result = repo.get_stocks().await;
    ApiError::handle(stocks_result)
}

#[get("/stocks/{id}")]
pub async fn get_stock(
    repo: web::Data<Repo>,
    id: web::Path<i32>,
) -> Result<HttpResponse> {
    let stock_result = repo.get_stock_by_id(*id).await;
    ApiError::handle(stock_result)
}

#[post("stocks")]
pub async fn create_stock(
    repo: web::Data<Repo>,
    stock_data: web::Json<NewStock>,
) -> Result<HttpResponse> {
    let new_stock = NewStock {
        stonker_id: stock_data.stonker_id,
        company_id: stock_data.company_id,
        bought_for: stock_data.bought_for,
        share: stock_data.share,
    };
    let stock_result = repo.create_stock(new_stock).await;
    ApiError::handle(stock_result)
}

#[get("stocks/trade/{sid}/{cid}/{s}/{t}/{k}")]
pub async fn trade_stock_g(
    repo: web::Data<Repo>,
    sid: web::Path<i32>,
    cid: web::Path<i32>,
    s: web::Path<i32>,
    t: web::Path<i32>,
    k: web::Path<String>,
) -> Result<HttpResponse> {
    println!("Heyyyyyy {}", k.deref());
    let kind = match &(k.deref())[..] {
        "Sell" => CommandTypes::Sell,
        "SellIfHigh" => CommandTypes::SellIfHigh,
        "SellIfLow" => CommandTypes::SellIfLow,
        "BuyIfLow" => CommandTypes::BuyIfLow,
        _ => {return Ok(HttpResponse::BadRequest().body(String::from("No such command.")))}
    };

    let cmd = NewCommand {
        stonker_id: *sid,
        company_id: *cid,
        threshold: *t,
        share: *s,
        kind: kind.clone(),
        created_at: Utc::now().naive_utc()
    };
    let result = match kind {
        CommandTypes::Sell => {repo.sell_stock(cmd)}
        CommandTypes::SellIfHigh => {repo.sell_stock(cmd)}
        CommandTypes::SellIfLow => {repo.sell_stock(cmd)}
        CommandTypes::BuyIfLow => {repo.buy_stock(cmd)}
    };

    Ok(HttpResponse::Ok().finish())
}



#[post("stocks/trade")]
pub async fn trade_stock(
    repo: web::Data<Repo>,
    command: web::Json<CommandCreateJson>,
) -> Result<HttpResponse> {
    println!("Heyyyyyy");
    let kind = match command.kind {
        1 => CommandTypes::Sell,
        2 => CommandTypes::SellIfHigh,
        3 => CommandTypes::SellIfLow,
        4 => CommandTypes::BuyIfLow,
        _ => {return Ok(HttpResponse::Unauthorized().body(String::from("Bad trade command (must be 1 - 4")))}
    };
    let cmd = NewCommand {
        stonker_id: command.stonker_id,
        company_id: command.company_id,
        threshold: command.threshold,
        share: command.share,
        kind: kind.clone(),
        created_at: Utc::now().naive_utc()
    };
    let result = match kind {
        CommandTypes::Sell => {repo.sell_stock(cmd)}
        CommandTypes::SellIfHigh => {repo.sell_stock(cmd)}
        CommandTypes::SellIfLow => {repo.sell_stock(cmd)}
        CommandTypes::BuyIfLow => {repo.buy_stock(cmd)}
    };

    Ok(HttpResponse::Ok().finish())
}
