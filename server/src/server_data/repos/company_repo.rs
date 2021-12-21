use crate::diesel::BelongingToDsl;
use crate::models::company::Company;
use crate::models::stock::Stock;
use crate::schema::company::dsl::*;
use crate::server_data::models::ToJson;
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
        let company_entities = Repo::all::<Company, _>(
            &connection,
            company,
            "companies"
        )?;
        company_entities.to_json(&connection)
    }

    async fn get_company_by_id(&self, company_id: i32) -> anyhow::Result<CompanyJSON> {
        let connection = self.connect()?;
        let result = Repo::find::<Company, _>(
            &connection,
            company,
            company_id,
            "company"
        )?;
        result.to_json(&connection)
    }

    async fn get_company_stocks(&self, company_id: i32) -> anyhow::Result<Vec<StockJSON>> {
        let connection = self.connect()?;
        let c = Repo::find::<Company, _>(
            &connection,
            company,
            company_id,
            "company"
        )?;
        let company_stocks = Repo::all::<Stock, _>(
            &connection,
            Stock::belonging_to(&c),
            format!("stock belonging to company with id {}", company_id).as_str()
        )?;
        company_stocks.to_json(&connection)
    }
}
