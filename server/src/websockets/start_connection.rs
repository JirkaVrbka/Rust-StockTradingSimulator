use crate::server_data::endpoints::ApiError;
use crate::server_data::repos::{Repo, stonker_repo::StonkerRepo};

use super::WsConn;
use super::lobby::Lobby;
use actix::Addr;
use actix_web::{get, web::Data, web::Path, web::Payload, Error, HttpResponse, HttpRequest};
use actix_web_actors::ws;
use uuid::Uuid;

#[get("/chat/{group_id}/{user_id}")]
pub async fn start_connection(
    repo: Data<Repo>,
    req: HttpRequest,
    stream: Payload,
    Path((group_id, user_id)): Path<(Uuid, i32)>,
    srv: Data<Addr<Lobby>>,
) -> Result<HttpResponse, Error> {
    let username = match repo.get_stonker_by_id(user_id).await {
        Ok(stonker) => stonker.name,
        error => return ApiError::handle(error)
    };
    let ws = WsConn::new(
        username,
        group_id,
        srv.get_ref().clone(),
    );
    let resp = ws::start(ws, &req, stream)?;
    Ok(resp)
}