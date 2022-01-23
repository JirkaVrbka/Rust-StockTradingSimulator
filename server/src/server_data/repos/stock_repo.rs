use crate::diesel::RunQueryDsl;
use crate::models::stock::NewStock;
use crate::schema::stock;
use crate::schema::stock::dsl::*;
use crate::server_data::models::ToJson;
use crate::models::stock::Stock;
use anyhow::{anyhow, Context};
use async_trait::async_trait;
use diesel::{ExpressionMethods, QueryDsl, QueryResult};
use utils::json::StockJSON;
use crate::models::command::{Command, NewCommand, CommandTypes};
use crate::models::stonker::Stonker;
use crate::schema::command;
use crate::schema::command::dsl::*;
use crate::schema::stonker::dsl::*;
use crate::schema::stonker;
use super::Repo;

#[async_trait]
pub trait StockRepo {
    async fn get_stocks(&self) -> anyhow::Result<Vec<StockJSON>>;
    async fn get_stock_by_id(&self, stock_id: i32) -> anyhow::Result<StockJSON>;
    async fn create_stock(&self, new_stock: NewStock) -> anyhow::Result<StockJSON>;
    async fn sell_stock(&self, command: NewCommand) -> anyhow::Result<()>;
    async fn buy_stock(&self, command_r: NewCommand) -> anyhow::Result<()>;
    fn resolve_commands(&self, company_id: i32) -> ();
    fn perform_trade(&self, buy_cmd: &mut Command, sell_cmd: &mut Command) -> bool;
}

#[async_trait]
impl StockRepo for Repo {
    async fn get_stocks(&self) -> anyhow::Result<Vec<StockJSON>> {
        let connection = self.connect()?;
        let stock_entities = Repo::all::<Stock, _>(
            &connection,
            stock,
            "stocks"
        )?;
        stock_entities.to_json(&connection)
    }

    async fn get_stock_by_id(&self, stock_id: i32) -> anyhow::Result<StockJSON> {
        let connection = self.connect()?;
        let result = Repo::find::<Stock, _>(
            &connection,
            stock,
            stock_id,
            "stock"
        )?;
        result.to_json(&connection)
    }

    async fn create_stock(&self, new_stock: NewStock) -> anyhow::Result<StockJSON> {
        let connection = self.connect()?;

        let result: &Stock = &diesel::insert_into(stock::table)
            .values(&new_stock)
            .get_result(&connection)
            .context("500::::Could not save stock")?;

        result.to_json(&connection)
    }

    async fn sell_stock(&self, command_r: NewCommand) -> anyhow::Result<()> {
        let connection = self.connect()?;
        let stock_share: i32 = Repo::all::<Stock, _>(
            &connection,
            stock,
            "stocks"
        ).unwrap().into_iter()
            .filter(|s| s.company_id == command_r.company_id && s.stonker_id == command_r.stonker_id)
            .map(|s| s.share)
            .sum();

        if stock_share < command_r.share {
            return Err(anyhow!("You don't have enough share."));
        }

        let result = &diesel::insert_into(command::table)
            .values(&command_r)
            .execute(&connection)?;

        self.resolve_commands(command_r.company_id);

        Ok(())
    }

    async fn buy_stock(&self, command_r: NewCommand) -> anyhow::Result<()> {
        let connection = self.connect()?;

        let stonker_entity = Repo::find::<Stonker, _>(
            &connection,
            stonker,
            command_r.stonker_id,
            "stonker"
        )?;

        if command_r.threshold > stonker_entity.balance {
            return Err(anyhow!("You don't have enough money."));
        }

        let updated_row: QueryResult<Stonker> = diesel::update(stonker::table)
            .filter(stonker::id.eq(command_r.stonker_id))
            .set((balance.eq(stonker_entity.balance - command_r.threshold),
                  blocked_balance.eq(stonker_entity.blocked_balance + command_r.threshold)))
            .get_result(&connection);

        let result = &diesel::insert_into(command::table)
            .values(&command_r)
            .execute(&connection)?;

        self.resolve_commands(command_r.company_id);

        Ok(())
    }

    fn resolve_commands(&self, com_id: i32) -> () {
        let connection = self.connect().unwrap();
        let mut sell_commands:Vec<Command> = Repo::all::<Command, _>(
            &connection,
            command,
            "command"
        ).unwrap().into_iter()
            .filter(|cmd| cmd.company_id == com_id && (cmd.kind == CommandTypes::Sell || cmd.kind == CommandTypes::SellIfHigh) && cmd.share > 0)
            .collect();

        let mut buy_commands:Vec<Command> = Repo::all::<Command, _>(
            &connection,
            command,
            "command"
        ).unwrap().into_iter()
            .filter(|cmd| cmd.company_id == com_id && (cmd.kind == CommandTypes::BuyIfLow) && cmd.share > 0)
            .collect();



        for b_cmd in buy_commands.iter_mut() {
            for s_cmd in sell_commands.iter_mut() {
                if s_cmd.threshold > b_cmd.threshold {
                    continue;
                }
                if self.perform_trade(b_cmd, s_cmd) {
                    break;
                }
            }
        }
    }

    fn perform_trade(&self, buy_cmd: &mut Command, sell_cmd: &mut Command) -> bool {
        let connection = self.connect().unwrap();
        let b_stonker: Stonker = Repo::find::<Stonker, _>(
            &connection,
            stonker,
            buy_cmd.stonker_id,
            "stonker"
        ).unwrap();
        let s_stonker: Stonker = Repo::find::<Stonker, _>(
            &connection,
            stonker,
            sell_cmd.id,
            "stonker"
        ).unwrap();

        let stock_entities: Vec<Stock> = Repo::all::<Stock, _>(
            &connection,
            stock,
            "stocks"
        ).unwrap().into_iter()
            .filter(|s| s.stonker_id == sell_cmd.stonker_id)
            .collect();

        let mut bought = 0;

        for s in stock_entities {
            if buy_cmd.share == 0 {
                break;
            }

            if s.share <= buy_cmd.share {
                let updated_row = diesel::update(stock::table)
                    .filter(stock::id.eq(s.id))
                    .set(stock::stonker_id.eq(buy_cmd.stonker_id))
                    .execute(&connection);
                buy_cmd.share -= s.share;
                bought += s.share;
            } else {
                let updated_row = diesel::update(stock::table)
                    .filter(stock::id.eq(s.id))
                    .set(stock::share.eq(s.share - buy_cmd.share))
                    .execute(&connection);
                self.create_stock(NewStock {
                    stonker_id: buy_cmd.stonker_id,
                    share: buy_cmd.share,
                    bought_for: buy_cmd.share,
                    company_id: buy_cmd.company_id
                });
                buy_cmd.share = 0;
                bought += buy_cmd.share;
            }
        }

        let updated_row: QueryResult<Stonker> = diesel::update(stonker::table)
            .filter(stonker::id.eq(buy_cmd.stonker_id))
            .set((blocked_balance.eq(b_stonker.blocked_balance - buy_cmd.threshold),
                  invested_balance.eq(b_stonker.invested_balance + buy_cmd.threshold)))
            .get_result(&connection);

        let updated_row: QueryResult<Stonker> = diesel::update(stonker::table)
            .filter(stonker::id.eq(buy_cmd.stonker_id))
            .set((balance.eq(b_stonker.balance + buy_cmd.threshold),
                  invested_balance.eq(s_stonker.invested_balance - buy_cmd.threshold)))
            .get_result(&connection);

        buy_cmd.share == 0
    }
}

