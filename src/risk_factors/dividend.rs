use super::IdentifiableRiskFactor;

use crate::{symbol::Symbol, utils::date::get_duration_in_years};

use chrono::{DateTime, Utc};

#[derive(Clone)]
pub struct AnnualisedDividendRate {
    symbol: Symbol,
    rate: f64,
}

impl AnnualisedDividendRate {
    pub fn new(symbol: Symbol, rate: f64) -> AnnualisedDividendRate {
        AnnualisedDividendRate { symbol, rate }
    }
    pub fn rate(&self) -> f64 {
        self.rate
    }
}

pub trait DividendRf {
    fn discount(
        &self,
        initial_value: f64,
        begin_date: DateTime<Utc>,
        end_date: DateTime<Utc>,
    ) -> f64;
}

impl DividendRf for AnnualisedDividendRate {
    fn discount(
        &self,
        initial_value: f64,
        begin_date: DateTime<Utc>,
        end_date: DateTime<Utc>,
    ) -> f64 {
        let delta_t = get_duration_in_years(begin_date, end_date);
        initial_value * (-self.rate * delta_t).exp()
    }
}

impl IdentifiableRiskFactor for AnnualisedDividendRate {
    fn id(&self) -> &Symbol {
        &self.symbol
    }
}

#[derive(Clone)]
pub enum Dividend {
    AnnualisedRate(AnnualisedDividendRate),
    Schedule,
}

impl DividendRf for Dividend {
    fn discount(
        &self,
        initial_value: f64,
        begin_date: DateTime<Utc>,
        end_date: DateTime<Utc>,
    ) -> f64 {
        match &self {
            Dividend::AnnualisedRate(adr) => adr.discount(initial_value, begin_date, end_date),
            Dividend::Schedule => panic!("Dividend schedules are not implement yet"),
        }
    }
}

impl IdentifiableRiskFactor for Dividend {
    fn id(&self) -> &Symbol {
        match &self {
            Dividend::AnnualisedRate(adr) => adr.id(),
            Dividend::Schedule => panic!("Dividend schedules are not implement yet"),
        }
    }
}
