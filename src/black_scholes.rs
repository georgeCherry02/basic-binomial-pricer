#[cfg(test)]
use crate::option::{get_call, get_put};
use crate::option::{Call, FinancialOption, Put};
use crate::result::{PricerError, PricerResult};
use crate::utils::date::get_duration_in_years;

use chrono::prelude::Utc;
use chrono::DateTime;
#[cfg(test)]
use chrono::TimeZone;

use statrs::distribution::{ContinuousCDF, Normal};
use statrs::StatsError;

fn failed_to_create_gaussian_error(_: StatsError) -> PricerError {
    PricerError {
        code: 2,
        message: String::from("Failed to construct Gaussian distribution"),
    }
}

fn get_d1_and_d2<O: FinancialOption>(
    option: &O,
    duration_in_years: f64,
    volatility: f64,
    current_underlying_value: f64,
    rfr: f64,
) -> (f64, f64) {
    let ln_val_over_strike = (current_underlying_value / option.strike()).ln();
    let rfr_plus_vol_squared_over_two = rfr + (volatility.powi(2) / 2f64);
    let d1 = (ln_val_over_strike + rfr_plus_vol_squared_over_two * duration_in_years)
        / (volatility * duration_in_years.sqrt());
    let d2 = d1 - volatility * duration_in_years.sqrt();
    (d1, d2)
}

fn calculate_black_scholes<O: FinancialOption>(
    option: &O,
    valuation_func: &dyn Fn(Normal, f64, f64, f64, f64, f64, f64) -> f64, // This is horrifying...
    valuation_time: DateTime<Utc>,
    volatility: f64,
    current_underlying_value: f64,
    rfr: f64,
) -> PricerResult<f64> {
    let duration_in_years = get_duration_in_years(valuation_time, option.expiry());
    let (d1, d2) = get_d1_and_d2(
        option,
        duration_in_years,
        volatility,
        current_underlying_value,
        rfr,
    );
    let curried_func = |n: Normal| -> f64 {
        valuation_func(
            n,
            option.strike(),
            rfr,
            duration_in_years,
            current_underlying_value,
            d1,
            d2,
        )
    };
    Normal::new(0.0, 1.0)
        .map_err(failed_to_create_gaussian_error)
        .map(curried_func)
}

pub trait BlackScholes: FinancialOption {
    fn value_black_scholes(
        &self,
        valuation_time: DateTime<Utc>,
        volatility: f64,
        current_underlying_value: f64,
        rfr: f64,
    ) -> PricerResult<f64>;
}

impl BlackScholes for Call {
    fn value_black_scholes(
        &self,
        valuation_time: DateTime<Utc>,
        volatility: f64,
        current_underlying_value: f64,
        rfr: f64,
    ) -> PricerResult<f64> {
        let evaluate_black_scholes = |n: Normal,
                                      strike: f64,
                                      rfr: f64,
                                      duration_in_years: f64,
                                      current_underlying_value: f64,
                                      d1: f64,
                                      d2: f64|
         -> f64 {
            current_underlying_value * n.cdf(d1)
                - strike * (-rfr * duration_in_years).exp() * n.cdf(d2)
        };
        calculate_black_scholes(
            self,
            &evaluate_black_scholes,
            valuation_time,
            volatility,
            current_underlying_value,
            rfr,
        )
    }
}

impl BlackScholes for Put {
    fn value_black_scholes(
        &self,
        valuation_time: DateTime<Utc>,
        volatility: f64,
        current_underlying_value: f64,
        rfr: f64,
    ) -> PricerResult<f64> {
        let evaluate_black_scholes = |n: Normal,
                                      strike: f64,
                                      rfr: f64,
                                      duration_in_years: f64,
                                      current_underlying_value: f64,
                                      d1: f64,
                                      d2: f64|
         -> f64 {
            strike * (-rfr * duration_in_years).exp() * n.cdf(-d2)
                - current_underlying_value * n.cdf(-d1)
        };
        calculate_black_scholes(
            self,
            &evaluate_black_scholes,
            valuation_time,
            volatility,
            current_underlying_value,
            rfr,
        )
    }
}

#[test]
fn half_year_black_scholes_put() {
    let underlying_price = 42f64;
    let strike = 40f64;
    let implied_volatility = 0.2f64;
    let begin_date = Utc.timestamp_millis_opt(1688917143000).unwrap();
    let end_date = Utc.timestamp_millis_opt(1704697100000).unwrap();
    let put = get_put(strike, end_date);
    let rfr = 0.05;
    #[allow(unused_must_use)]
    {
        put.value_black_scholes(begin_date, implied_volatility, underlying_price, rfr)
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
    let implied_volatility = 0.2f64;
    let begin_date = Utc.timestamp_millis_opt(1688917143000).unwrap();
    let end_date = Utc.timestamp_millis_opt(1704697100000).unwrap();
    let call = get_call(strike, end_date);
    let rfr = 0.05;
    #[allow(unused_must_use)]
    {
        call.value_black_scholes(begin_date, implied_volatility, underlying_price, rfr)
            .map(|value| {
                assert!(value > 4.0817);
                assert!(value < 4.0818);
            });
    }
}
