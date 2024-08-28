mod cli;
mod greeks;
mod option;
mod result;
mod risk_factors;
mod risk_free_model;
mod shock;
mod utils;

mod black_scholes;
mod monte_carlo;
mod tree;

use chrono::NaiveDate;
use clap;
use clap::Parser;
use result::PricerResult;

#[derive(Parser)]
pub struct Cli {
    pub underlying_price: f64,
    pub option_type: cli::OptionType,
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
