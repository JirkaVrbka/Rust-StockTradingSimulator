use crate::models::stock::NewStock;
use crate::server_data::endpoints::handle_api_result;
use crate::server_data::repos::Repo;
use crate::server_data::repos::stock_repo::StockRepo;
use actix_web::web;
use actix_web::{get, post, HttpResponse, Result};

#[get("/stocks")]
pub async fn get_stocks(repo: web::Data<Repo>) -> Result<HttpResponse> {
    let stocks_result = repo.get_stocks().await;
    handle_api_result(stocks_result)
}

#[get("/stocks/{id}")]
pub async fn get_stock(
    repo: web::Data<Repo>,
    id: web::Path<i32>,
) -> Result<HttpResponse> {
    let stock_result = repo.get_stock_by_id(*id).await;
    handle_api_result(stock_result)
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
    handle_api_result(stock_result)
}
