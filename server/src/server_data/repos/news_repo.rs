use crate::models::news::News;
use crate::schema::news::dsl::*;
use crate::server_data::models::ConvertJson;
use async_trait::async_trait;
use utils::json::NewsJSON;
use super::Repo;

#[async_trait]
pub trait NewsRepo {
    async fn get_news(&self) -> anyhow::Result<Vec<NewsJSON>>;
}

#[async_trait]
impl NewsRepo for Repo {
    async fn get_news(&self) -> anyhow::Result<Vec<NewsJSON>> {
        let connection = self.connect()?;
        let news_entities = Repo::all::<News, _>(
            &connection,
            news,
            "news"
        )?;
        news_entities.to_json(&connection)
    }
}
