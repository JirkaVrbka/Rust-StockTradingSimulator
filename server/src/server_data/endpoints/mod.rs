pub mod company_endpoints;
pub mod news_endpoints;
pub mod stock_endpoints;
pub mod stonker_endpoints;

use actix_web::{HttpResponse, Result};
use log::error;
use serde::{Deserialize, Serialize};
use anyhow::Context;

// Used with `.context(<string>)` where string is in format "<code>::::<cause>"
#[derive(Serialize, Deserialize)]
pub struct ApiError {
    pub code: u32,
    pub cause: String,
}

fn api_error_from_string(err: String) -> anyhow::Result<ApiError> {
    let err_segments: Vec<&str> = err.split("::::").collect();
    let code: u32 = err_segments
        .get(0)
        .context("Use correct format for error messages")?
        .trim()
        .parse()
        .context("First segment should be number")?;
    let cause: String = String::from(
        *err_segments
            .get(1)
            .context("Use correct format for error messages")?,
    );

    Ok(ApiError { code, cause })
}

fn handle_api_error(e: anyhow::Error) -> Result<HttpResponse> {
    let api_error = match api_error_from_string(e.to_string()) {
        Ok(err) => err,
        Err(e) => ApiError {code:500, cause: e.to_string()}
    };
    error!("{}", api_error.cause);
    // TODO: handle more error codes (eg. 400, 404, ...)
    match api_error.code {
        500 => Ok(HttpResponse::InternalServerError().json(api_error)),
        400 => Ok(HttpResponse::BadRequest().json(api_error)),
        404 => Ok(HttpResponse::NotFound().json(api_error)),
        _ => Ok(HttpResponse::InternalServerError().json(ApiError {code:500, cause:"Internal Server Error".to_string()}))
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
