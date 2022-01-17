use std::collections::HashMap;
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
use crate::server_data::models::ToJson;
use crate::server_data::models::command::Command;
use crate::server_data::models::command::CommandTypes;
use crate::server_data::models::company::Company;
use crate::server_data::models::stonker::Stonker;
use anyhow::Context;
use async_trait::async_trait;
use chrono::Datelike;
use diesel::dsl::min;
use utils::json::PortfolioJSON;
use utils::json::StockJSON;
use utils::json::StonkerHistoryJSON;
use utils::json::StonkerJSON;
use utils::json::StonkerOverviewJSON;
use utils::json::UsageJSON;
use crate::schema::command::company_id;
use crate::schema::command::kind;
use crate::schema::command::threshold;

use super::Repo;

#[async_trait]
pub trait StonkerRepo {
    async fn get_stonkers(&self) -> anyhow::Result<Vec<Stonker>>;
    async fn get_stonker_overview(&self, s_id: i32) -> anyhow::Result<StonkerOverviewJSON>;
    async fn get_stonker_by_id(&self, s_id: i32) -> anyhow::Result<StonkerJSON>;
    async fn create_stonker(&self, new_stonker: NewStonker) -> anyhow::Result<StonkerJSON>;
    async fn get_stonker_stocks(&self, s_id: i32) -> anyhow::Result<Vec<StockJSON>>;
    async fn get_stonker_by_name(&self, name: &String) -> anyhow::Result<StonkerJSON>;
}

#[async_trait]
impl StonkerRepo for Repo {
    async fn get_stonkers(&self) -> anyhow::Result<Vec<Stonker>> {
        let connection = self.connect()?;
        let results = Repo::all::<Stonker, _>(
            &connection,
            stonker,
            "stonkers"
        )?;
        Ok(results)
    }

    async fn get_stonker_overview(&self, s_id: i32) -> anyhow::Result<StonkerOverviewJSON> {
        let connection = self.connect()?;
        let stonker_entity = Repo::find::<Stonker, _>(
            &connection,
            stonker,
            s_id,
            "stonker"
        )?;

        let usage = UsageJSON {
            free: stonker_entity.balance,
            invested: stonker_entity.invested_balance,
            blocked: stonker_entity.blocked_balance,
        };

        // TODO: After you check why "BUY_IF_LOW" is not recognized, delete this variable
        let s1: Vec<Command> = command
        .filter(stonker_id.eq(s_id))
        .load::<Command>(&connection).unwrap();

        let stonker_commands = Repo::all::<(Command, Company), _>(
            &connection,
            command.filter(stonker_id.eq(s_id)).inner_join(company),
            format!("commands for stonker {}", s_id).as_str()
        )?;

        let stonker_history: Vec<StonkerHistoryJSON> = stonker_commands
            .iter()
            .filter_map(|(cmd, comp)| Some(StonkerHistoryJSON {
                day: format!("{}.{}", cmd.created_at.date().day(), cmd.created_at.date().month()),
                action: cmd.kind.to_json(&connection).ok()?,
                stock: comp.name.clone(),
                money: cmd.threshold,
            }))
            .collect();

        let stonker_stocks = Repo::all::<(Stock, Company), _>(
            &connection,
            Stock::belonging_to(&stonker_entity).inner_join(company),
            format!("stocks belonging to stonker with id {}", s_id).as_str()
        )?;

        let portfolio: anyhow::Result<Vec<PortfolioJSON>> = stonker_stocks
        .iter()
        .map(|(st, comp)| {
            let cheapest_company_stocks = Repo::all::<Option<i32>, _>(
                &connection,
                command
                .filter(company_id.eq(comp.id))
                .filter(kind.eq(CommandTypes::Sell))
                .select(min(threshold)),
                "cheapest company stocks"
            )?;
            let cheapest_company_stock = match cheapest_company_stocks.len() {
                0 => st.bought_for,
                _ => cheapest_company_stocks.get(0).unwrap().unwrap(),
            };

            let difference: i32 = if st.bought_for > 0 {
                (((cheapest_company_stock -  st.bought_for) as f32 / st.bought_for as f32) * 100.0) as i32
            } else { 0 };


            print!("Cheapest comapny stocks:");
            println!("{}", cheapest_company_stock);
            print!("BoughtFor: ");
            println!("{}", st.bought_for);
        Ok(PortfolioJSON {
            stock: comp.name.clone(),
            share: st.share,
            money: cheapest_company_stock - st.bought_for,
            difference,
            bought_for: st.bought_for,
        })})
        .collect();

        let mut portfolio_grp: HashMap<String, PortfolioJSON> = HashMap::new();

        for port in portfolio.as_ref().unwrap().iter() {
            if !portfolio_grp.contains_key(&port.stock) {
                portfolio_grp.insert(port.stock.clone(), port.clone());
                continue;
            }

            let mut p = match portfolio_grp.get_mut(&port.stock) {
                None => {panic!("Invalid state")}
                Some(pp) => pp
            };
            p.share += port.share;
            p.money += port.money;
            p.bought_for += port.bought_for;
        }

        let mut portfolio_overview: Vec<PortfolioJSON> = portfolio_grp.into_values().collect();
        for port in portfolio_overview.iter_mut() {
            port.difference = if port.bought_for > 0 {
                (port.money as f32 / port.bought_for as f32 * 100.0) as i32
            } else { 0 };
        }


        let result = StonkerOverviewJSON {
            portfolio: portfolio?,
            usage,
            stonker_history,
            portfolio_overview,
        };

        Ok(result)
    }

    async fn get_stonker_by_id(&self, s_id: i32) -> anyhow::Result<StonkerJSON> {
        let connection = self.connect()?;
        let result = Repo::find::<Stonker, _>(
            &connection,
            stonker,
            s_id,
            "stonker"
        )?;
        result.to_json(&connection)
    }

    async fn create_stonker(&self, new_stonker: NewStonker) -> anyhow::Result<StonkerJSON> {
        let connection = self.connect()?;

        let result: Stonker = diesel::insert_into(stonker::table)
            .values(&new_stonker)
            .get_result::<Stonker>(&connection)
            .context("500::::Error saving stonker")?;

        result.to_json(&connection)
    }

    async fn get_stonker_stocks(&self, s_id: i32) -> anyhow::Result<Vec<StockJSON>> {
        let connection = self.connect()?;
        let s = Repo::find::<Stonker, _>(
            &connection,
            stonker,
            s_id,
            "stonker"
        )?;
        let stonker_stocks = Repo::all::<Stock, _>(
            &connection,
            Stock::belonging_to(&s),
            format!("stocks belonging to stonker with id {}", s_id).as_str()
        )?;
        stonker_stocks.to_json(&connection)
    }

    async fn get_stonker_by_name(&self, nname: &String) -> anyhow::Result<StonkerJSON> {
        let connection = self.connect()?;
        let stonkers = Repo::all::<Stonker, _>(
            &connection,
            stonker,
            "stonkers"
        )?;
        for s in stonkers.iter() {
            if &s.name == nname {
                return s.to_json(&connection);
            }
        }
        return Err(anyhow::Error::msg("No such stonker."));
    }
}
