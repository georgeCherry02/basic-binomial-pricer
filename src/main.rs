mod result;

use chrono::NaiveDate;
use clap::{arg, Parser};
use log::info;
use result::PricerResult;

#[derive(Parser)]
pub struct Cli {
    pub underlying_price: f64,
    pub strike_price: f64,
    pub volatility: f64,
    pub expiry: NaiveDate,
    #[arg(default_value_t = 1000)]
    pub num_steps: u64,
    #[arg(default_value_t = 0.05)]
    pub apr: f64,
}

fn main() -> PricerResult<()> {
    env_logger::init();
    let args = Cli::parse();
    info!("Received a pricing call with:\nUnderlying Price = {}\nStrike Price = {}\nVolatility = {}\nExpiry = {}\nRisk-free Rate = {}\nAnd pricing with {} steps", args.underlying_price, args.strike_price, args.volatility, args.expiry, args.apr, args.num_steps);
    Ok(())
}
