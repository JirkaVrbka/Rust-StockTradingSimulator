use crate::diesel::BelongingToDsl;
use crate::diesel::QueryDsl;
use crate::diesel::RunQueryDsl;
use crate::models::company::Company;
use crate::models::stock::Stock;
use crate::repos::connection::PgPool;
use crate::schema::company::dsl::*;
use crate::schema::stonker::dsl::stonker;
use crate::server_data::models::company::CompanyJSON;
use crate::server_data::models::stock::StockJSON;
use crate::server_data::models::stonker::Stonker;
use crate::server_data::repos::stock_repo::stocks_to_json;
use anyhow::Context;
use async_trait::async_trait;
use diesel::r2d2::ConnectionManager;
use diesel::r2d2::PooledConnection;
use diesel::PgConnection;
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

pub fn company_to_json(
    connection: &PooledConnection<ConnectionManager<PgConnection>>,
    entity: &Company,
) -> anyhow::Result<CompanyJSON> {
    let performer: Stonker = stonker
        .find(entity.performer_id)
        .get_result::<Stonker>(connection)
        .context(format!(
            "404::::Cannot find perfomer {} of company {}",
            entity.performer_id, entity.id
        ))?;
    Ok(CompanyJSON {
        id: entity.id,
        name: entity.name.clone(),
        performer,
    })
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
            .map(|entity| company_to_json(&connection, entity))
            .collect();

        Ok(company_jsons?)
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

        Ok(company_to_json(&connection, &result)?)
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

        let company_stocks: Vec<Stock> = Stock::belonging_to(&c)
            .load::<Stock>(&connection)
            .context(format!(
                "404::::Could not find stock belonging to company with id {}",
                company_id
            ))?;

        Ok(stocks_to_json(&connection, &company_stocks)?)
    }
}
