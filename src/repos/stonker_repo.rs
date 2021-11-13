use crate::models::stonker::NewStonker;
use crate::diesel::QueryDsl;
use crate::diesel::RunQueryDsl;
use crate::schema::stonker;
use crate::schema::stonker::dsl::*;
use crate::{models::stonker::Stonker, repos::connection::PgPool};
use async_trait::async_trait;
use std::sync::Arc;

#[async_trait]
pub trait StonkerRepo {
    async fn get_stonkers(&self) -> anyhow::Result<Vec<Stonker>>;
    async fn get_stonker_by_id(&self, stonker_id: i32) -> anyhow::Result<Stonker>;
    async fn create_stonker(&self, new_stonker: NewStonker) -> anyhow::Result<Stonker>;
}

#[derive(std::clone::Clone)]
pub struct PostgresStonkerRepo {
    pg_pool: Arc<PgPool>,
}

impl PostgresStonkerRepo {
    pub fn new(pg_pool: Arc<PgPool>) -> Self {
        Self { pg_pool: pg_pool }
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

    async fn get_stonker_by_id(&self, stonker_id: i32) -> anyhow::Result<Stonker> {
        let connection = self.pg_pool.get().expect("Cannot get connection from pool");
        let result = stonker
            .find(stonker_id)
            .first(&connection)
            .expect("Error loading stonkers");

        Ok(result)
    }

    async fn create_stonker(&self, new_stonker: NewStonker) -> anyhow::Result<Stonker> {
        let connection = self.pg_pool.get().expect("Cannot get connection from pool");

        let result = diesel::insert_into(stonker::table)
            .values(&new_stonker)
            .get_result(&connection)
            .expect("Error saving new message");

        Ok(result)
    }
}
