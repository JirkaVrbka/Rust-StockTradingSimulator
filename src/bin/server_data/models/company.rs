use crate::schema::company;
use serde::{Deserialize, Serialize};

use super::{stock::StockJSON, stonker::Stonker};

#[derive(Queryable, Clone, Associations, Identifiable, PartialEq)]
#[table_name = "company"]
pub struct Company {
    pub id: i32,
    pub name: String,
    pub performer_id: i32,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct CompanyJSON {
    pub id: i32,
    pub name: String,
    pub performer: Stonker,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct CheapestStocksJSON {
    pub company: CompanyJSON,
    pub stock: StockJSON,
}

#[derive(Insertable, Serialize, Deserialize, Clone)]
#[table_name = "company"]
pub struct NewCompany {
    pub name: String,
    pub performer_id: i32,
}
