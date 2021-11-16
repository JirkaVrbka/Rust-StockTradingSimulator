use crate::models::stock::NewStock;
use crate::repos::stock_repo::StockRepo;
use crate::server_data::models::api_error::handle_api_result;
use crate::PostgresStockRepo;
use actix_web::web;
use actix_web::{get, post, HttpResponse, Result};

#[get("/stocks")]
pub async fn get_stocks(repo: web::Data<PostgresStockRepo>) -> Result<HttpResponse> {
    let stocks_result = repo.get_stocks().await;
    handle_api_result(stocks_result)
}

#[get("/stocks/{id}")]
pub async fn get_stock(
    repo: web::Data<PostgresStockRepo>,
    id: web::Path<i32>,
) -> Result<HttpResponse> {
    let stock_result = repo.get_stock_by_id(*id).await;
    handle_api_result(stock_result)
}

#[post("stocks")]
pub async fn create_stock(
    repo: web::Data<PostgresStockRepo>,
    stock_data: web::Json<NewStock>,
) -> Result<HttpResponse> {
    let new_stock = NewStock {
        stonker_id: stock_data.stonker_id,
        company_id: stock_data.company_id,
    };
    let stock_result = repo.create_stock(new_stock).await;
    handle_api_result(stock_result)
}
