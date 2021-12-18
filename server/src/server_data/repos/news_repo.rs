use crate::models::news::News;
use crate::repos::connection::PgPool;
use crate::schema::news::dsl::*;
use crate::server_data::models::ToJson;
use anyhow::Context;
use async_trait::async_trait;
use diesel::RunQueryDsl;
use utils::json::NewsJSON;
use std::sync::Arc;

#[async_trait]
pub trait NewsRepo {
    async fn get_news(&self) -> anyhow::Result<Vec<NewsJSON>>;
}

#[derive(std::clone::Clone)]
pub struct PostgresNewsRepo {
    pg_pool: Arc<PgPool>,
}

impl PostgresNewsRepo {
    pub fn new(pg_pool: Arc<PgPool>) -> Self {
        Self { pg_pool: pg_pool }
    }
}

#[async_trait]
impl NewsRepo for PostgresNewsRepo {
    async fn get_news(&self) -> anyhow::Result<Vec<NewsJSON>> {
        let connection = self
            .pg_pool
            .get()
            .context("500::::Cannot get connection from pool")?;

        let news_entities: &Vec<News> = &news
            .load::<News>(&connection)
            .context(format!("404::::Could not get news"))?;

        news_entities.to_json(&connection)
    }
}
