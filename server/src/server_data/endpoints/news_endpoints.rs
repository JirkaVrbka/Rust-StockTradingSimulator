use actix_web::web;
use actix_web::{get, HttpResponse, Result};

use crate::server_data::api_error::handle_api_result;
use crate::server_data::repos::Repo;
use crate::server_data::repos::news_repo::NewsRepo;

#[get("/news")]
pub async fn get_news(repo: web::Data<Repo>) -> Result<HttpResponse> {
    let news_result = repo.get_news().await;
    handle_api_result(news_result)
}
