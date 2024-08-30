mod finite_difference;
#[cfg(test)]
mod test;

pub use finite_difference::FiniteDifferenceGreeks;

use crate::result::PricerResult;
use crate::risk_factors::RiskFactors;

use chrono::{DateTime, Utc};

pub trait AnalyticalGreeks {
    fn delta(&self, valuation_time: DateTime<Utc>, risk_factors: RiskFactors) -> PricerResult<f64>;
    fn gamma(&self, valuation_time: DateTime<Utc>, risk_factors: RiskFactors) -> PricerResult<f64>;
    fn rho(&self, valuation_time: DateTime<Utc>, risk_factors: RiskFactors) -> PricerResult<f64>;
    fn theta(&self, valuation_time: DateTime<Utc>, risk_factors: RiskFactors) -> PricerResult<f64>;
    fn vega(&self, valuation_time: DateTime<Utc>, risk_factors: RiskFactors) -> PricerResult<f64>;
}
