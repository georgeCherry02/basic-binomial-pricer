use crate::Cli;

use crate::black_scholes::BlackScholes;
use crate::option::{get_call, get_put};
use crate::{PricerError, PricerResult};

use chrono::prelude::Utc;
use chrono::{DateTime, NaiveDate, NaiveDateTime, NaiveTime};

use log::info;

use std::str::FromStr;

enum OptionType {
    CALL,
    PUT,
}

impl FromStr for OptionType {
    type Err = PricerError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "call" => Ok(OptionType::CALL),
            "put" => Ok(OptionType::PUT),
            _ => Err(PricerError {
                code: 2,
                message: String::from("Tried to price invalid type of option"),
            }),
        }
    }
}

struct ValidatedInterface {
    underlying_price: f64,
    option: Box<dyn BlackScholes>,
    annualised_rate: f64,
}

fn get_expiry_datetime(expiry_nd: NaiveDate) -> PricerResult<DateTime<Utc>> {
    let midnight = NaiveTime::from_hms_opt(0, 0, 0).unwrap();
    let expiry_naive = NaiveDateTime::new(expiry_nd, midnight);
    let expiry_dt: DateTime<Utc> = DateTime::<Utc>::from_utc(expiry_naive, Utc);
    let today: DateTime<Utc> = Utc::now();
    if expiry_dt < today {
        return Err(PricerError {
            message: String::from("Attempted to price expired option"),
            code: 1,
        });
    }
    Ok(expiry_dt)
}

fn construct_option(args: &Cli, expiry: DateTime<Utc>) -> PricerResult<Box<dyn BlackScholes>> {
    match args.option_type.as_str() {
        "call" => Ok(Box::new(get_call(args.strike_price, args.volatility, expiry))),
        "put" => Ok(Box::new(get_put(args.strike_price, args.volatility, expiry))),
        _ => Err(PricerError{code: 1, message: String::from("Failed to provide valid type of option, can price \"call\"s and \"put\"s")}),
    }
}

fn parse_cli(args: Cli) -> PricerResult<ValidatedInterface> {
    let naive_date = args.expiry;
    let expiry = get_expiry_datetime(naive_date)?;
    let option = construct_option(&args, expiry)?;
    Ok(ValidatedInterface{
        underlying_price: args.underlying_price,
        option,
        annualised_rate: args.apr,
    })
}

pub fn price(args: Cli) -> PricerResult<()> {
    parse_cli(args).and_then(|interface| {
        interface.option.value_black_scholes(Utc::now(), interface.underlying_price, interface.annualised_rate)
    }).map(|price| { info!("Priced Black-Scholes at {}", price); })
}
