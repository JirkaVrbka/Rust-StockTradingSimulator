use crate::schema::company;
use crate::server_data::repos::Repo;
use serde::{Deserialize, Serialize};
use utils::json::CompanyJSON;
use crate::schema::stonker::dsl::stonker;
use crate::server_data::models::stonker::Stonker;

use super::{ConvertJson, Connection};

#[derive(Queryable, Clone, Associations, Identifiable, PartialEq)]
#[table_name = "company"]
pub struct Company {
    pub id: i32,
    pub name: String,
    pub performer_id: i32,
}

impl ConvertJson<CompanyJSON> for Company {
    fn to_json(&self, connection: &Connection) -> anyhow::Result<CompanyJSON> {
        let performer = Repo::find::<Stonker, _>(
            &connection,
            stonker,
            self.performer_id,
            format!("stonker from {}", self.id).as_str()
        )?;
        Ok(CompanyJSON {
            id: self.id,
            name: self.name.clone(),
            performer: performer.to_json(connection)?,
        })
    }
    fn from_json(json: &CompanyJSON) -> Self {
        Company {
            id: json.id,
            name: json.name.clone(),
            performer_id: json.performer.id,
        }
    }
}

#[derive(Insertable, Serialize, Deserialize, Clone)]
#[table_name = "company"]
pub struct NewCompany {
    pub name: String,
    pub performer_id: i32,
}
