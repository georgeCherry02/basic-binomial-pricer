use super::common::{gaussian, get_d1_and_d2};
use super::types::BlackScholesInputs;
use crate::option::{Call, FinancialOption, Put};
use crate::result::PricerResult;
use crate::risk_factor::RiskFactors;
use crate::shock::{FloatShock, Scenario, Shock};

use chrono::{DateTime, Utc};

// PDF under `Continuous`
use statrs::distribution::ContinuousCDF;

pub trait BlackScholes: FinancialOption {
    fn value_black_scholes(
        &self,
        valuation_time: DateTime<Utc>,
        risk_factors: RiskFactors,
        shock_scenarios: Scenario,
    ) -> PricerResult<f64>;
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

fn apply_scenario(mut input: BlackScholesInputs, scenario: Scenario) -> BlackScholesInputs {
    for shock in scenario {
        apply_shock(&mut input, shock);
    }
    return input;
}

impl BlackScholes for Call {
    fn value_black_scholes(
        &self,
        valuation_time: DateTime<Utc>,
        risk_factors: RiskFactors,
        scenario: Scenario,
    ) -> PricerResult<f64> {
        let inputs = BlackScholesInputs::gather(self, valuation_time, risk_factors);
        let shocked_inputs = apply_scenario(inputs, scenario);
        let (d1, d2) = get_d1_and_d2(self.strike(), &shocked_inputs);
        gaussian()
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
        let inputs = BlackScholesInputs::gather(self, valuation_time, risk_factors);
        let shocked_inputs = apply_scenario(inputs, scenario);
        let (d1, d2) = get_d1_and_d2(self.strike(), &shocked_inputs);
        gaussian()
            .map(|gaussian| {
                self.strike()
                    * (-shocked_inputs.risk_free_rate * shocked_inputs.delta_t).exp()
                    * gaussian.cdf(-d2)
                    - shocked_inputs.underlying_price * gaussian.cdf(-d1)
            })
            .map(|valuation| valuation - self.cost())
    }
}
