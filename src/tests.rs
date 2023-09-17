#[cfg(test)]
use chrono::prelude::Utc;
#[cfg(test)]
use chrono::Datelike;
#[cfg(test)]
use chrono::TimeZone;

#[cfg(test)]
use crate::black_scholes::BlackScholes;
#[cfg(test)]
use crate::option::{get_call, get_put};
#[cfg(test)]
use crate::risk_free_model;
#[cfg(test)]
use crate::tree::build::{construct_tree, get_next_layer};
#[cfg(test)]
use crate::tree::node::{Node, Position};
#[cfg(test)]
use crate::tree::Tree;

#[cfg(test)]
use log::info;

use test_log;

#[test_log::test]
fn one_year_forward_test() {
    let rfm = risk_free_model::get_annualised_risk_free_rate(5.0);
    let start: f64 = 100.0;
    let begin_date = Utc.timestamp_millis_opt(1688917143000).unwrap();
    let end_date = Utc.timestamp_millis_opt(1720539543000).unwrap();
    let ret = rfm.apply(start, begin_date, end_date);
    let upper_bound: f64 = 105.02;
    let lower_bound: f64 = 105.01;
    assert!(ret < upper_bound);
    assert!(ret > lower_bound);
}

#[test_log::test]
fn one_year_backward_test() {
    let rfm = risk_free_model::get_annualised_risk_free_rate(5.0);
    let start: f64 = 105.0;
    let begin_date = Utc.timestamp_millis_opt(1720539543000).unwrap();
    let end_date = Utc.timestamp_millis_opt(1688917143000).unwrap();
    let ret = rfm.apply(start, begin_date, end_date);
    let upper_bound: f64 = 99.99;
    let lower_bound: f64 = 99.98;
    assert!(ret < upper_bound);
    assert!(ret > lower_bound);
}

#[test_log::test]
fn one_year_tree_one_step() {
    let underlying_price: f64 = 100.0;
    let volatility: f64 = 0.05;
    let begin_date = Utc.timestamp_millis_opt(1688917143000).unwrap();
    let end_date = Utc.timestamp_millis_opt(1720539543000).unwrap();
    let num_steps = 1;

    let tree = construct_tree(
        underlying_price,
        volatility,
        begin_date,
        end_date,
        num_steps,
    );

    #[allow(unused_must_use)]
    {
        assert!(tree.is_ok());
        tree.map(|tree: Tree| {
            let node = tree.head;
            assert!(node.price == 100.0);
            assert!(node.datetime == begin_date);
            assert!(
                node.pos
                    == Position {
                        num_ups: 0,
                        num_downs: 0
                    }
            );
            assert!(tree.nodes.len() == 3);

            let (up_pos, down_pos) = node.pos.get_branches();
            assert!(tree.nodes.get(&up_pos).is_some());
            tree.nodes.get(&up_pos).map(|node: &Node| {
                assert!(node.price == 105.00);
                assert!(node.datetime == end_date);
                assert!(
                    node.pos
                        == Position {
                            num_ups: 1,
                            num_downs: 0
                        }
                );
            });
            assert!(tree.nodes.get(&down_pos).is_some());
            tree.nodes.get(&down_pos).map(|node: &Node| {
                assert!(node.price > 94.999);
                assert!(node.price < 95.001);
                assert!(node.datetime == end_date);
                assert!(
                    node.pos
                        == Position {
                            num_ups: 0,
                            num_downs: 1
                        }
                );
            });
        });
    }
}

#[test_log::test]
fn get_next_layer_basic() {
    let start_tree_positions = vec![
        Position {
            num_ups: 1,
            num_downs: 0,
        },
        Position {
            num_ups: 0,
            num_downs: 1,
        },
    ];
    let expected_end_positions = vec![
        Position {
            num_ups: 2,
            num_downs: 0,
        },
        Position {
            num_ups: 1,
            num_downs: 1,
        },
        Position {
            num_ups: 0,
            num_downs: 2,
        },
    ];
    let processed_end_positions = get_next_layer(start_tree_positions);
    assert!(processed_end_positions == expected_end_positions);
}

#[test_log::test]
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

#[test_log::test]
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

#[test_log::test]
fn half_year_black_scholes() {
    let underlying_price = 42f64;
    let strike = 40f64;
    let implied_volatility = 0.2f64;
    let begin_date = Utc.timestamp_millis_opt(1688917143000).unwrap();
    let end_date = Utc.timestamp_millis_opt(1704697100000).unwrap();
    let put = get_put(strike, implied_volatility, end_date);
    let rfr = 0.05;
    #[allow(unused_must_use)]
    {
        put.value_black_scholes(begin_date, underlying_price, rfr)
            .and_then(|value| {
                info!("value={}", value);
                Ok(())
            });
    }
    assert!(false);
}
