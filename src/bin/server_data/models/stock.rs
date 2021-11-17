use crate::models::company::Company;
use crate::models::stonker::Stonker;
use crate::schema::stock;
use serde::{Deserialize, Serialize};

use super::company::CompanyJSON;

#[derive(Queryable, Clone, Associations, Identifiable, PartialEq)]
#[belongs_to(Company)]
#[belongs_to(Stonker)]
#[table_name = "stock"]
pub struct Stock {
    pub id: i32,
    pub stonker_id: i32,
    pub company_id: i32,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct StockJSON {
    pub id: i32,
    pub owner: Stonker,
    pub issued_by: CompanyJSON,
}

#[derive(Insertable, Serialize, Deserialize, Clone)]
#[table_name = "stock"]
pub struct NewStock {
    pub stonker_id: i32,
    pub company_id: i32,
}
