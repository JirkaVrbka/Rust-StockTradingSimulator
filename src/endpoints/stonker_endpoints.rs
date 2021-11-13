use crate::repos::stonker_repo::StonkerRepo;
use crate::PostgresStonkerRepo;
use crate::Stonker;
use actix_web::web;
use actix_web::{get, HttpResponse, Result};

#[get("/stonkers")]
pub async fn get_stonkers(repo: web::Data<PostgresStonkerRepo>) -> Result<HttpResponse> {
    let stonkers: Vec<Stonker> = repo.get_stonkers().await.expect("Fetching stonkers failed");
    Ok(HttpResponse::Ok().json(stonkers))
}

#[get("/stonkers/{id}")]
pub async fn get_stonker(repo: web::Data<PostgresStonkerRepo>, id: web::Path<i32>) -> Result<HttpResponse> {
    let stonker: Stonker = repo.get_stonker_by_id(*id).await.expect("Fetching stonkers failed");
    Ok(HttpResponse::Ok().json(stonker))
}
