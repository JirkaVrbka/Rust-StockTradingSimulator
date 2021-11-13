use actix_web::HttpResponse;
use actix_web::Responder;
use actix_web::web;
use crate::PostgresStonkerRepo;
use crate::Stonker;
use crate::repos::stonker_repo::StonkerRepo;

pub async fn get_stonkers(repo: web::Data<PostgresStonkerRepo>) -> impl Responder {
    let stonkers: Vec<Stonker> = repo
        .get_stonkers()
        .await
        .expect("Fetching stonkers failed");

    let stonker = stonkers.get(0).expect("stonker error");

     HttpResponse::Ok().body(stonker.name.clone())
}