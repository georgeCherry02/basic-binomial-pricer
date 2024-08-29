use super::common::{gaussian, get_d1_and_d2};
use super::types::{BlackScholesInputs, BlackScholesRiskFactors};
use super::BlackScholes;
use crate::option::{Call, FinancialOption, Put};
use crate::result::PricerResult;
use crate::risk_factors::RiskFactors;

use chrono::{DateTime, Utc};

use statrs::distribution::{Continuous, ContinuousCDF};

static DAYS_IN_YEAR: u32 = 365;

// Source of equations: https://www.macroption.com/black-scholes-formula/

type BlackScholesGreekImplementation =
    fn(&dyn BlackScholesGreeks, DateTime<Utc>, BlackScholesRiskFactors) -> PricerResult<f64>;

fn map_to_impl(
    greeks: &dyn BlackScholesGreeks,
    valuation_time: DateTime<Utc>,
    risk_factors: RiskFactors,
    implementation: BlackScholesGreekImplementation,
) -> PricerResult<f64> {
    risk_factors
        .try_into()
        .and_then(|risk_factors| {
            greeks.is_sensitive_to_risk_factors(&risk_factors)?;
            Ok(risk_factors)
        })
        .and_then(|risk_factors| implementation(greeks, valuation_time, risk_factors))
}

pub trait BlackScholesGreeks: BlackScholes {
    fn delta(&self, valuation_time: DateTime<Utc>, risk_factors: RiskFactors) -> PricerResult<f64>
    where
        Self: Sized,
    {
        map_to_impl(self, valuation_time, risk_factors, |bs, dt, rf| {
            bs.delta_impl(dt, rf)
        })
    }
    fn gamma(&self, valuation_time: DateTime<Utc>, risk_factors: RiskFactors) -> PricerResult<f64>
    where
        Self: Sized,
    {
        map_to_impl(self, valuation_time, risk_factors, |bs, dt, rf| {
            bs.gamma_impl(dt, rf)
        })
    }
    fn rho(&self, valuation_time: DateTime<Utc>, risk_factors: RiskFactors) -> PricerResult<f64>
    where
        Self: Sized,
    {
        map_to_impl(self, valuation_time, risk_factors, |bs, dt, rf| {
            bs.rho_impl(dt, rf)
        })
    }
    fn theta(&self, valuation_time: DateTime<Utc>, risk_factors: RiskFactors) -> PricerResult<f64>
    where
        Self: Sized,
    {
        map_to_impl(self, valuation_time, risk_factors, |bs, dt, rf| {
            bs.theta_impl(dt, rf)
        })
    }
    fn vega(&self, valuation_time: DateTime<Utc>, risk_factors: RiskFactors) -> PricerResult<f64>
    where
        Self: Sized,
    {
        map_to_impl(self, valuation_time, risk_factors, |bs, dt, rf| {
            bs.vega_impl(dt, rf)
        })
    }
    fn delta_impl(
        &self,
        valuation_time: DateTime<Utc>,
        risk_factors: BlackScholesRiskFactors,
    ) -> PricerResult<f64>;
    fn gamma_impl(
        &self,
        valuation_time: DateTime<Utc>,
        risk_factors: BlackScholesRiskFactors,
    ) -> PricerResult<f64>;
    fn rho_impl(
        &self,
        valuation_time: DateTime<Utc>,
        risk_factors: BlackScholesRiskFactors,
    ) -> PricerResult<f64>;
    fn theta_impl(
        &self,
        valuation_time: DateTime<Utc>,
        risk_factors: BlackScholesRiskFactors,
    ) -> PricerResult<f64>;
    fn vega_impl(
        &self,
        valuation_time: DateTime<Utc>,
        risk_factors: BlackScholesRiskFactors,
    ) -> PricerResult<f64>;
}

