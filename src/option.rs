use chrono::prelude::Utc;
use chrono::DateTime;

use std::fmt;

pub struct Call {
    pub strike: f64,
    pub volatility: f64,
    pub expiry: DateTime<Utc>,
}

impl fmt::Display for Call {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[strike = {}, volatility = {}, expiry = {}]",
            self.strike,
            self.volatility,
            self.expiry.format("%F")
        )
    }
}

pub fn get_call(strike: f64, volatility: f64, expiry: DateTime<Utc>) -> Call {
    return Call {
        strike,
        volatility,
        expiry,
    };
}
