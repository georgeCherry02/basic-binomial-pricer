use crate::Cli;

use crate::black_scholes::BlackScholes;
use crate::option::{get_call, get_put};
use crate::result::{PricerError, PricerResult};
use crate::risk_factor::RiskFactors;

use chrono::prelude::Utc;
use chrono::{DateTime, NaiveDate, NaiveDateTime, NaiveTime};

use log::info;

use std::str::FromStr;

#[derive(Clone)]
pub enum OptionType {
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
                message: String::from("Tried to parse invalid type of option"),
            }),
        }
    }
}

struct ValidatedInterface {
    option: Box<dyn BlackScholes>,
    risk_factors: RiskFactors,
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

fn construct_option(
    args: &Cli,
    expiry: DateTime<Utc>,
    cost: f64,
) -> PricerResult<Box<dyn BlackScholes>> {
    match args.option_type {
        OptionType::CALL => Ok(Box::new(get_call(args.strike_price, expiry, cost))),
        OptionType::PUT => Ok(Box::new(get_put(args.strike_price, expiry, cost))),
    }
}

fn parse_cli(args: Cli) -> PricerResult<ValidatedInterface> {
    let naive_date = args.expiry;
    let expiry = get_expiry_datetime(naive_date)?;
    let option = construct_option(&args, expiry, 0.0)?;
    let risk_factors = RiskFactors::new(args.underlying_price, args.volatility, args.apr);
    Ok(ValidatedInterface {
        option,
        risk_factors,
    })
}

pub fn price(args: Cli) -> PricerResult<()> {
    parse_cli(args)
        .and_then(|interface| {
            interface
                .option
                .value_black_scholes(Utc::now(), interface.risk_factors, vec![])
        })
        .map(|price| {
            info!("Priced Black-Scholes at {}", price);
        })
}
