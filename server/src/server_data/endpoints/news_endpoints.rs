use actix_web::web;
use actix_web::{get, HttpResponse, Result};

use crate::repos::news_repo::{NewsRepo, PostgresNewsRepo};
use crate::server_data::models::api_error::handle_api_result;

#[get("/news")]
pub async fn get_news(repo: web::Data<PostgresNewsRepo>) -> Result<HttpResponse> {
    let news_result = repo.get_news().await;
    handle_api_result(news_result)
}
