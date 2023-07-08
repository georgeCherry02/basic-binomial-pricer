mod result;

use chrono::prelude::Utc;
use chrono::{DateTime, NaiveDate, NaiveDateTime, NaiveTime};
use clap::{arg, Parser};
use log::info;
use result::{PricerError, PricerResult};

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
    let midnight = NaiveTime::from_hms_opt(0, 0, 0).unwrap();
    let expiry_naive = NaiveDateTime::new(args.expiry, midnight);
    let expiry_dt: DateTime<Utc> = DateTime::<Utc>::from_utc(expiry_naive, Utc);
    let today: DateTime<Utc> = Utc::now();
    if expiry_dt < today {
        return Err(PricerError {
            message: String::from("Attempted to price expired option"),
            code: 1,
        });
    }
    info!("Received a pricing call with:\nUnderlying Price = {}\nStrike Price = {}\nVolatility = {}\nExpiry = {}\nRisk-free Rate = {}\nAnd pricing with {} steps", args.underlying_price, args.strike_price, args.volatility, args.expiry, args.apr, args.num_steps);
    Ok(())
}
