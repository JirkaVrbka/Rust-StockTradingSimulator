use crate::models::news::News;
use crate::repos::connection::PgPool;
use crate::schema::news::dsl::*;
use crate::server_data::models::ToJson;
use crate::server_data::models::company::Company;
use crate::schema::company::dsl::company;
use crate::diesel::QueryDsl;
use anyhow::Context;
use async_trait::async_trait;
use diesel::{RunQueryDsl, PgConnection};
use diesel::r2d2::{PooledConnection, ConnectionManager};
use utils::json::{NewsJSON, CompanyJSON};
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

fn new_to_json(
    connection: &PooledConnection<ConnectionManager<PgConnection>>,
    entity: &News,
) -> anyhow::Result<NewsJSON> {
    let affected: &Company = &company
        .find(entity.company_id)
        .get_result::<Company>(connection)
        .context(format!(
            "404::::Cannot find company {} of news {}",
            entity.company_id, entity.id
        ))?;
    Ok(NewsJSON {
        id: entity.id,
        title: entity.title.clone(),
        description: entity.description.clone(),
        author: entity.author.clone(),
        created_at: entity.created_at,
        effect: entity.kind.to_json(connection)?,
        company: affected.to_json(connection)?
    })
}

pub fn news_to_json(
    connection: &PooledConnection<ConnectionManager<PgConnection>>,
    entities: &Vec<News>,
) -> anyhow::Result<Vec<NewsJSON>> {
    entities
        .iter()
        .map(|entity| new_to_json(&connection, entity))
        .collect()
}


#[async_trait]
impl NewsRepo for PostgresNewsRepo {
    async fn get_news(&self) -> anyhow::Result<Vec<NewsJSON>> {
        let connection = self
            .pg_pool
            .get()
            .context("500::::Cannot get connection from pool")?;

        let news_entities: Vec<News> = news
            .load::<News>(&connection)
            .context(format!("404::::Could not get news"))?;

        Ok(news_to_json(&connection, &news_entities)?)
    }
}
