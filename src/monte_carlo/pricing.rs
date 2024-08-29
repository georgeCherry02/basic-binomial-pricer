use super::types::{MonteCarloInputs, MonteCarloParams, MonteCarloRiskFactors};

use crate::option::{Call, FinancialOption, Put};
use crate::result::{PricerError, PricerResult};

use crate::risk_factors::RiskFactors;
use crate::risk_factors::price::{Price, PriceTick};
use crate::risk_factors::volatility::{Volatility, ImpliedVolatility};
use crate::risk_factors::discount::{DiscountFactor, HistoricReturn};

use chrono::{DateTime, Utc};
use rand::Rng;
use statrs::distribution::Normal;
use statrs::StatsError;

pub trait MonteCarlo: FinancialOption {
    fn get_mc_risk_factors(&self, price: f64, volatility: f64, historic_return: f64) -> RiskFactors {
        RiskFactors {
            price_sensitivities: vec![Price::PriceTick(PriceTick::new(self.symbol().clone(), price))],
            volatility_sensitivities: vec![Volatility::ImpliedVolatility(ImpliedVolatility::new(self.symbol().clone(), volatility))],
            discount_factors: vec![DiscountFactor::HistoricReturn(HistoricReturn::new(self.symbol().clone(), historic_return))],
            dividend_sensitivities: vec![]
        }
    }
    fn value_monte_carlo_impl(
        &self,
        valuation_time: DateTime<Utc>,
        risk_factors: MonteCarloRiskFactors,
        parameters: MonteCarloParams,
    ) -> PricerResult<f64>;
    fn value_monte_carlo(
        &self,
        valuation_time: DateTime<Utc>,
        risk_factors: RiskFactors,
        parameters: MonteCarloParams,
    ) -> PricerResult<f64> {
        risk_factors.try_into().and_then(|risk_factors| {
            self.value_monte_carlo_impl(valuation_time, risk_factors, parameters)
        })
    }
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
    let nudt = (inputs.discount_rate() - 0.5 * inputs.volatility().powi(2)) * dt;
    let sidt = inputs.volatility() * dt.sqrt();

    let gaussian = gaussian()?;
    let mut rng = rand::thread_rng();
    Ok((0..parameters.repetitions)
        .map(|_| {
            (0..parameters.steps)
                .map(|_| rng.sample(gaussian))
                .map(|sample| (nudt + sidt * sample).exp())
                .scan(inputs.price(), |acc, v| {
                    *acc = *acc * v;
                    Some(*acc)
                })
                .collect()
        })
        .collect())
}

impl MonteCarlo for Call {
    fn value_monte_carlo_impl(
        &self,
        valuation_time: DateTime<Utc>,
        risk_factors: MonteCarloRiskFactors,
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
        Ok(inputs.discount(expected_payoff))
    }
}

impl MonteCarlo for Put {
    fn value_monte_carlo_impl(
        &self,
        valuation_time: DateTime<Utc>,
        risk_factors: MonteCarloRiskFactors,
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
        Ok(inputs.discount(expected_payoff))
    }
}
