mod black_scholes;
mod cli;
mod option;
mod result;
mod risk_free_model;
mod tree;
mod utils;

use chrono::NaiveDate;
use clap;
use clap::Parser;
use result::{PricerError, PricerResult};

#[derive(Parser)]
pub struct Cli {
    pub underlying_price: f64,
    pub option_type: String,
    pub strike_price: f64,
    pub volatility: f64,
    pub expiry: NaiveDate,
    #[arg(default_value_t = 0.05)]
    pub apr: f64,
}

fn main() -> PricerResult<()> {
    env_logger::init();
    let args = Cli::parse();
    cli::price(args)
}
