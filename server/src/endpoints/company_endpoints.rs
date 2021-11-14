use actix_web::web;
use actix_web::{get, HttpResponse, Result};

use crate::models::company::Company;

use crate::repos::company_repo::{CompanyRepo, PostgresCompanyRepo};

#[get("/companies")]
pub async fn get_companies(repo: web::Data<PostgresCompanyRepo>) -> Result<HttpResponse> {
    let companies: Vec<Company> = repo
        .get_companies()
        .await
        .expect("Fetching companies failed");
    Ok(HttpResponse::Ok().json(companies))
}

#[get("/companies/{id}")]
pub async fn get_company(
    repo: web::Data<PostgresCompanyRepo>,
    id: web::Path<i32>,
) -> Result<HttpResponse> {
    let company: Company = repo
        .get_company_by_id(*id)
        .await
        .expect("Fetching company failed");
    Ok(HttpResponse::Ok().json(company))
}
