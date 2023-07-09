mod option;
mod result;
mod risk_free_model;

use chrono::prelude::Utc;
use chrono::{DateTime, NaiveDate, NaiveDateTime, NaiveTime};
use clap::{arg, Parser};
use log::info;
use option::Call;
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

struct ValidatedInterface {
    underlying_price: f64,
    call: Call,
    num_steps: u64,
    annualised_rate: f64,
}

fn parse_cli(args: Cli) -> PricerResult<ValidatedInterface> {
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
    let call = option::get_call(args.strike_price, args.volatility, expiry_dt);
    Ok(ValidatedInterface {
        underlying_price: args.underlying_price,
        call,
        num_steps: args.num_steps,
        annualised_rate: args.apr,
    })
}

fn main() -> PricerResult<()> {
    env_logger::init();
    let args = Cli::parse();
    let interface = parse_cli(args)?;
    info!("Received a pricing call with:\nUnderlying Price = {}\nCall = {}\nRisk-free Rate = {}\nAnd pricing with {} steps", interface.underlying_price, interface.call, interface.annualised_rate, interface.num_steps);
    Ok(())
}
