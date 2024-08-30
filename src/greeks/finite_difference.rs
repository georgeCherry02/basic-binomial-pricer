use crate::result::{PricerError, PricerResult};
use crate::Pricer;

use crate::risk_factors::discount::DiscountFactor;
use crate::risk_factors::{IdentifiableRiskFactor, RiskFactors};

use crate::shock::{absolute_shock, absolute_time_shock};
use crate::shock::{interest_rate_shock, price_shock, time_shock, volatility_shock};
use crate::shock::{Scenario, Shock, ShockDirection};

use chrono::{DateTime, Duration, Utc};

pub trait FiniteDifferenceGreeks {
    fn delta_fd(
        &self,
        valuation_time: DateTime<Utc>,
        risk_factors: RiskFactors,
    ) -> PricerResult<f64>;
    fn rho_fd(&self, valuation_time: DateTime<Utc>, risk_factors: RiskFactors)
        -> PricerResult<f64>;
    fn theta_fd(
        &self,
        valuation_time: DateTime<Utc>,
        risk_factors: RiskFactors,
    ) -> PricerResult<f64>;
    fn vega_fd(
        &self,
        valuation_time: DateTime<Utc>,
        risk_factors: RiskFactors,
    ) -> PricerResult<f64>;
}

fn bump_and_reprice<T: Pricer>(
    option: &T,
    valuation_time: DateTime<Utc>,
    risk_factors: RiskFactors,
    scenario: Scenario,
) -> PricerResult<f64> {
    let base = option
        .value(valuation_time, risk_factors.clone(), vec![])
        .map_err(|e| PricerError::new(format!("Failed base pricing: {}", e), 2))?;
    let shock = option
        .value(valuation_time, risk_factors, scenario)
        .map_err(|e| PricerError::new(format!("Failed shock pricing: {}", e), 3))?;
    Ok(shock - base)
}

impl<T> FiniteDifferenceGreeks for T
where
    T: Pricer,
{
    fn delta_fd(
        &self,
        valuation_time: DateTime<Utc>,
        risk_factors: RiskFactors,
    ) -> PricerResult<f64> {
        let delta_shocks = risk_factors
            .price_sensitivities
            .iter()
            .map(|price| price_shock(price.id().clone(), absolute_shock(1.0, ShockDirection::Up)))
            .collect();
        bump_and_reprice(self, valuation_time, risk_factors, delta_shocks)
    }
    fn rho_fd(
        &self,
        valuation_time: DateTime<Utc>,
        risk_factors: RiskFactors,
    ) -> PricerResult<f64> {
        let rho_shocks = risk_factors
            .discount_factors
            .iter()
            .flat_map(|discount| match discount {
                DiscountFactor::RiskFreeRate(rfr) => Some(interest_rate_shock(
                    rfr.id().clone(),
                    absolute_shock(1.0, ShockDirection::Up),
                )),
                _ => None,
            })
            .collect();
        bump_and_reprice(self, valuation_time, risk_factors, rho_shocks).map(|value| value / 100.0)
    }
    fn theta_fd(
        &self,
        valuation_time: DateTime<Utc>,
        risk_factors: RiskFactors,
    ) -> PricerResult<f64> {
        let day = Duration::days(1);
        let theta: Shock = time_shock(absolute_time_shock(day, ShockDirection::Down));
        bump_and_reprice(self, valuation_time, risk_factors, vec![theta])
    }
    fn vega_fd(
        &self,
        valuation_time: DateTime<Utc>,
        risk_factors: RiskFactors,
    ) -> PricerResult<f64> {
        let vega_shocks = risk_factors
            .volatility_sensitivities
            .iter()
            .map(|vol| volatility_shock(vol.id().clone(), absolute_shock(1.0, ShockDirection::Up)))
            .collect();
        bump_and_reprice(self, valuation_time, risk_factors, vega_shocks).map(|value| value / 100.0)
    }
}
