use crate::diesel::RunQueryDsl;
use crate::schema::stonker::dsl::*;
use crate::{models::stonker::Stonker, repos::connection::PgPool};
use async_trait::async_trait;
use std::sync::Arc;

#[async_trait]
pub trait StonkerRepo {
    async fn get_stonkers(&self) -> anyhow::Result<Vec<Stonker>>;
}

pub struct PostgresStonkerRepo {
    pg_pool: Arc<PgPool>,
}

impl PostgresStonkerRepo {
    pub fn new(pg_pool: PgPool) -> Self {
        Self {
            pg_pool: Arc::new(pg_pool),
        }
    }
}

#[async_trait]
impl StonkerRepo for PostgresStonkerRepo {
    async fn get_stonkers(&self) -> anyhow::Result<Vec<Stonker>> {
        let connection = self.pg_pool.get().expect("Cannot get connection from pool");
        let results = stonker
            .load::<Stonker>(&connection)
            .expect("Error loading stonkers");
        Ok(results)
    }
}