impl BlackScholesGreeks for Call {
    fn delta_impl(
        &self,
        valuation_time: DateTime<Utc>,
        risk_factors: BlackScholesRiskFactors,
    ) -> PricerResult<f64> {
        let inputs = BlackScholesInputs::gather(self, valuation_time, risk_factors);
        let (d1, _) = get_d1_and_d2(self.strike(), &inputs);
        gaussian().map(|gaussian| inputs.dividend_adjustment() * gaussian.cdf(d1))
    }
    fn gamma_impl(
        &self,
        valuation_time: DateTime<Utc>,
        risk_factors: BlackScholesRiskFactors,
    ) -> PricerResult<f64> {
        let inputs = BlackScholesInputs::gather(self, valuation_time, risk_factors);
        let (d1, _) = get_d1_and_d2(self.strike(), &inputs);
        let one_over_price_vol_delta_t =
            inputs.dividend_adjustment() / (inputs.price() * inputs.volatility_for_delta_t());
        gaussian().map(|gaussian| gaussian.pdf(d1) * one_over_price_vol_delta_t)
    }
    fn rho_impl(
        &self,
        valuation_time: DateTime<Utc>,
        risk_factors: BlackScholesRiskFactors,
    ) -> PricerResult<f64> {
        let inputs = BlackScholesInputs::gather(self, valuation_time, risk_factors);
        let (_, d2) = get_d1_and_d2(self.strike(), &inputs);
        gaussian().map(|gaussian| {
            0.01 * self.strike() * inputs.delta_t * inputs.risk_free_adjustment() * gaussian.cdf(d2)
        })
    }
    fn theta_impl(
        &self,
        valuation_time: DateTime<Utc>,
        risk_factors: BlackScholesRiskFactors,
    ) -> PricerResult<f64> {
        let inputs = BlackScholesInputs::gather(self, valuation_time, risk_factors);
        let (d1, d2) = get_d1_and_d2(self.strike(), &inputs);
        let lost_price_movement =
            -(inputs.price() * inputs.volatility_for_delta_t()) / (2.0 * inputs.delta_t);
        let risk_free_adjustment =
            -(inputs.discount_rate() * self.strike() * inputs.risk_free_adjustment());
        let dividend_adjustment =
            inputs.annualised_dividend_rate() * inputs.dividend_adjusted_price();
        gaussian()
            .map(|gaussian| {
                lost_price_movement * gaussian.pdf(d1)
                    + risk_free_adjustment * gaussian.cdf(d2)
                    + dividend_adjustment * gaussian.cdf(d1)
            })
            .map(|value| value / DAYS_IN_YEAR as f64)
    }
    fn vega_impl(
        &self,
        valuation_time: DateTime<Utc>,
        risk_factors: BlackScholesRiskFactors,
    ) -> PricerResult<f64> {
        let inputs = BlackScholesInputs::gather(self, valuation_time, risk_factors);
        let (d1, _) = get_d1_and_d2(self.strike(), &inputs);
        gaussian().map(|gaussian| {
            0.01 * inputs.dividend_adjusted_price() * inputs.delta_t.sqrt() * gaussian.pdf(d1)
        })
    }
}

impl BlackScholesGreeks for Put {
    fn delta_impl(
        &self,
        valuation_time: DateTime<Utc>,
        risk_factors: BlackScholesRiskFactors,
    ) -> PricerResult<f64> {
        let inputs = BlackScholesInputs::gather(self, valuation_time, risk_factors);
        let (d1, _) = get_d1_and_d2(self.strike(), &inputs);
        gaussian().map(|gaussian| inputs.dividend_adjustment() * (gaussian.cdf(d1) - 1.0))
    }
    fn gamma_impl(
        &self,
        valuation_time: DateTime<Utc>,
        risk_factors: BlackScholesRiskFactors,
    ) -> PricerResult<f64> {
        let inputs = BlackScholesInputs::gather(self, valuation_time, risk_factors);
        let (d1, _) = get_d1_and_d2(self.strike(), &inputs);
        let one_over_price_vol_delta_t =
            inputs.dividend_adjustment() / (inputs.price() * inputs.volatility_for_delta_t());
        gaussian().map(|gaussian| gaussian.pdf(d1) * one_over_price_vol_delta_t)
    }
    fn rho_impl(
        &self,
        valuation_time: DateTime<Utc>,
        risk_factors: BlackScholesRiskFactors,
    ) -> PricerResult<f64> {
        let inputs = BlackScholesInputs::gather(self, valuation_time, risk_factors);
        let (_, d2) = get_d1_and_d2(self.strike(), &inputs);
        gaussian().map(|gaussian| {
            -0.01
                * self.strike()
                * inputs.delta_t
                * inputs.risk_free_adjustment()
                * gaussian.cdf(-d2)
        })
    }
    fn theta_impl(
        &self,
        valuation_time: DateTime<Utc>,
        risk_factors: BlackScholesRiskFactors,
    ) -> PricerResult<f64> {
        let inputs = BlackScholesInputs::gather(self, valuation_time, risk_factors);
        let (d1, d2) = get_d1_and_d2(self.strike(), &inputs);
        let lost_price_movement =
            -(inputs.price() * inputs.volatility_for_delta_t()) / (2.0 * inputs.delta_t);
        let risk_free_adjustment =
            inputs.discount_rate() * self.strike() * inputs.risk_free_adjustment();
        let dividend_adjustment =
            -inputs.annualised_dividend_rate() * inputs.dividend_adjusted_price();
        gaussian()
            .map(|gaussian| {
                lost_price_movement * gaussian.pdf(d1)
                    + risk_free_adjustment * gaussian.cdf(d2)
                    + dividend_adjustment * gaussian.cdf(-d1)
            })
            .map(|value| value / DAYS_IN_YEAR as f64)
    }
    fn vega_impl(
        &self,
        valuation_time: DateTime<Utc>,
        risk_factors: BlackScholesRiskFactors,
    ) -> PricerResult<f64> {
        let inputs = BlackScholesInputs::gather(self, valuation_time, risk_factors);
        let (d1, _) = get_d1_and_d2(self.strike(), &inputs);
        gaussian().map(|gaussian| {
            0.01 * inputs.dividend_adjusted_price() * inputs.delta_t.sqrt() * gaussian.pdf(d1)
        })
    }
}
