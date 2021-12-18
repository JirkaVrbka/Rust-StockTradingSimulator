use crate::schema::company;
use serde::{Deserialize, Serialize};
use utils::json::CompanyJSON;
use crate::schema::stonker::dsl::stonker;
use crate::server_data::models::stonker::Stonker;
use anyhow::Context;
use crate::diesel::QueryDsl;
use crate::diesel::RunQueryDsl;

use super::{ToJson, Connection};

#[derive(Queryable, Clone, Associations, Identifiable, PartialEq)]
#[table_name = "company"]
pub struct Company {
    pub id: i32,
    pub name: String,
    pub performer_id: i32,
}

impl ToJson<CompanyJSON> for Company {
    fn to_json(&self, connection: &Connection) -> anyhow::Result<CompanyJSON> {
        let performer: &Stonker = &stonker
            .find(self.performer_id)
            .get_result::<Stonker>(connection)
            .context(format!(
                "404::::Cannot find performer {} of company {}",
                self.performer_id, self.id
            ))?;
        Ok(CompanyJSON {
            id: self.id,
            name: self.name.clone(),
            performer: performer.to_json(connection)?,
        })
    }
}

#[derive(Insertable, Serialize, Deserialize, Clone)]
#[table_name = "company"]
pub struct NewCompany {
    pub name: String,
    pub performer_id: i32,
}
