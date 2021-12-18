pub mod company_repo;
pub mod connection;
pub mod news_repo;
pub mod stock_repo;
pub mod stonker_repo;

use std::sync::Arc;
use crate::repos::connection::PgPool;

#[derive(std::clone::Clone)]
pub struct Repo {
    pg_pool: Arc<PgPool>
}

impl Repo {
    pub fn new(pg_pool: Arc<PgPool>) -> Self {
        Self { pg_pool }
    }
}
