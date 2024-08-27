use super::common::{gaussian, get_d1_and_d2};
use super::types::BlackScholesInputs;
use super::BlackScholes;
use crate::option::{Call, FinancialOption, Put};
use crate::result::PricerResult;
use crate::risk_factor::RiskFactors;

use chrono::{DateTime, Utc};

use statrs::distribution::{Continuous, ContinuousCDF, Normal};
use statrs::StatsError;

pub trait BlackScholesGreeks: BlackScholes {
    fn delta(&self, valuation_time: DateTime<Utc>, risk_factors: RiskFactors) -> PricerResult<f64>;
    fn gamma(&self, valuation_time: DateTime<Utc>, risk_factors: RiskFactors) -> PricerResult<f64>;
    fn rho(&self, valuation_time: DateTime<Utc>, risk_factors: RiskFactors) -> PricerResult<f64>;
    fn theta(&self, valuation_time: DateTime<Utc>, risk_factors: RiskFactors) -> PricerResult<f64>;
    fn vega(&self, valuation_time: DateTime<Utc>, risk_factors: RiskFactors) -> PricerResult<f64>;
}

impl BlackScholesGreeks for Call {
    fn delta(&self, valuation_time: DateTime<Utc>, risk_factors: RiskFactors) -> PricerResult<f64> {
        let inputs = BlackScholesInputs::gather(self, valuation_time, risk_factors);
        let (d1, _) = get_d1_and_d2(self.strike(), &inputs);
        gaussian().map(|gaussian| gaussian.cdf(d1))
    }
    fn gamma(&self, valuation_time: DateTime<Utc>, risk_factors: RiskFactors) -> PricerResult<f64> {
        let inputs = BlackScholesInputs::gather(self, valuation_time, risk_factors);
        let (d1, _) = get_d1_and_d2(self.strike(), &inputs);
        let one_over_price_vol_delta_t =
            1.0 / (inputs.underlying_price * inputs.underlying_volatility * inputs.delta_t.sqrt());
        gaussian().map(|gaussian| gaussian.pdf(d1) * one_over_price_vol_delta_t)
    }
    fn rho(&self, valuation_time: DateTime<Utc>, risk_factors: RiskFactors) -> PricerResult<f64> {}
    fn theta(&self, valuation_time: DateTime<Utc>, risk_factors: RiskFactors) -> PricerResult<f64> {
    }
    fn vega(&self, valuation_time: DateTime<Utc>, risk_factors: RiskFactors) -> PricerResult<f64> {}
}

impl BlackScholesGreeks for Put {
    fn delta(&self, valuation_time: DateTime<Utc>, risk_factors: RiskFactors) -> PricerResult<f64> {
        let inputs = BlackScholesInputs::gather(self, valuation_time, risk_factors);
        let (d1, _) = get_d1_and_d2(self.strike(), &inputs);
        gaussian().map(|gaussian| gaussian.cdf(d1) - 1.)
    }
    fn gamma(&self, valuation_time: DateTime<Utc>, risk_factors: RiskFactors) -> PricerResult<f64> {
        let inputs = BlackScholesInputs::gather(self, valuation_time, risk_factors);
        let (d1, _) = get_d1_and_d2(self.strike(), &inputs);
        let one_over_price_vol_delta_t =
            1.0 / (inputs.underlying_price * inputs.underlying_volatility * inputs.delta_t.sqrt());
        gaussian().map(|gaussian| gaussian.pdf(d1) * one_over_price_vol_delta_t)
    }
    fn rho(&self, valuation_time: DateTime<Utc>, risk_factors: RiskFactors) -> PricerResult<f64> {}
    fn theta(&self, valuation_time: DateTime<Utc>, risk_factors: RiskFactors) -> PricerResult<f64> {
    }
    fn vega(&self, valuation_time: DateTime<Utc>, risk_factors: RiskFactors) -> PricerResult<f64> {}
}
