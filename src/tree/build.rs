use crate::tree::node::{Node, Position};
use crate::tree::Tree;

use crate::result::PricerResult;
use crate::utils::date as date_utils;

use chrono::prelude::Utc;
use chrono::DateTime;
#[cfg(test)]
use chrono::TimeZone;

use std::collections::HashMap;

fn new_tree(price: f64, datetime: DateTime<Utc>) -> Tree {
    Tree {
        head: Node {
            price,
            datetime,
            pos: Position {
                num_ups: 0,
                num_downs: 0,
            },
        },
        nodes: HashMap::new(),
        valuation_cache: HashMap::new(),
    }
}

pub fn get_next_layer(tree_positions: Vec<Position>) -> Vec<Position> {
    let mut next_layer: Vec<Position> = tree_positions
        .into_iter()
        .map(|pos: Position| -> Position {
            Position {
                num_ups: pos.num_ups + 1,
                num_downs: pos.num_downs,
            }
        })
        .collect();
    next_layer
        .last()
        .map(|pos: &Position| -> Position {
            Position {
                num_ups: pos.num_ups - 1,
                num_downs: pos.num_downs + 1,
            }
        })
        .map(|true_last: Position| {
            next_layer.push(true_last);
        });
    next_layer
}

fn get_node_value(underlying_price: f64, volatility: f64, position: &Position) -> f64 {
    let up_multi = f64::powf(1.0 + volatility, position.num_ups as f64);
    let down_multi = f64::powf(1.0 - volatility, position.num_downs as f64);
    underlying_price * up_multi * down_multi
}

fn get_node(price: f64, datetime: DateTime<Utc>, volatility: f64, position: &Position) -> Node {
    let price = get_node_value(price, volatility, position);
    Node {
        price,
        datetime,
        pos: position.clone(),
    }
}

pub fn construct_tree(
    underlying_price: f64,
    volatility: f64,
    start: DateTime<Utc>,
    end: DateTime<Utc>,
    num_steps: i32,
) -> PricerResult<Tree> {
    let date_range = date_utils::get_datetime_range(start, end, num_steps);
    let mut tree = new_tree(underlying_price, start);
    let mut current_layer = vec![Position {
        num_ups: 0,
        num_downs: 0,
    }];
    for datetime in date_range {
        let nodes: Vec<Node> = current_layer
            .iter()
            .map(|position: &Position| get_node(underlying_price, datetime, volatility, position))
            .collect();
        tree.nodes
            .extend(current_layer.clone().into_iter().zip(nodes.into_iter()));
        current_layer = get_next_layer(current_layer);
    }
    Ok(tree)
}

#[test]
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

#[test]
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
