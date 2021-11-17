use crate::models::stonker::NewStonker;
use crate::repos::stonker_repo::StonkerRepo;
use crate::server_data::models::api_error::handle_api_result;
use crate::PostgresStonkerRepo;
use actix_web::web;
use actix_web::{get, post, HttpResponse, Result};

#[get("/stonkers")]
pub async fn get_stonkers(repo: web::Data<PostgresStonkerRepo>) -> Result<HttpResponse> {
    let stonkers_result = repo.get_stonkers().await;
    handle_api_result(stonkers_result)
}

#[get("/stonkers/{id}")]
pub async fn get_stonker(
    repo: web::Data<PostgresStonkerRepo>,
    id: web::Path<i32>,
) -> Result<HttpResponse> {
    let stonker_result = repo.get_stonker_by_id(*id).await;
    handle_api_result(stonker_result)
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
    let stonker_result = repo.create_stonker(new_stonker).await;
    handle_api_result(stonker_result)
}

#[get("/stonkers/{id}/stocks")]
pub async fn get_stonker_stocks(
    repo: web::Data<PostgresStonkerRepo>,
    id: web::Path<i32>,
) -> Result<HttpResponse> {
    let stonker_stocks = repo.get_stonker_stocks(*id).await;
    handle_api_result(stonker_stocks)
}
