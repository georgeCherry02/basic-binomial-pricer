#[cfg(test)]
use chrono::prelude::Utc;
#[cfg(test)]
use chrono::TimeZone;

#[cfg(test)]
use crate::build_tree::{get_next_layer, TreePosition};
#[cfg(test)]
use crate::result::PricerError;
#[cfg(test)]
use crate::risk_free_model;

use test_log;

use log::error;

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

/*
#[test_log::test]
fn one_year_tree_one_step() {
    let underlying_price: f64 = 100.0;
    let volatility: f64 = 5.0;
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
        tree.map(|node: Node| {
            assert!(node.price == 100.0);
            assert!(node.datetime == begin_date);
            assert!(node.up.is_some());
            node.up.map(|node_up: Box<Node>| {
                assert!(node_up.price == 105.0);
                assert!(node_up.datetime == end_date);
                assert!(node_up.up.is_none());
                assert!(node_up.down.is_none());
            });
            assert!(node.down.is_some());
            node.down.map(|node_down: Box<Node>| {
                assert!(node_down.price == 95.0);
                assert!(node_down.datetime == end_date);
                assert!(node_down.up.is_none());
                assert!(node_down.down.is_none());
            });
        });
    }
}
*/

#[test_log::test]
fn get_next_layer_basic() {
    let start_tree_positions = vec![
        TreePosition {
            num_ups: 1,
            num_downs: 0,
        },
        TreePosition {
            num_ups: 0,
            num_downs: 1,
        },
    ];
    let expected_end_positions = vec![
        TreePosition {
            num_ups: 2,
            num_downs: 0,
        },
        TreePosition {
            num_ups: 1,
            num_downs: 1,
        },
        TreePosition {
            num_ups: 0,
            num_downs: 2,
        },
    ];
    let processed_end_positions = get_next_layer(start_tree_positions);
    assert!(processed_end_positions == expected_end_positions);
}
