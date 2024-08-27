use super::BlackScholes;

use crate::greeks::FiniteDifferenceGreeks;
use crate::result::{PricerError, PricerResult};
use crate::risk_factor::RiskFactors;
use crate::shock::{absolute_shock, absolute_time_shock, relative_percentage_shock};
use crate::shock::{interest_rate_shock, price_shock, time_shock, volatility_shock};
use crate::shock::{Scenario, Shock, ShockDirection};

use chrono::{DateTime, Duration, Utc};

static DELTA_SHOCK: Shock = price_shock(String::new(), absolute_shock(1.0, ShockDirection::Up));
static RHO_SHOCK: Shock =
    interest_rate_shock(String::new(), absolute_shock(1.0, ShockDirection::Up));
static VEGA_SHOCK: Shock = volatility_shock(
    String::new(),
    relative_percentage_shock(1.0, ShockDirection::Up),
);

fn bump_and_reprice<T: BlackScholes>(
    option: &T,
    valuation_time: DateTime<Utc>,
    risk_factors: RiskFactors,
    scenario: Scenario,
) -> PricerResult<f64> {
    let base = option
        .value_black_scholes(valuation_time, risk_factors.clone(), vec![])
        .map_err(|e| PricerError::new(format!("Failed base pricing: {}", e), 2))?;
    let shock = option
        .value_black_scholes(valuation_time, risk_factors, scenario)
        .map_err(|e| PricerError::new(format!("Failed shock pricing: {}", e), 3))?;
    Ok(shock - base)
}

impl<T> FiniteDifferenceGreeks for T
where
    T: BlackScholes,
{
    fn delta(&self, valuation_time: DateTime<Utc>, risk_factors: RiskFactors) -> PricerResult<f64> {
        bump_and_reprice(self, valuation_time, risk_factors, vec![&DELTA_SHOCK])
    }
    fn rho(&self, valuation_time: DateTime<Utc>, risk_factors: RiskFactors) -> PricerResult<f64> {
        bump_and_reprice(self, valuation_time, risk_factors, vec![&RHO_SHOCK])
    }
    fn theta(&self, valuation_time: DateTime<Utc>, risk_factors: RiskFactors) -> PricerResult<f64> {
        let day = Duration::days(1);
        let theta: Shock = time_shock(
            String::from("time-to-expiry"),
            absolute_time_shock(day, ShockDirection::Up),
        );
        bump_and_reprice(self, valuation_time, risk_factors, vec![&theta])
    }
    fn vega(&self, valuation_time: DateTime<Utc>, risk_factors: RiskFactors) -> PricerResult<f64> {
        bump_and_reprice(self, valuation_time, risk_factors, vec![&VEGA_SHOCK])
    }
}
