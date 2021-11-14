use crate::diesel::QueryDsl;
use crate::diesel::RunQueryDsl;
use crate::models::stock::Stock;
use crate::schema::company::dsl::*;
use crate::models::company::Company;
use crate::repos::connection::PgPool;
use async_trait::async_trait;
use std::sync::Arc;

#[async_trait]
pub trait CompanyRepo {
    async fn get_companies(&self) -> anyhow::Result<Vec<Company>>;
    async fn get_company_by_id(&self, company_id: i32) -> anyhow::Result<Company>;
    // async fn get_company_stocks(&self, company_id: i32) -> anyhow::Result<Vec<Stock>>;
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
    async fn get_companies(&self) -> anyhow::Result<Vec<Company>> {
        let connection = self.pg_pool.get().expect("Cannot get connection from pool");
        let results = company
            .load::<Company>(&connection)
            .expect("Error loading companies");

        Ok(results)
    }

    async fn get_company_by_id(&self, company_id: i32) -> anyhow::Result<Company> {
        let connection = self.pg_pool.get().expect("Cannot get connection from pool");
        let result = company
            .find(company_id)
            .first(&connection)
            .expect("Error loading company");

        Ok(result)
    }

    // async fn get_company_stocks(&self, company_id: i32) -> anyhow::Result<Vec<Stock>> {
    //     let connection = self.pg_pool.get().expect("Cannot get connection from pool");
    //     let result = company
    //         .find(company_id)
    //         .first(&connection)
    //         .expect("Error loading company");
    //     let company_stocks = Stock::belonging_to(&company).load::<Company>(&connection).expect("Error loading company stocks");

    //     Ok(company_stocks)
    // }
}
