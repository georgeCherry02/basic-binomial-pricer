use super::types::{MonteCarloInputs, MonteCarloParams};

use crate::option::{Call, Put, FinancialOption};
use crate::result::{PricerError, PricerResult};
use crate::risk_factor::RiskFactors;

use chrono::{DateTime, Utc};
use rand::Rng;
use statrs::distribution::Normal;
use statrs::StatsError;

pub trait MonteCarlo: FinancialOption {
    fn value_monte_carlo(
        &self,
        valuation_time: DateTime<Utc>,
        risk_factors: RiskFactors,
        parameters: MonteCarloParams,
    ) -> PricerResult<f64>;
}

fn failed_to_create_gaussian_error(_: StatsError) -> PricerError {
    PricerError {
        code: 2,
        message: String::from(
            "Failed to construct Gaussian disrtribution for Black-Scholes pricing",
        ),
    }
}

fn gaussian() -> PricerResult<Normal> {
    Normal::new(0.0, 1.0).map_err(failed_to_create_gaussian_error)
}

pub fn generate_monte_carlo_paths(
    inputs: &MonteCarloInputs,
    parameters: &MonteCarloParams,
) -> PricerResult<Vec<Vec<f64>>> {
    let dt = inputs.delta_t / parameters.steps as f64;
    let nudt =
        (inputs.annualised_historic_return - 0.5 * inputs.underlying_volatility.powi(2)) * dt;
    let sidt = inputs.underlying_volatility * dt.sqrt();

    let gaussian = gaussian()?;
    let mut rng = rand::thread_rng();
    Ok((0..parameters.repetitions)
        .map(|_| {
            (0..parameters.steps)
                .map(|_| rng.sample(gaussian))
                .map(|sample| (nudt + sidt * sample).exp())
                .scan(inputs.underlying_price, |acc, v| {
                    *acc = *acc * v;
                    Some(*acc)
                })
                .collect()
        })
        .collect())
}

impl MonteCarlo for Call {
    fn value_monte_carlo(
        &self,
        valuation_time: DateTime<Utc>,
        risk_factors: RiskFactors,
        parameters: MonteCarloParams,
    ) -> PricerResult<f64> {
        let inputs = MonteCarloInputs::gather(self, valuation_time, risk_factors);
        let paths = generate_monte_carlo_paths(&inputs, &parameters)?;
        let payoffs = paths
            .iter()
            .flat_map(|path| path.last())
            .map(|value| value - self.strike())
            .map(|value| if value > 0. { value } else { 0. });
        let expected_payoff = payoffs.sum::<f64>() / parameters.repetitions as f64;
        Ok(expected_payoff * inputs.historic_return_discount())
    }
}

impl MonteCarlo for Put {
    fn value_monte_carlo(
        &self,
        valuation_time: DateTime<Utc>,
        risk_factors: RiskFactors,
        parameters: MonteCarloParams,
    ) -> PricerResult<f64> {
        let inputs = MonteCarloInputs::gather(self, valuation_time, risk_factors);
        let paths = generate_monte_carlo_paths(&inputs, &parameters)?;
        let payoffs = paths
            .iter()
            .flat_map(|path| path.last())
            .map(|value| self.strike() - value)
            .map(|value| if value > 0. { value } else { 0. });
        let expected_payoff = payoffs.sum::<f64>() / parameters.repetitions as f64;
        Ok(expected_payoff * inputs.historic_return_discount())
    }
}
