use actix_web::web;
use actix_web::{get, HttpResponse, Result};
use crate::server_data::endpoints::ApiError;
use crate::server_data::{repos::Repo, models::stonker::NewStonker};
use crate::repos::stonker_repo::StonkerRepo;

#[get("/datagen/news")]
pub async fn datagen_stonker(
    repo: web::Data<Repo>,
) -> Result<HttpResponse> {
    let new_stonker = NewStonker {
        name: "Tom".to_string(),
        balance: 5555,
    };
    let stonker_result = repo.create_stonker(new_stonker).await;
    ApiError::handle(stonker_result)
}
