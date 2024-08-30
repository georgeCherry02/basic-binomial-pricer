use super::BlackScholesInputs;

use crate::result::{PricerError, PricerResult};

use statrs::distribution::Normal;
use statrs::StatsError;

pub fn get_d1_and_d2(strike: f64, inputs: &BlackScholesInputs) -> (f64, f64) {
    let ln_val_over_strike = (inputs.price() / strike).ln();
    let rfr_minus_dividends_plus_vol_squared_over_two = inputs.discount_rate()
        - inputs.annualised_dividend_rate()
        + (inputs.volatility().powi(2) / 2f64);
    let d1 = (ln_val_over_strike + rfr_minus_dividends_plus_vol_squared_over_two * inputs.delta_t)
        / inputs.volatility_for_delta_t();
    let d2 = d1 - inputs.volatility_for_delta_t();
    (d1, d2)
}

fn failed_to_create_gaussian_error(_: StatsError) -> PricerError {
    PricerError {
        code: 2,
        message: String::from(
            "Failed to construct Gaussian disrtribution for Black-Scholes pricing",
        ),
    }
}

pub fn gaussian() -> PricerResult<Normal> {
    Normal::new(0.0, 1.0).map_err(failed_to_create_gaussian_error)
}
