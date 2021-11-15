use actix_web::{HttpResponse, Result};
use serde::{Deserialize, Serialize};

// Used with `.context(<string>)` where string is in format "<code>::::<cause>"
#[derive(Serialize, Deserialize)]
pub struct ApiError {
    pub code: u32,
    pub cause: String,
}

fn api_error_from_string(err: String) -> ApiError {
    let err_segments: Vec<&str> = err.split("::::").collect();
    let code: u32 = err_segments
        .get(0)
        .expect("Use correct format for error messages")
        .trim()
        .parse()
        .expect("First segment should be number");
    let cause: String = String::from(
        *err_segments
            .get(1)
            .expect("Use correct format for error messages"),
    );
    ApiError { code, cause }
}

fn handle_api_error(e: anyhow::Error) -> Result<HttpResponse> {
    let api_error = api_error_from_string(e.to_string());
    // TODO: handle more error codes (eg. 400, 404, ...)
    match api_error.code {
        500 => Ok(HttpResponse::InternalServerError().json(api_error)),
        400 => Ok(HttpResponse::BadRequest().json(api_error)),
        404 => Ok(HttpResponse::NotFound().json(api_error)),
        _ => {
            panic!("Error not defined yet");
        }
    }
}

pub fn handle_api_result<T>(result: anyhow::Result<T>) -> Result<HttpResponse>
where
    T: Serialize,
{
    match result {
        Ok(response) => Ok(HttpResponse::Ok().json(response)),
        Err(e) => handle_api_error(e),
    }
}
