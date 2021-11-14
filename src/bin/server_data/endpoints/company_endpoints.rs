use actix_web::web;
use actix_web::{get, HttpResponse, Result};

use crate::models::company::Company;

use crate::models::stock::Stock;
use crate::models::apiError::ApiError;
use crate::repos::company_repo::{CompanyRepo, PostgresCompanyRepo};


#[get("/companies")]
pub async fn get_companies(repo: web::Data<PostgresCompanyRepo>) -> Result<HttpResponse> {
    let companies_result = repo
        .get_companies()
        .await;

    match companies_result {
        Ok(companies) => Ok(HttpResponse::Ok().json(companies)),
        Err(e) =>  Ok(HttpResponse::InternalServerError().json(ApiError {code: 500, cause: e.to_string()})),
    }
}

#[get("/companies/{id}")]
pub async fn get_company(
    repo: web::Data<PostgresCompanyRepo>,
    id: web::Path<i32>,
) -> Result<HttpResponse> {
    let company_result = repo
        .get_company_by_id(*id)
        .await;

    match company_result {
        Ok(company) => Ok(HttpResponse::Ok().json(company)),
        Err(e) =>  Ok(HttpResponse::InternalServerError().json(ApiError {code: 500, cause: e.to_string()})),
    }
}

#[get("/companies/{id}/stocks")]
pub async fn get_company_stocks(
    repo: web::Data<PostgresCompanyRepo>,
    id: web::Path<i32>,
) -> Result<HttpResponse> {
    let company_stocks: Vec<Stock> = repo
        .get_company_stocks(*id)
        .await
        .expect("Fetching company failed");
    Ok(HttpResponse::Ok().json(company_stocks))
}
