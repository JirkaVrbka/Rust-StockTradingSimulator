use crate::diesel::BelongingToDsl;
use crate::diesel::ExpressionMethods;
use crate::diesel::QueryDsl;
use crate::diesel::RunQueryDsl;
use crate::models::stock::Stock;
use crate::models::stonker::NewStonker;
use crate::schema::command::dsl::command;
use crate::schema::command::stonker_id;
use crate::schema::company::dsl::*;
use crate::schema::stonker;
use crate::schema::stonker::dsl::*;
use crate::server_data::models::command::Command;
use crate::server_data::models::command::CommandTypes;
use crate::server_data::models::company::Company;
use crate::server_data::models::stock::StockJSON;
use crate::server_data::models::stonker::PortfolioJSON;
use crate::server_data::models::stonker::StonkerHistoryJSON;
use crate::server_data::models::stonker::StonkerOverviewJSON;
use crate::server_data::models::stonker::UsageJSON;
use crate::server_data::repos::stock_repo::stocks_to_json;
use crate::{models::stonker::Stonker, repos::connection::PgPool};
use anyhow::Context;
use async_trait::async_trait;
use chrono::Datelike;
use diesel::dsl::min;
use std::sync::Arc;
use crate::schema::command::company_id;
use crate::schema::command::kind;
use crate::schema::command::threshold;

#[async_trait]
pub trait StonkerRepo {
    async fn get_stonkers(&self) -> anyhow::Result<Vec<Stonker>>;
    async fn get_stonker_overview(&self, s_id: i32) -> anyhow::Result<StonkerOverviewJSON>;
    async fn get_stonker_by_id(&self, s_id: i32) -> anyhow::Result<Stonker>;
    async fn create_stonker(&self, new_stonker: NewStonker) -> anyhow::Result<Stonker>;
    async fn get_stonker_stocks(&self, s_id: i32) -> anyhow::Result<Vec<StockJSON>>;
}

#[derive(std::clone::Clone)]
pub struct PostgresStonkerRepo {
    pg_pool: Arc<PgPool>,
}

impl PostgresStonkerRepo {
    pub fn new(pg_pool: Arc<PgPool>) -> Self {
        Self { pg_pool: pg_pool }
    }
}

#[async_trait]
impl StonkerRepo for PostgresStonkerRepo {
    async fn get_stonkers(&self) -> anyhow::Result<Vec<Stonker>> {
        let connection = self
            .pg_pool
            .get()
            .context("500::::Cannot get connection from pool")?;
        let results = stonker
            .load::<Stonker>(&connection)
            .context("404::::Could not find stonkers")?;

        Ok(results)
    }

    async fn get_stonker_by_id(&self, s_id: i32) -> anyhow::Result<Stonker> {
        let connection = self
            .pg_pool
            .get()
            .context("500::::Cannot get connection from pool")?;
        let result = stonker
            .find(s_id)
            .first(&connection)
            .context(format!("404::::Could not find stonker with id {}", s_id))?;

        Ok(result)
    }

    async fn get_stonker_overview(&self, s_id: i32) -> anyhow::Result<StonkerOverviewJSON> {
        let connection = self
            .pg_pool
            .get()
            .context("500::::Cannot get connection from pool")?;

        let stonker_entity: Stonker = stonker
            .find(s_id)
            .first(&connection)
            .context(format!("404::::Could not find stonker with id {}", s_id))?;

        let usage = UsageJSON {
            free: stonker_entity.balance,
            invested: stonker_entity.invested_balance,
            blocked: stonker_entity.blocked_balance,
        };

        // TODO: After you check why "BUY_IF_LOW" is not recognized, delete this variable
        let s1: Vec<Command> = command
        .filter(stonker_id.eq(s_id))
        .load::<Command>(&connection).unwrap();


        let stonker_commands: Vec<(Command, Company)> = command
            .filter(stonker_id.eq(s_id))
            .inner_join(company)
            .load::<(Command, Company)>(&connection)
            .context(format!(
                "404::::Could not find commands for stonker {}",
                s_id
            ))?;

        let stonker_history: Vec<StonkerHistoryJSON> = stonker_commands
            .iter()
            .map(|(cmd, comp)| StonkerHistoryJSON {
                day: format!("{}.{}", cmd.created_at.date().day(), cmd.created_at.date().month()),
                action: cmd.kind.clone(),
                stock: comp.name.clone(),
                money: cmd.threshold,
            })
            .collect();

        let stonker_stocks: Vec<(Stock, Company)> = Stock::belonging_to(&stonker_entity).inner_join(company)
            .load::<(Stock, Company)>(&connection)
            .context(format!(
                "404::::Could not find stocks belonging to stonker with id {}",
                s_id
        ))?;

        let portfolio: anyhow::Result<Vec<PortfolioJSON>> = stonker_stocks
        .iter()
        .map(|(st, comp)| {
            let cheapest_company_stocks: Vec<Option<i32>> = command
                .filter(company_id.eq(comp.id))
                .filter(kind.eq(CommandTypes::SELL))
                .select(min(threshold))
                .load::<Option<i32>>(&connection)?;
            let cheapest_company_stock = match cheapest_company_stocks.len() {
                0 => st.bought_for,
                _ => cheapest_company_stocks.get(0).unwrap().unwrap(),
            };

        Ok(PortfolioJSON {
            stock: comp.name.clone(),
            share: st.share,
            money: cheapest_company_stock - st.bought_for,
            difference: ((cheapest_company_stock -  st.bought_for) / st.bought_for) * 100,
        })})
        .collect();


        let result = StonkerOverviewJSON {
            portfolio: portfolio?,
            usage,
            stonker_history,
        };

        Ok(result)
    }

    async fn create_stonker(&self, new_stonker: NewStonker) -> anyhow::Result<Stonker> {
        let connection = self
            .pg_pool
            .get()
            .context("500::::Cannot get connection from pool")?;

        let result = diesel::insert_into(stonker::table)
            .values(&new_stonker)
            .get_result(&connection)
            .context("500::::Error saving new message")?;

        Ok(result)
    }

    async fn get_stonker_stocks(&self, s_id: i32) -> anyhow::Result<Vec<StockJSON>> {
        let connection = self
            .pg_pool
            .get()
            .context("500::::Cannot get connection from pool")?;
        let s: Stonker = stonker
            .find(s_id)
            .first(&connection)
            .context(format!("404::::Could not find stonker with id {}", s_id))?;

        let stonker_stocks: Vec<Stock> = Stock::belonging_to(&s)
            .load::<Stock>(&connection)
            .context(format!(
                "404::::Could not find stock belonging to stonker with id {}",
                s_id
            ))?;

        Ok(stocks_to_json(&connection, &stonker_stocks)?)
    }
}
