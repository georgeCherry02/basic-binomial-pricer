use crate::tree::Tree;
use crate::tree::node::{Node, Position};

use crate::utils::date as date_utils;
use crate::result::PricerResult;

use chrono::prelude::Utc;
use chrono::DateTime;

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
            .map(|position: &Position| {
                get_node(underlying_price, datetime, volatility, position)
            })
            .collect();
        tree.nodes
            .extend(current_layer.clone().into_iter().zip(nodes.into_iter()));
        current_layer = get_next_layer(current_layer);
    }
    Ok(tree)
}
