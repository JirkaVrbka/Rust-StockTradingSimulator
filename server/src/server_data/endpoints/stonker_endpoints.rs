use crate::models::stonker::NewStonker;
use crate::repos::stonker_repo::StonkerRepo;
use crate::server_data::endpoints::ApiError;
use crate::server_data::repos::Repo;
use actix_web::{http, HttpRequest, web};
use actix_web::{get, post, HttpResponse, Result};
use actix_web::web::Bytes;
use utils::json::{StonkerCredentials, StonkerJSON, AuthJSON};
use crate::endpoints::auth::auth;

#[get("/stonkers")]
pub async fn get_stonkers(repo: web::Data<Repo>) -> Result<HttpResponse> {
    let stonkers_result = repo.get_stonkers().await;
    ApiError::handle(stonkers_result)
}

#[get("/stonkers/{id}")]
pub async fn get_stonker(
    repo: web::Data<Repo>,
    id: web::Path<i32>,
) -> Result<HttpResponse> {
    let stonker_result = repo.get_stonker_by_id(*id).await;
    ApiError::handle(stonker_result)
}

#[get("/stonkers/{id}/overview")]
pub async fn get_stonker_overview(
    repo: web::Data<Repo>,
    id: web::Path<i32>,
) -> Result<HttpResponse> {
    let stonker_result = repo.get_stonker_overview(*id).await;
    ApiError::handle(stonker_result)
}

#[post("/stonkers")]
pub async fn create_stonker(
    repo: web::Data<Repo>,
    stonker_data: web::Json<NewStonker>,
) -> Result<HttpResponse> {
    let new_stonker = NewStonker {
        name: stonker_data.name.clone(),
        balance: stonker_data.balance,
        password: stonker_data.password.clone(),
    };
    let stonker_result = repo.create_stonker(new_stonker).await;
    ApiError::handle(stonker_result)
}

#[post("/login")]
pub async fn login(
    repo: web::Data<Repo>,
    bytes: Bytes,
) -> Result<HttpResponse> {
    let body = String::from_utf8(bytes.to_vec()).unwrap();
    let to_json = |a: AuthJSON| serde_json::to_string(&a).unwrap();

    // body is loaded, now we can deserialize serde-json
    let stonker_data = serde_json::from_str::<StonkerCredentials>(&body)?;


    let user: StonkerJSON = match repo.get_stonker_by_name(&stonker_data.name).await {
        Err(_) => return Ok(HttpResponse::Unauthorized().body(to_json(AuthJSON::NoSuchUser))),
        Ok(user) => user,
    };

    if user.password != stonker_data.password {
        return Ok(HttpResponse::Unauthorized().body(to_json(AuthJSON::WrongPassword)));
    }

    return Ok(HttpResponse::Ok()
        .cookie(
            http::Cookie::build("user_id", user.id.to_string())
                .secure(false)
                .http_only(false)
                .finish())
        .cookie(
            http::Cookie::build("passwd", user.password.clone())
                .secure(false)
                .http_only(false)
                .finish())
        .body(to_json(AuthJSON::Ok))
    )
}

#[get("/l/stonkers/stocks")]
pub async fn get_curr_stonker_stocks(
    repo: web::Data<Repo>,
    req: HttpRequest,
) -> Result<HttpResponse> {
    let user_id = match auth(&req, &repo).await {
        Ok(user_id) => user_id,
        Err(e) => return Ok(HttpResponse::Unauthorized().body(e)),
    };
    let stonker_stocks = repo.get_stonker_stocks(user_id).await;
    ApiError::handle(stonker_stocks)
}

#[get("/stonkers/{id}/stocks")]
pub async fn get_stonker_stocks(
    repo: web::Data<Repo>,
    id: web::Path<i32>,
) -> Result<HttpResponse> {
    let stonker_stocks = repo.get_stonker_stocks(*id).await;
    ApiError::handle(stonker_stocks)
}