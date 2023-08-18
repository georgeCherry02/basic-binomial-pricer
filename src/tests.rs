#[cfg(test)]
use chrono::prelude::Utc;
#[cfg(test)]
use chrono::Datelike;
#[cfg(test)]
use chrono::TimeZone;

#[cfg(test)]
use crate::build_tree::{construct_tree, get_next_layer, value_tree, Node, Tree, TreePosition};
#[cfg(test)]
use crate::option::Call;
#[cfg(test)]
use crate::risk_free_model;

use test_log;

#[cfg(test)]
use log::info;

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
                node.up
                    == TreePosition {
                        num_ups: 1,
                        num_downs: 0
                    }
            );
            assert!(
                node.down
                    == TreePosition {
                        num_ups: 0,
                        num_downs: 1
                    }
            );
            assert!(tree.nodes.len() == 3);
            assert!(tree.nodes.get(&node.up).is_some());
            tree.nodes.get(&node.up).map(|node: &Node| {
                assert!(node.price == 105.00);
                assert!(node.datetime == end_date);
                assert!(
                    node.up
                        == TreePosition {
                            num_ups: 2,
                            num_downs: 0
                        }
                );
                assert!(
                    node.down
                        == TreePosition {
                            num_ups: 1,
                            num_downs: 1
                        }
                );
            });
            tree.nodes.get(&node.down).map(|node: &Node| {
                assert!(node.price > 94.999);
                assert!(node.price < 95.001);
                assert!(node.datetime == end_date);
                assert!(
                    node.up
                        == TreePosition {
                            num_ups: 1,
                            num_downs: 1
                        }
                );
                assert!(
                    node.down
                        == TreePosition {
                            num_ups: 0,
                            num_downs: 2
                        }
                );
            });
            assert!(tree.nodes.get(&node.down).is_some());
        });
    }
}

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

#[test_log::test]
fn one_year_basic_put() {
    let underlying_price: f64 = 20.0;
    let strike = 20.0;
    let volatility = 0.2;
    let begin_date = Utc.timestamp_millis_opt(1688917143000).unwrap();
    let number_of_years = 2;
    let end_date = begin_date
        .with_year(begin_date.year() + number_of_years)
        .unwrap();
    let call = Call {
        strike,
        volatility,
        expiry: end_date,
    };
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
        .map(|tree: Tree| {
            let option_value = value_tree(&tree, &call, risk_free_rate);
            info!("Found value={}", option_value);
            assert!(option_value > 1.2377);
            assert!(option_value < 1.2378);
        });
    }
}
