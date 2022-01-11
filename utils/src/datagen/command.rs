use rand::prelude::SliceRandom;
use strum::IntoEnumIterator;

use crate::{json::{CommandJSON, CommandTypesJSON, CompanyJSON}, datagen::ToTSQLValue};

use super::{Generator, ToTSQL, Data, TSQLValue, JsonGenerator};

impl ToTSQLValue for CommandTypesJSON {
    fn to(&self) -> TSQLValue {
        match self {
            CommandTypesJSON::Sell => "SELL",
            CommandTypesJSON::SellIfHigh => "SELL_IF_HIGH",
            CommandTypesJSON::SellIfLow => "SELL_IF_LOW",
            CommandTypesJSON::BuyIfLow => "BUY_IF_LOW",
        }.to_string().to()
    }
}

impl ToTSQL for CommandJSON {
    fn to_header() -> &'static str {
        "Command"
    }
    fn to_columns() -> Vec<&'static str> {
        vec!["id", "stonker_id", "company_id", "threshold", "share", "kind", "created_at"]
    }
    fn to_data(&self) -> Vec<TSQLValue> {
        vec![self.id.to_id(), self.stonker.id.to_id(), self.company.id.to_id(),
            self.threshold.to(), self.share.to(), self.kind.to(), self.created_at.to()]
    }
}

fn get_lowest_sell(favorite: &CompanyJSON, data: &mut Data) -> Option<CommandJSON> {
    let mut lowest_price = i32::MAX;
    let mut chosen = None;
    for command in data.commands.iter() {
        if let CommandTypesJSON::Sell = command.kind {
            if command.company.id == favorite.id && command.threshold < lowest_price {
                lowest_price = command.threshold;
                chosen = Some(command.clone());
            }
        }
    }
    chosen
}

pub struct CommandGenerator;

impl JsonGenerator for CommandGenerator {
    fn new() -> anyhow::Result<CommandGenerator> {
        Ok(CommandGenerator)
    }
    fn create(&mut self, generator: &mut Generator, data: &mut Data) {
        // choose random stonker, if he owns stocks, put random sell command on their random stock
        // if he does not own stocks, put buy_if_low command on random stock
        let stonker = generator.choose(&mut data.stonkers).clone();
        let mut owned_stocks = Vec::new();
        for stock in data.stocks.iter() {
            if stock.owner.id == stonker.id {
                owned_stocks.push(stock.clone());
            }
        }
        let id = data.next();
        let all_types = CommandTypesJSON::iter().collect::<Vec<CommandTypesJSON>>();
        let kind = all_types.choose(&mut generator.random).unwrap();
        let is_buy = if let CommandTypesJSON::BuyIfLow = kind { true } else { false };
        if owned_stocks.is_empty() || is_buy {
            let favorite = generator.choose(&mut data.companies).clone();
            let chosen = get_lowest_sell(&favorite, data);
            if let None = chosen {
                // no one is selling this stock at the moment
                return;
            }
            let command = chosen.unwrap();
            let lower_price = command.threshold - generator.random_price(0, command.threshold / 10).abs();
            data.commands.push_back(CommandJSON{
                id,
                stonker,
                company: command.company.clone(),
                threshold: lower_price.max(1),
                share: command.share,
                kind: CommandTypesJSON::BuyIfLow,
                created_at: generator.random_date(command.created_at),
            });
            return;
        }
        let my_random_stock = owned_stocks.choose(&mut generator.random).unwrap();
        let cheapest = get_lowest_sell(&my_random_stock.issued_by, data);
        let shift = generator.random_price(0, cheapest.clone().map_or(100, |c| c.threshold) / 10).abs();
        let price = (cheapest.clone().map_or(100, |command| command.threshold) + match kind {
            CommandTypesJSON::Sell => -shift,
            CommandTypesJSON::SellIfHigh => shift,
            CommandTypesJSON::SellIfLow => -shift,
            CommandTypesJSON::BuyIfLow => panic!("I though we checked before"),
        }).max(1); // do not sell it for free
        let date = cheapest.map_or_else(|| Generator::get_beginning(), |command| command.created_at );
        data.commands.push_back(CommandJSON{
            id,
            stonker,
            company: my_random_stock.issued_by.clone(),
            threshold: price,
            share: my_random_stock.share,
            kind: kind.clone(),
            created_at: generator.random_date(date)
        })
    }

}