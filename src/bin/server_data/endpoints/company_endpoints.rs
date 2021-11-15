use actix_web::web;
use actix_web::{get, HttpResponse, Result};

use crate::repos::company_repo::{CompanyRepo, PostgresCompanyRepo};
use crate::server_data::models::apiError::handle_api_result;

#[get("/companies")]
pub async fn get_companies(repo: web::Data<PostgresCompanyRepo>) -> Result<HttpResponse> {
    let companies_result = repo.get_companies().await;
    handle_api_result(companies_result)
}

#[get("/companies/{id}")]
pub async fn get_company(
    repo: web::Data<PostgresCompanyRepo>,
    id: web::Path<i32>,
) -> Result<HttpResponse> {
    let company_result = repo.get_company_by_id(*id).await;
    handle_api_result(company_result)
}

#[get("/companies/{id}/stocks")]
pub async fn get_company_stocks(
    repo: web::Data<PostgresCompanyRepo>,
    id: web::Path<i32>,
) -> Result<HttpResponse> {
    let company_stocks_result = repo.get_company_stocks(*id).await;
    handle_api_result(company_stocks_result)
}
