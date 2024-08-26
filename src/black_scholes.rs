#[cfg(test)]
use crate::option::{get_call, get_put};
use crate::option::{Call, FinancialOption, Put};
use crate::result::{PricerError, PricerResult};
use crate::shock::{FloatShock, Scenario, Shock};
use crate::utils::date::get_duration_in_years;

#[cfg(test)]
use chrono::TimeZone;
use chrono::{DateTime, Utc};

use statrs::distribution::{ContinuousCDF, Normal};
use statrs::StatsError;

fn failed_to_create_gaussian_error(_: StatsError) -> PricerError {
    PricerError {
        code: 2,
        message: String::from(
            "Failed to construct Gaussian disrtribution for Black-Scholes pricing",
        ),
    }
}

#[derive(Clone)]
pub struct RiskFactors {
    underlying_price: f64,
    underlying_volatility: f64,
    risk_free_rate: f64,
}

impl RiskFactors {
    pub fn new(
        underlying_price: f64,
        underlying_volatility: f64,
        risk_free_rate: f64,
    ) -> RiskFactors {
        RiskFactors {
            underlying_price,
            underlying_volatility,
            risk_free_rate,
        }
    }
}

pub trait BlackScholes: FinancialOption {
    fn value_black_scholes(
        &self,
        valuation_time: DateTime<Utc>,
        risk_factors: RiskFactors,
        shock_scenarios: Scenario,
    ) -> PricerResult<f64>;
}

struct BlackScholesInputs {
    delta_t: f64,
    underlying_price: f64,
    underlying_volatility: f64,
    risk_free_rate: f64,
}

fn apply_shock(input: &mut BlackScholesInputs, shock: &Shock) {
    match shock {
        Shock::InterestRateShock(shock) => input.risk_free_rate = shock.apply(input.risk_free_rate),
        Shock::PriceShock(shock) => input.underlying_price = shock.apply(input.underlying_price),
        Shock::TimeShock(shock) => input.delta_t = shock.apply(input.delta_t),
        Shock::VolatilityShock(shock) => {
            input.underlying_volatility = shock.apply(input.underlying_volatility)
        }
    };
}

fn apply_scenario(
    delta_t: f64,
    underlying_price: f64,
    underlying_volatility: f64,
    risk_free_rate: f64,
    scenario: Scenario,
) -> BlackScholesInputs {
    let mut input = BlackScholesInputs {
        delta_t,
        underlying_price,
        underlying_volatility,
        risk_free_rate,
    };
    for shock in scenario {
        apply_shock(&mut input, shock);
    }
    return input;
}

fn get_d1_and_d2(strike: f64, inputs: &BlackScholesInputs) -> (f64, f64) {
    let ln_val_over_strike = (inputs.underlying_price / strike).ln();
    let rfr_plus_vol_squared_over_two =
        inputs.risk_free_rate + (inputs.underlying_volatility.powi(2) / 2f64);
    let d1 = (ln_val_over_strike + rfr_plus_vol_squared_over_two * inputs.delta_t)
        / (inputs.underlying_volatility * inputs.delta_t.sqrt());
    let d2 = d1 - inputs.underlying_volatility * inputs.delta_t.sqrt();
    (d1, d2)
}

impl BlackScholes for Call {
    fn value_black_scholes(
        &self,
        valuation_time: DateTime<Utc>,
        risk_factors: RiskFactors,
        scenario: Scenario,
    ) -> PricerResult<f64> {
        let delta_t = get_duration_in_years(valuation_time, self.expiry());
        let shocked_inputs = apply_scenario(
            delta_t,
            risk_factors.underlying_price,
            risk_factors.underlying_volatility,
            risk_factors.risk_free_rate,
            scenario,
        );
        let (d1, d2) = get_d1_and_d2(self.strike(), &shocked_inputs);
        Normal::new(0.0, 1.0)
            .map_err(failed_to_create_gaussian_error)
            .map(|gaussian| {
                shocked_inputs.underlying_price * gaussian.cdf(d1)
                    - self.strike()
                        * (-shocked_inputs.risk_free_rate * shocked_inputs.delta_t).exp()
                        * gaussian.cdf(d2)
            })
            .map(|valuation| valuation - self.cost())
    }
}

impl BlackScholes for Put {
    fn value_black_scholes(
        &self,
        valuation_time: DateTime<Utc>,
        risk_factors: RiskFactors,
        scenario: Scenario,
    ) -> PricerResult<f64> {
        let delta_t = get_duration_in_years(valuation_time, self.expiry());
        let shocked_inputs = apply_scenario(
            delta_t,
            risk_factors.underlying_price,
            risk_factors.underlying_volatility,
            risk_factors.risk_free_rate,
            scenario,
        );
        let (d1, d2) = get_d1_and_d2(self.strike(), &shocked_inputs);
        Normal::new(0.0, 1.0)
            .map_err(failed_to_create_gaussian_error)
            .map(|gaussian| {
                self.strike()
                    * (-shocked_inputs.risk_free_rate * shocked_inputs.delta_t).exp()
                    * gaussian.cdf(-d2)
                    - shocked_inputs.underlying_price * gaussian.cdf(-d1)
            })
            .map(|valuation| valuation - self.cost())
    }
}

#[test]
fn half_year_black_scholes_put() {
    let underlying_price = 42f64;
    let strike = 40f64;
    let cost = 0f64;
    let implied_volatility = 0.2f64;
    let begin_date = Utc.timestamp_millis_opt(1688917143000).unwrap();
    let end_date = Utc.timestamp_millis_opt(1704697100000).unwrap();
    let put = get_put(strike, end_date, cost);
    let rfr = 0.05;
    let risk_factors = RiskFactors::new(underlying_price, implied_volatility, rfr);
    #[allow(unused_must_use)]
    {
        put.value_black_scholes(begin_date, risk_factors, vec![])
            .map(|value| {
                assert!(value > 1.0934);
                assert!(value < 1.0935);
            });
    }
}

#[test]
fn half_year_black_scholes_call() {
    let underlying_price = 42f64;
    let strike = 40f64;
    let cost = 0f64;
    let implied_volatility = 0.2f64;
    let begin_date = Utc.timestamp_millis_opt(1688917143000).unwrap();
    let end_date = Utc.timestamp_millis_opt(1704697100000).unwrap();
    let call = get_call(strike, end_date, cost);
    let rfr = 0.05;
    let risk_factors = RiskFactors::new(underlying_price, implied_volatility, rfr);
    #[allow(unused_must_use)]
    {
        call.value_black_scholes(begin_date, risk_factors, vec![])
            .map(|value| {
                assert!(value > 4.0817);
                assert!(value < 4.0818);
            });
    }
}
