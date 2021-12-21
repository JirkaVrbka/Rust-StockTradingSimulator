use crate::diesel::RunQueryDsl;
use crate::models::stock::NewStock;
use crate::schema::stock;
use crate::schema::stock::dsl::*;
use crate::server_data::models::ConvertJson;
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
        let stock_entities = Repo::all::<Stock, _>(
            &connection,
            stock,
            "stocks"
        )?;
        stock_entities.to_json(&connection)
    }

    async fn get_stock_by_id(&self, stock_id: i32) -> anyhow::Result<StockJSON> {
        let connection = self.connect()?;
        let result = Repo::find::<Stock, _>(
            &connection,
            stock,
            stock_id,
            "stock"
        )?;
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
