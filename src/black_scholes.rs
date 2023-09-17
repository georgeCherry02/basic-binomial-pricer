use crate::option::{Call, FinancialOption, Put};
use crate::result::PricerResult;
use crate::utils::date::get_duration_in_years;

use chrono::prelude::Utc;
use chrono::DateTime;

use statrs::distribution::{ContinuousCDF, Normal};

fn get_d1_and_d2<O: FinancialOption>(
    option: &O,
    duration_in_years: f64,
    current_underlying_value: f64,
    rfr: f64,
) -> (f64, f64) {
    let d1 = (((current_underlying_value / option.strike()).ln())
        + ((rfr + (option.volatility().powi(2) / 2f64)) * duration_in_years))
        / (option.volatility() * duration_in_years.sqrt());
    let d2 = d1 - option.volatility() * duration_in_years.sqrt();
    (d1, d2)
}

pub trait BlackScholes: FinancialOption {
    fn value_black_scholes(
        &self,
        valuation_time: DateTime<Utc>,
        current_underlying_value: f64,
        rfr: f64,
    ) -> PricerResult<f64>;
}

impl BlackScholes for Call {
    fn value_black_scholes(
        &self,
        valuation_time: DateTime<Utc>,
        current_underlying_value: f64,
        rfr: f64,
    ) -> PricerResult<f64> {
        let duration_in_years = get_duration_in_years(valuation_time, self.expiry());
        let (d1, d2) = get_d1_and_d2(self, duration_in_years, current_underlying_value, rfr);
        let n = Normal::new(0.0, 1.0).unwrap();
        Ok(current_underlying_value * n.cdf(d1)
            - self.strike() * (-rfr * duration_in_years).exp() * n.cdf(d2))
    }
}

impl BlackScholes for Put {
    fn value_black_scholes(
        &self,
        valuation_time: DateTime<Utc>,
        current_underlying_value: f64,
        rfr: f64,
    ) -> PricerResult<f64> {
        let duration_in_years = get_duration_in_years(valuation_time, self.expiry());
        let (d1, d2) = get_d1_and_d2(self, duration_in_years, current_underlying_value, rfr);
        let n = Normal::new(0.0, 1.0).unwrap();
        Ok(self.strike() * (-rfr * duration_in_years).exp() * n.cdf(-d2) - current_underlying_value * n.cdf(-d1))
    }
}
