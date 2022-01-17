use actix_web::{HttpMessage, HttpRequest, web};
use utils::json::StonkerJSON;
use crate::Repo;
use crate::repos::stonker_repo::StonkerRepo;

pub async fn auth(req: &HttpRequest, repo: &web::Data<Repo>) ->  Result<i32, String> {
    let user_id: i32 = match req.cookie("user_id") {
        None => return Err(String::from("user_id missing")),
        Some(uid) => {
            match uid.value().parse::<i32>() {
                Err(_) => return Err(String::from("user_id is not integer")),
                Ok(uid) => uid,
            }}
    };
    let user: StonkerJSON = match repo.get_stonker_by_id(user_id).await {
        Err(_) => return Err(String::from("User with the id not found")),
        Ok(user) => user,
    };
    let passwd: String = match req.cookie("passwd") {
        None => return Err(String::from("passwd is missing")),
        Some(passwd) => passwd.value().to_string()
    };

    match user.password == passwd {
        true => Ok(user_id),
        false => Err(String::from("Wrong passwd")),
    }
}
