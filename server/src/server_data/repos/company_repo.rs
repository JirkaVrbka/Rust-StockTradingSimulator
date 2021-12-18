use crate::diesel::BelongingToDsl;
use crate::diesel::QueryDsl;
use crate::diesel::RunQueryDsl;
use crate::models::company::Company;
use crate::models::stock::Stock;
use crate::schema::company::dsl::*;
use crate::server_data::models::ToJson;
use anyhow::Context;
use async_trait::async_trait;
use utils::json::CompanyJSON;
use utils::json::StockJSON;
use super::Repo;

#[async_trait]
pub trait CompanyRepo {
    async fn get_companies(&self) -> anyhow::Result<Vec<CompanyJSON>>;
    async fn get_company_by_id(&self, company_id: i32) -> anyhow::Result<CompanyJSON>;
    async fn get_company_stocks(&self, company_id: i32) -> anyhow::Result<Vec<StockJSON>>;
}

#[async_trait]
impl CompanyRepo for Repo {
    async fn get_companies(&self) -> anyhow::Result<Vec<CompanyJSON>> {
        let connection = self.connect()?;
        let company_entities = company
            .load::<Company>(&connection)
            .context(format!("404::::Could not get companies"))?;

        company_entities.to_json(&connection)
    }

    async fn get_company_by_id(&self, company_id: i32) -> anyhow::Result<CompanyJSON> {
        let connection = self.connect()?;

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
        let connection = self.connect()?;

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
