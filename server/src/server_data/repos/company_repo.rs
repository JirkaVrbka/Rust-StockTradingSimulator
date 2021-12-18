use crate::diesel::BelongingToDsl;
use crate::diesel::QueryDsl;
use crate::diesel::RunQueryDsl;
use crate::models::company::Company;
use crate::models::stock::Stock;
use crate::repos::connection::PgPool;
use crate::schema::company::dsl::*;
use crate::server_data::models::ToJson;
use anyhow::Context;
use async_trait::async_trait;
use utils::json::CompanyJSON;
use utils::json::StockJSON;
use std::sync::Arc;

#[async_trait]
pub trait CompanyRepo {
    async fn get_companies(&self) -> anyhow::Result<Vec<CompanyJSON>>;
    async fn get_company_by_id(&self, company_id: i32) -> anyhow::Result<CompanyJSON>;
    async fn get_company_stocks(&self, company_id: i32) -> anyhow::Result<Vec<StockJSON>>;
}

#[derive(std::clone::Clone)]
pub struct PostgresCompanyRepo {
    pg_pool: Arc<PgPool>,
}

impl PostgresCompanyRepo {
    pub fn new(pg_pool: Arc<PgPool>) -> Self {
        Self { pg_pool: pg_pool }
    }
}

#[async_trait]
impl CompanyRepo for PostgresCompanyRepo {
    async fn get_companies(&self) -> anyhow::Result<Vec<CompanyJSON>> {
        let connection = self
            .pg_pool
            .get()
            .context("500::::Cannot get connection from pool")?;
        let company_entities = company
            .load::<Company>(&connection)
            .context(format!("404::::Could not get companies"))?;

        let company_jsons: anyhow::Result<Vec<CompanyJSON>> = company_entities
            .iter()
            .map(|entity| entity.to_json(&connection))
            .collect();

        company_jsons
    }

    async fn get_company_by_id(&self, company_id: i32) -> anyhow::Result<CompanyJSON> {
        let connection = self
            .pg_pool
            .get()
            .context("500::::Cannot get connection from pool")?;

        let result: Company = company
            .find(company_id)
            .first(&connection)
            .context(format!(
                "404::::Could not find company with id {}",
                company_id
            ))?;

        result.to_json(&connection)
    }

    async fn get_company_stocks(&self, company_id: i32) -> anyhow::Result<Vec<StockJSON>> {
        let connection = self
            .pg_pool
            .get()
            .context("500::::::Cannot not get connection from pool")?;
        let c: Company = company
            .find(company_id)
            .first(&connection)
            .context(format!(
                "404::::Could not find company with id {}",
                company_id
            ))?;

        let company_stocks: &Vec<Stock> = &Stock::belonging_to(&c)
            .load::<Stock>(&connection)
            .context(format!(
                "404::::Could not find stock belonging to company with id {}",
                company_id
            ))?;

        company_stocks.to_json(&connection)
    }
}
