use crate::result::{PricerError, PricerResult};

use std::str::FromStr;

pub struct Symbol {
    pub id: String,
}

impl FromStr for Symbol {
    type Err = PricerError;
    fn from_str(s: &str) -> PricerResult<Self> {
        Ok(Symbol {
            id: String::from(s),
        })
    }
}
