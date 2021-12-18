use crate::models::company::Company;
use crate::models::stonker::Stonker;
use crate::schema::stock;
use crate::diesel::QueryDsl;
use crate::diesel::RunQueryDsl;
use crate::schema::company::dsl::company;
use crate::schema::stonker::dsl::stonker;
use crate::server_data::models::{ToJson, Connection};
use anyhow::Context;
use serde::{Deserialize, Serialize};
use utils::json::StockJSON;

#[derive(Queryable, Clone, Associations, Identifiable, PartialEq)]
#[belongs_to(Company)]
#[belongs_to(Stonker)]
#[table_name = "stock"]
pub struct Stock {
    pub id: i32,
    pub stonker_id: i32,
    pub company_id: i32,
    pub share: i32, // eg.: 50% = 50 * 10000 = 500000
    pub bought_for: i32,
    pub sold_for: Option<i32>,
}

impl ToJson<StockJSON> for Stock {
    fn to_json(&self, connection: &Connection) -> anyhow::Result<StockJSON> {
        let c: &Company = &company
            .find(self.company_id)
            .get_result::<Company>(connection)
            .context(format!(
                "404::::Cannot find company {} of stock {}",
                self.company_id, self.id
            ))?;
        let owner: &Stonker = &stonker
            .find(self.stonker_id)
            .get_result::<Stonker>(connection)
            .context(format!(
                "404::::Cannot find owner {} of stock {}",
                self.stonker_id, self.id
            ))?;
        Ok(StockJSON {
            id: self.id,
            owner: owner.to_json(connection)?,
            issued_by: c.to_json(connection)?,
            bought_for: self.bought_for,
            share: self.share,
            sold_for: self.sold_for
        })
    }
}

impl ToJson<Vec<StockJSON>> for Vec<Stock> {
    fn to_json(
        &self,
        connection: &Connection,
    ) -> anyhow::Result<Vec<StockJSON>> {
        Ok(self
            .iter()
            .filter_map(|entity| entity.to_json(connection).ok())
            .collect())
    }
}

#[derive(Insertable, Serialize, Deserialize, Clone)]
#[table_name = "stock"]
pub struct NewStock {
    pub stonker_id: i32,
    pub company_id: i32,
    pub share: i32, // eg.: 50% = 50 * 10000 = 500000
    pub bought_for: i32
}
