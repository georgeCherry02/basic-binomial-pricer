#[cfg(test)]
use crate::option::{get_call, get_put};
#[cfg(test)]
use crate::tree::{Tree, build::construct_tree};

#[cfg(test)]
use chrono::prelude::Utc;
#[cfg(test)]
use chrono::Datelike;
#[cfg(test)]
use chrono::TimeZone;

#[test]
fn two_year_basic_put() {
    let underlying_price: f64 = 20.0;
    let strike = 20.0;
    let volatility = 0.2;
    let begin_date = Utc.timestamp_millis_opt(1688917143000).unwrap();
    let number_of_years = 2;
    let end_date = begin_date
        .with_year(begin_date.year() + number_of_years)
        .unwrap();
    let put = get_put(strike, volatility, end_date);
    let num_steps = number_of_years;
    let risk_free_rate = 0.05;
    #[allow(unused_must_use)]
    {
        construct_tree(
            underlying_price,
            volatility,
            begin_date,
            end_date,
            num_steps,
        )
        .map(|mut tree: Tree| {
            let option_value = tree.value(&put, risk_free_rate);
            assert!(option_value > 1.2377);
            assert!(option_value < 1.2378);
        });
    }
}

#[test]
fn two_year_basic_call() {
    let underlying_price: f64 = 20.0;
    let strike = 20.0;
    let volatility = 0.2;
    let begin_date = Utc.timestamp_millis_opt(1688917143000).unwrap();
    let number_of_years = 2;
    let end_date = begin_date
        .with_year(begin_date.year() + number_of_years)
        .unwrap();
    let call = get_call(strike, volatility, end_date);
    let num_steps = number_of_years;
    let risk_free_rate = 0.05;
    #[allow(unused_must_use)]
    {
        construct_tree(
            underlying_price,
            volatility,
            begin_date,
            end_date,
            num_steps,
        )
        .map(|mut tree: Tree| {
            let option_value = tree.value(&call, risk_free_rate);
            assert!(option_value > 3.1434);
            assert!(option_value < 3.1435);
        });
    }
}
