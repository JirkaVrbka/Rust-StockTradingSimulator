use crate::diesel::QueryDsl;
use crate::diesel::RunQueryDsl;
use crate::models::stock::NewStock;
use crate::schema::stock;
use crate::schema::stock::dsl::*;
use crate::server_data::models::ToJson;
use crate::models::stock::Stock;
use anyhow::Context;
use async_trait::async_trait;
use utils::json::StockJSON;
use super::Repo;

#[async_trait]
pub trait StockRepo {
    async fn get_stocks(&self) -> anyhow::Result<Vec<StockJSON>>;
    async fn get_stock_by_id(&self, stock_id: i32) -> anyhow::Result<StockJSON>;
    async fn create_stock(&self, new_stock: NewStock) -> anyhow::Result<StockJSON>;
}

#[async_trait]
impl StockRepo for Repo {
    async fn get_stocks(&self) -> anyhow::Result<Vec<StockJSON>> {
        let connection = self.connect()?;

        let stock_entities = &stock
            .load::<Stock>(&connection)
            .context("404::::Could not find stocks")?;
        stock_entities.to_json(&connection)
    }

    async fn get_stock_by_id(&self, stock_id: i32) -> anyhow::Result<StockJSON> {
        let connection = self.connect()?;

        let result: &Stock = &stock
            .find(stock_id)
            .first(&connection)
            .context(format!("404::::Could not find stock with id {}", stock_id))?;
        result.to_json(&connection)
    }

    async fn create_stock(&self, new_stock: NewStock) -> anyhow::Result<StockJSON> {
        let connection = self.connect()?;

        let result: &Stock = &diesel::insert_into(stock::table)
            .values(&new_stock)
            .get_result(&connection)
            .context("500::::Could not save stock")?;

        result.to_json(&connection)
    }
}
