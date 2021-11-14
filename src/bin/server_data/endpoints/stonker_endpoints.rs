use crate::models::stock::Stock;
use crate::models::stonker::{NewStonker, Stonker};
use crate::repos::stonker_repo::StonkerRepo;
use crate::PostgresStonkerRepo;
use actix_web::web;
use actix_web::{get, post, HttpResponse, Result};

#[get("/stonkers")]
pub async fn get_stonkers(repo: web::Data<PostgresStonkerRepo>) -> Result<HttpResponse> {
    let stonkers: Vec<Stonker> = repo.get_stonkers().await.expect("Fetching stonkers failed");
    Ok(HttpResponse::Ok().json(stonkers))
}

#[get("/stonkers/{id}")]
pub async fn get_stonker(
    repo: web::Data<PostgresStonkerRepo>,
    id: web::Path<i32>,
) -> Result<HttpResponse> {
    let stonker: Stonker = repo
        .get_stonker_by_id(*id)
        .await
        .expect("Fetching stonkers failed");
    Ok(HttpResponse::Ok().json(stonker))
}

#[post("stonkers")]
pub async fn create_stonker(
    repo: web::Data<PostgresStonkerRepo>,
    stonker_data: web::Json<NewStonker>,
) -> Result<HttpResponse> {
    let new_stonker = NewStonker {
        name: stonker_data.name.clone(),
        balance: stonker_data.balance,
    };
    let stonker: Stonker = repo
        .create_stonker(new_stonker)
        .await
        .expect("Fetching stonkers failed");
    Ok(HttpResponse::Ok().json(stonker))
}

#[get("/stonkers/{id}/stocks")]
pub async fn get_stonker_stocks(
    repo: web::Data<PostgresStonkerRepo>,
    id: web::Path<i32>,
) -> Result<HttpResponse> {
    let stonker_stocks: Vec<Stock> = repo
        .get_stonker_stocks(*id)
        .await
        .expect("Fetching stonker stocks failed");
    Ok(HttpResponse::Ok().json(stonker_stocks))
}
