use crate::schema::models::stock::{NewStock, Stock};
use crate::schema::repos::stock_repo::StockRepo;
use crate::PostgresStockRepo;
use actix_web::web;
use actix_web::{get, post, HttpResponse, Result};

#[get("/stocks")]
pub async fn get_stocks(repo: web::Data<PostgresStockRepo>) -> Result<HttpResponse> {
    let stocks: Vec<Stock> = repo.get_stocks().await.expect("Fetching stocks failed");
    Ok(HttpResponse::Ok().json(stocks))
}

#[get("/stocks/{id}")]
pub async fn get_stock(
    repo: web::Data<PostgresStockRepo>,
    id: web::Path<i32>,
) -> Result<HttpResponse> {
    let stock: Stock = repo
        .get_stock_by_id(*id)
        .await
        .expect("Fetching stocks failed");
    Ok(HttpResponse::Ok().json(stock))
}

#[post("stocks")]
pub async fn create_stock(repo: web::Data<PostgresStockRepo>, stock_data: web::Json<NewStock>) -> Result<HttpResponse> {
    let new_stock = NewStock {
        stonker_id: stock_data.stonker_id,
        company_id: stock_data.company_id,
    };
    let stock: Stock = repo
        .create_stock(new_stock)
        .await
        .expect("Fetching stocks failed");
    Ok(HttpResponse::Ok().json(stock))
}
