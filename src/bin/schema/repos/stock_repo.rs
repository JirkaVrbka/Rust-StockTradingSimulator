use crate::schema::models::stock::NewStock;
use crate::diesel::QueryDsl;
use crate::diesel::RunQueryDsl;
use crate::schema::stock;
use crate::schema::stock::dsl::*;
use crate::schema::{models::stock::Stock as Stock, repos::connection::PgPool};
use async_trait::async_trait;
use std::sync::Arc;

#[async_trait]
pub trait StockRepo {
    async fn get_stocks(&self) -> anyhow::Result<Vec<Stock>>;
    async fn get_stock_by_id(&self, stock_id: i32) -> anyhow::Result<Stock>;
    async fn create_stock(&self, new_stock: NewStock) -> anyhow::Result<Stock>;
}

#[derive(std::clone::Clone)]
pub struct PostgresStockRepo {
    pg_pool: Arc<PgPool>,
}

impl PostgresStockRepo {
    pub fn new(pg_pool: Arc<PgPool>) -> Self {
        Self { pg_pool: pg_pool }
    }
}

#[async_trait]
impl StockRepo for PostgresStockRepo {
    async fn get_stocks(&self) -> anyhow::Result<Vec<Stock>> {
        let connection = self.pg_pool.get().expect("Cannot get connection from pool");
        let results = stock
            .load::<Stock>(&connection)
            .expect("Error loading stocks");

        Ok(results)
    }

    async fn get_stock_by_id(&self, stock_id: i32) -> anyhow::Result<Stock> {
        let connection = self.pg_pool.get().expect("Cannot get connection from pool");
        let result = stock
            .find(stock_id)
            .first(&connection)
            .expect("Error loading stonkers");

        Ok(result)
    }

    async fn create_stock(&self, new_stock: NewStock) -> anyhow::Result<Stock> {
        let connection = self.pg_pool.get().expect("Cannot get connection from pool");

        let result = diesel::insert_into(stock::table)
            .values(&new_stock)
            .get_result(&connection)
            .expect("Error saving new message");

        Ok(result)
    }
}
