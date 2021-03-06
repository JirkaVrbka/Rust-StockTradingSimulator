use actix_web::web;
use actix_web::{get, HttpResponse, Result};

use crate::server_data::endpoints::ApiError;
use crate::server_data::repos::Repo;
use crate::server_data::repos::company_repo::CompanyRepo;

#[get("/companies")]
pub async fn get_companies(repo: web::Data<Repo>) -> Result<HttpResponse> {
    let companies_result = repo.get_companies().await;
    ApiError::handle(companies_result)
}

#[get("/companies/{id}")]
pub async fn get_company(
    repo: web::Data<Repo>,
    id: web::Path<i32>,
) -> Result<HttpResponse> {
    let company_result = repo.get_company_by_id(*id).await;
    ApiError::handle(company_result)
}

#[get("/companies/{id}/stocks")]
pub async fn get_company_stocks(
    repo: web::Data<Repo>,
    id: web::Path<i32>,
) -> Result<HttpResponse> {
    let company_stocks_result = repo.get_company_stocks(*id).await;
    ApiError::handle(company_stocks_result)
}
