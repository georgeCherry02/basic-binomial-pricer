use crate::{symbol::Symbol, utils::date::get_duration_in_years};

use chrono::{DateTime, Utc};

pub struct AnnualisedDividendRate {
    symbol: Symbol,
    rate: f64,
}

trait DividendRf {
    fn approximate_annualised_rate(&self) -> f64;
    fn discount(
        &self,
        initial_value: f64,
        begin_date: DateTime<Utc>,
        end_date: DateTime<Utc>,
    ) -> f64;
}

impl DividendRf for AnnualisedDividendRate {
    fn approximate_annualised_rate(&self) -> f64 {
        self.rate
    }
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

pub enum Dividend {
    AnnualisedRate(AnnualisedDividendRate),
    Schedule,
}

impl DividendRf for Dividend {
    fn approximate_annualised_rate(&self) -> f64 {
        match &self {
            Dividend::AnnualisedRate(adr) => adr.approximate_annualised_rate(),
            Dividend::Schedule => panic!("Dividend schedules are not implement yet"),
        }
    }
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
