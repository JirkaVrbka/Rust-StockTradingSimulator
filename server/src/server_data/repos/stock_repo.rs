use crate::diesel::QueryDsl;
use crate::diesel::RunQueryDsl;
use crate::models::company::Company;
use crate::models::stock::NewStock;
use crate::models::stonker::Stonker;
use crate::schema::company::dsl::company;
use crate::schema::stock;
use crate::schema::stock::dsl::*;
use crate::schema::stonker::dsl::stonker;
use crate::server_data::models::ToJson;
use crate::{models::stock::Stock, repos::connection::PgPool};
use anyhow::Context;
use async_trait::async_trait;
use diesel::r2d2::ConnectionManager;
use diesel::r2d2::PooledConnection;
use diesel::PgConnection;
use utils::json::CompanyJSON;
use utils::json::StockJSON;
use utils::json::StonkerJSON;
use std::sync::Arc;

use super::stonker_repo::stonker_to_json;

#[async_trait]
pub trait StockRepo {
    async fn get_stocks(&self) -> anyhow::Result<Vec<StockJSON>>;
    async fn get_stock_by_id(&self, stock_id: i32) -> anyhow::Result<StockJSON>;
    async fn create_stock(&self, new_stock: NewStock) -> anyhow::Result<StockJSON>;
}

#[derive(std::clone::Clone)]
pub struct PostgresStockRepo {
    pg_pool: Arc<PgPool>,
}

impl PostgresStockRepo {
    pub fn new(pg_pool: Arc<PgPool>) -> Self {
        Self { pg_pool }
    }
}

pub fn stock_to_json(
    connection: &PooledConnection<ConnectionManager<PgConnection>>,
    entity: &Stock,
) -> anyhow::Result<StockJSON> {
    let c: Company = company
        .find(entity.company_id)
        .get_result::<Company>(connection)
        .context(format!(
            "404::::Cannot find company {} of stock {}",
            entity.company_id, entity.id
        ))?;
    let issued_by: CompanyJSON = c.to_json(connection)?;
    let owner: StonkerJSON = stonker_to_json(connection, &stonker
        .find(entity.stonker_id)
        .get_result::<Stonker>(connection)
        .context(format!(
            "404::::Cannot find owner {} of stock {}",
            entity.stonker_id, entity.id
        ))?)?;
    Ok(StockJSON {
        id: entity.id,
        owner,
        issued_by,
        bought_for: entity.bought_for,
        share: entity.share,
        sold_for: entity.sold_for
    })
}

pub fn stocks_to_json(
    connection: &PooledConnection<ConnectionManager<PgConnection>>,
    entities: &Vec<Stock>,
) -> anyhow::Result<Vec<StockJSON>> {
    entities
        .iter()
        .map(|entity| stock_to_json(&connection, entity))
        .collect()
}

#[async_trait]
impl StockRepo for PostgresStockRepo {
    async fn get_stocks(&self) -> anyhow::Result<Vec<StockJSON>> {
        let connection = self
            .pg_pool
            .get()
            .context("500::::Cannot get connection from pool")?;
        let stock_entities = stock
            .load::<Stock>(&connection)
            .context("404::::Could not find stocks")?;

        Ok(stocks_to_json(&connection, &stock_entities)?)
    }

    async fn get_stock_by_id(&self, stock_id: i32) -> anyhow::Result<StockJSON> {
        let connection = self
            .pg_pool
            .get()
            .context("500::::Cannot get connection from pool")?;
        let result: Stock = stock
            .find(stock_id)
            .first(&connection)
            .context(format!("404::::Could not find stock with id {}", stock_id))?;

        Ok(stock_to_json(&connection, &result)?)
    }

    async fn create_stock(&self, new_stock: NewStock) -> anyhow::Result<StockJSON> {
        let connection = self
            .pg_pool
            .get()
            .context("500::::Cannot get connection from pool")?;

        let result: Stock = diesel::insert_into(stock::table)
            .values(&new_stock)
            .get_result(&connection)
            .context("500::::Could not save stock")?;

        Ok(stock_to_json(&connection, &result)?)
    }
}
