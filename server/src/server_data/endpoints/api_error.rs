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

impl ApiError {
    fn from(err: String) -> anyhow::Result<ApiError> {
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

    fn handle_error(e: anyhow::Error) -> Result<HttpResponse> {
        let api_error = match ApiError::from(e.to_string()) {
            Ok(err) => err,
            Err(e) => ApiError {code:500, cause: e.to_string()}
        };
        error!("{}", api_error.cause);
        match api_error.code {
            500 => Ok(HttpResponse::InternalServerError().json(api_error)),
            400 => Ok(HttpResponse::BadRequest().json(api_error)),
            404 => Ok(HttpResponse::NotFound().json(api_error)),
            _ => Ok(HttpResponse::InternalServerError().json(ApiError {code:500, cause:"Internal Server Error".to_string()}))
        }
    }

    pub fn handle<T: Serialize>(result: anyhow::Result<T>) -> Result<HttpResponse> {
        match result {
            Ok(response) => Ok(HttpResponse::Ok().json(response)),
            Err(e) => ApiError::handle_error(e),
        }
    }
}