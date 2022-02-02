use std::collections::HashSet;
use crate::diesel::BelongingToDsl;
use crate::models::company::Company;
use crate::models::stock::Stock;
use crate::schema::company::dsl::company;
use crate::schema::command::dsl::command;
use crate::schema::history::dsl::history;
use crate::server_data::models::ToJson;
use async_trait::async_trait;
use utils::json::*;
use crate::models::command::{Command, CommandTypes};
use crate::models::history::History;
use super::Repo;

#[async_trait]
pub trait CompanyRepo {
    async fn get_companies(&self) -> anyhow::Result<Vec<CompanyJSON>>;
    async fn get_company_by_id(&self, company_id: i32) -> anyhow::Result<CompanyDetailJSON>;
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

    async fn get_company_by_id(&self, company_id: i32) -> anyhow::Result<CompanyDetailJSON> {
        let connection = self.connect()?;
        let company_r: Company = Repo::find::<Company, _>(
            &connection,
            company,
            company_id,
            "company"
        ).unwrap();

        let stocks = self.get_company_stocks(company_id).await?;
        let mut stocks_id = HashSet::new();
        for s in stocks{
            stocks_id.insert(s.id);
        }

        let mut commands:Vec<CompanyValueJSON> = Repo::all::<Command, _>(
            &connection,
            command,
            "command"
            ).unwrap().into_iter()
            .filter(|cmd| cmd.company_id == company_id && (cmd.kind == CommandTypes::Sell || cmd.kind == CommandTypes::SellIfHigh) )
            .map(|cmd| CompanyValueJSON {datetime: cmd.created_at, value: cmd.threshold})
            .collect();

        let mut histories:Vec<CompanyValueJSON> = Repo::all::<History, _>(
            &connection,
            history,
            "history"
        ).unwrap().into_iter()
            .filter(|h| stocks_id.contains(&h.stock_id) && h.sold_for > 0)
            .map(|h| CompanyValueJSON {datetime: h.created_at, value: h.sold_for})
            .collect();

        let mut values:Vec<CompanyValueJSON> = vec!();
        values.append(&mut commands);
        values.append(&mut histories);

        values.sort_by(|a, b| a.datetime.cmp(&b.datetime));
        values.dedup_by(|a, b|
            a.datetime.eq(&b.datetime) && a.value.eq(&b.value));

        let company_json = company_r.to_json(&connection).unwrap();

        Ok(CompanyDetailJSON {
            id: company_json.id,
            name: company_json.name.clone(),
            performer: company_json.performer.clone(),
            value_history: values,
        })
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
