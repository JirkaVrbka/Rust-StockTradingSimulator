use crate::models::company::Company;
use crate::models::stonker::Stonker;
use crate::schema::stock;
use crate::schema::company::dsl::company;
use crate::schema::stonker::dsl::stonker;
use crate::server_data::models::{ToJson, Connection};
use crate::server_data::repos::Repo;
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
        let c = Repo::find::<Company, _>(
            &connection,
            company,
            self.company_id,
            format!("company of stock {}", self.id).as_str()
        )?;
        let owner = Repo::find::<Stonker, _>(
            &connection,
            stonker,
            self.company_id,
            format!("owner of stock {}", self.id).as_str()
        )?;
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

#[derive(Insertable, Serialize, Deserialize, Clone)]
#[table_name = "stock"]
pub struct NewStock {
    pub stonker_id: i32,
    pub company_id: i32,
    pub share: i32, // eg.: 50% = 50 * 10000 = 500000
    pub bought_for: i32
}
