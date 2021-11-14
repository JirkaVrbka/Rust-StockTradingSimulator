use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ApiError {
    pub code: u32,
    pub cause: String,
}