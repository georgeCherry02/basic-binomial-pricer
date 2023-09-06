use chrono::prelude::Utc;
use chrono::DateTime;

use std::fmt;

pub struct Call {
    strike: f64,
    volatility: f64,
    expiry: DateTime<Utc>,
}

pub fn get_call(strike: f64, volatility: f64, expiry: DateTime<Utc>) -> Call {
    Call {
        strike,
        volatility,
        expiry,
    }
}

pub struct Put {
    strike: f64,
    volatility: f64,
    expiry: DateTime<Utc>,
}

pub fn get_put(strike: f64, volatility: f64, expiry: DateTime<Utc>) -> Put {
    Put {
        strike,
        volatility,
        expiry,
    }
}

pub trait FinancialOption {
    fn strike(&self) -> f64;
    fn volatility(&self) -> f64;
    fn expiry(&self) -> DateTime<Utc>;
    fn value_if_executed(&self, underlying_value: f64) -> f64;
}

impl FinancialOption for Call {
    fn strike(&self) -> f64 {
        self.strike
    }
    fn volatility(&self) -> f64 {
        self.volatility
    }
    fn expiry(&self) -> DateTime<Utc> {
        self.expiry
    }
    fn value_if_executed(&self, underlying_value: f64) -> f64 {
        underlying_value - self.strike()
    }
}

impl FinancialOption for Put {
    fn strike(&self) -> f64 {
        self.strike
    }
    fn volatility(&self) -> f64 {
        self.volatility
    }
    fn expiry(&self) -> DateTime<Utc> {
        self.expiry
    }
    fn value_if_executed(&self, underlying_value: f64) -> f64 {
        self.strike() - underlying_value
    }
}

fn local_fmt<T: FinancialOption>(t: &T, prefix: &str, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(
        f,
        "{}[strike={}, volatility={}, expiry={}]",
        prefix,
        t.strike(),
        t.volatility(),
        t.expiry(),
    )
}

impl fmt::Display for Call {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        local_fmt(self, "Call", f)
    }
}

impl fmt::Display for Put {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        local_fmt(self, "Call", f)
    }
}
