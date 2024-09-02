use super::{MonteCarloInputs, MonteCarloParams};

use crate::option::{Call, FinancialOption, Put};
use crate::result::{make_not_implemented_error, PricerError, PricerResult};

use crate::risk_factors::discount::{DiscountFactor, HistoricReturn};
use crate::risk_factors::price::{Price, PriceTick};
use crate::risk_factors::volatility::{ImpliedVolatility, Volatility};
use crate::risk_factors::RiskFactors;

use crate::shock::{ApplyShock, Scenario};

use chrono::{DateTime, Utc};
use ndarray::{Array, Array2};
use rand::Rng;
use rayon::prelude::*;
use statrs::distribution::Normal;
use statrs::statistics::Statistics;
use statrs::StatsError;

pub trait LongstaffSchwartzMonteCarlo: FinancialOption {
    fn get_monte_carlo_risk_factors(
        &self,
        price: f64,
        volatility: f64,
        historic_return: f64,
    ) -> RiskFactors {
        RiskFactors {
            price_sensitivities: vec![Price::PriceTick(PriceTick::new(
                self.symbol().clone(),
                price,
            ))],
            volatility_sensitivities: vec![Volatility::ImpliedVolatility(ImpliedVolatility::new(
                self.symbol().clone(),
                volatility,
            ))],
            discount_factors: vec![DiscountFactor::HistoricReturn(HistoricReturn::new(
                self.symbol().clone(),
                historic_return,
            ))],
            dividend_sensitivities: vec![],
        }
    }
    fn value_monte_carlo_ls_impl(
        &self,
        inputs: MonteCarloInputs,
        parameters: MonteCarloParams,
    ) -> PricerResult<f64>;
    fn value_monte_carlo_ls(
        &self,
        valuation_time: DateTime<Utc>,
        risk_factors: RiskFactors,
        shock_scenarios: Scenario,
        parameters: MonteCarloParams,
    ) -> PricerResult<f64> {
        risk_factors.try_into().and_then(|risk_factors| {
            let mut inputs = MonteCarloInputs::gather(self.expiry(), valuation_time, risk_factors);
            shock_scenarios.apply(&mut inputs);
            self.value_monte_carlo_ls_impl(inputs, parameters)
        })
    }
    fn generate_monte_carlo_paths(
        &self,
        valuation_time: DateTime<Utc>,
        risk_factors: RiskFactors,
        parameters: MonteCarloParams,
    ) -> PricerResult<Vec<Vec<f64>>> {
        risk_factors.try_into().and_then(|risk_factors| {
            let inputs = MonteCarloInputs::gather(self.expiry(), valuation_time, risk_factors);
            generate_monte_carlo_paths(&inputs, &parameters)
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
    Ok((0..parameters.repetitions)
        .into_par_iter()
        .map(|_| {
            let mut rng = rand::thread_rng();
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

impl LongstaffSchwartzMonteCarlo for Call {
    fn value_monte_carlo_ls_impl(
        &self,
        inputs: MonteCarloInputs,
        parameters: MonteCarloParams,
    ) -> PricerResult<f64> {
        Err(make_not_implemented_error())
    }
}

fn zero_or_more(val: f64) -> f64 {
    if val > 0. {
        val
    } else {
        0.
    }
}

fn vectomatrix(vec: Vec<Vec<f64>>) -> PricerResult<Array2<f64>> {
    let shape = (vec.len(), vec.first().map(|v| v.len()).unwrap_or(0));
    Ok(
        Array::from_shape_vec(shape, vec.into_iter().flatten().collect())
            .map_err(|_| PricerError::new("f".into(), 1))?,
    )
}

impl LongstaffSchwartzMonteCarlo for Put {
    fn value_monte_carlo_ls_impl(
        &self,
        inputs: MonteCarloInputs,
        parameters: MonteCarloParams,
    ) -> PricerResult<f64> {
        let mut paths = generate_monte_carlo_paths(&inputs, &parameters)?;
        paths
            .iter_mut()
            .for_each(|path: &mut Vec<f64>| path.insert(0, inputs.price()));

        let dt = inputs.delta_t / parameters.steps as f64;
        let step_discount = (-dt * inputs.discount_rate()).exp();

        let mut payoffs: Vec<f64> = paths
            .iter()
            .map(|path| path.last().unwrap_or(&0.))
            .map(|final_value| self.strike() - final_value)
            .map(zero_or_more)
            .collect();

        for step in (0..parameters.steps - 1).rev() {
            let in_the_money: Vec<bool> = payoffs.iter().map(|v| v > &0.).collect();
            let A: Vec<Vec<f64>> = paths
                .iter()
                .map(|path| path[step])
                .map(|s_val| (0..3).map(|pow| s_val.powi(pow)).collect())
                .collect();
            let B: Vec<Vec<f64>> = A
                .iter()
                .zip(in_the_money.iter())
                .filter(|(_, is_itm)| **is_itm)
                .map(|(a_row, _)| a_row.clone())
                .collect();
            let mat_a = vectomatrix(A)?;
            let mat_b = vectomatrix(B)?;
            let cv = mat_a.dot(&mat_b.t());

            for p in 0..parameters.repetitions {
                let exercise_value = zero_or_more(self.strike() - paths[p][step]);
                if in_the_money[p] && cv[[p, 0]] < exercise_value {
                    payoffs[p] = exercise_value * step_discount;
                } else {
                    payoffs[p] = payoffs[p] * step_discount;
                }
            }
        }

        let immediate_exercise = self.strike() - inputs.price();
        let expected_value = payoffs.mean();
        Ok(if expected_value > immediate_exercise {
            expected_value
        } else {
            immediate_exercise
        })
    }
}
