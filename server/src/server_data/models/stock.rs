use crate::models::company::Company;
use crate::models::stonker::Stonker;
use crate::schema::stock;
use crate::schema::company::dsl::company;
use crate::schema::stonker::dsl::stonker;
use crate::server_data::models::{ToJson, Connection};
use crate::server_data::repos::Repo;
use serde::{Deserialize, Serialize};
use utils::json::StockJSON;

use super::FromJson;

#[derive(Queryable, Clone, Associations, Identifiable, PartialEq)]
#[belongs_to(Company)]
#[belongs_to(Stonker)]
#[table_name = "stock"]
pub struct Stock {
    pub id: i32,
    pub stonker_id: i32,
    pub company_id: i32,
    pub share: i32, // eg.: 50% = 50 * 10000 = 500000
    pub bought_for: i32
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
            share: self.share
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

impl FromJson<StockJSON> for NewStock {
    fn from_json(json: &StockJSON) -> Self {
        NewStock {
            stonker_id: json.owner.id,
            company_id: json.issued_by.id,
            share: json.share,
            bought_for: json.bought_for
        }
    }
}