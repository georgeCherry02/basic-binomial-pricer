use chrono::prelude::Utc;
use chrono::DateTime;

use crate::result::PricerResult;
use std::collections::HashMap;

#[derive(Clone, Eq, Hash)]
pub struct TreePosition {
    pub num_ups: usize,
    pub num_downs: usize,
}

impl PartialEq for TreePosition {
    fn eq(&self, other: &Self) -> bool {
        self.num_ups == other.num_ups && self.num_downs == other.num_downs
    }
}

pub struct Node {
    pub price: f64,
    pub datetime: DateTime<Utc>,
    pub up: TreePosition,
    pub down: TreePosition,
}

pub struct Tree {
    pub head: Node,
    nodes: HashMap<TreePosition, Node>,
}

fn new_tree(price: f64, datetime: DateTime<Utc>) -> Tree {
    Tree {
        head: Node {
            price,
            datetime,
            up: TreePosition {
                num_ups: 1,
                num_downs: 0,
            },
            down: TreePosition {
                num_ups: 0,
                num_downs: 1,
            },
        },
        nodes: HashMap::new(),
    }
}

fn get_datetime_range(
    start: DateTime<Utc>,
    end: DateTime<Utc>,
    num_steps: i32,
) -> Vec<DateTime<Utc>> {
    let dur = end - start;
    let diff = dur / num_steps;
    let mut date_range = Vec::new();
    for i in 0..num_steps + 1 {
        date_range.push(start + (diff * i));
    }
    date_range
}

enum Direction {
    DOWN,
    UP,
}

/*
fn make_node(
    price: f64,
    volatility: f64,
    time_slot: usize,
    date_range: &Vec<DateTime<Utc>>,
    dir: Direction,
) -> Option<Box<Node>> {
    let delta = match dir {
        Direction::UP => volatility,
        Direction::DOWN => -volatility,
    };
    let new_price = price + delta;
    date_range
        .get(time_slot)
        .map(|datetime: &DateTime<Utc>| -> Box<Node> {
            Box::new(Node {
                price: new_price,
                datetime: datetime.clone(),
                up: make_node(
                    new_price,
                    volatility,
                    time_slot + 1,
                    date_range,
                    Direction::UP,
                ),
                down: make_node(
                    new_price,
                    volatility,
                    time_slot + 1,
                    date_range,
                    Direction::DOWN,
                ),
            })
        })
}

Currently this is a very rudimentary tree and doesn't have any knowledge of when nodes overlap... which is very inefficient - this needs to be fixed
Additionally this just adds? Clearly wasn't concentrating - this should be a multiplier...
pub fn construct_tree(
    underlying_price: f64,
    vol: f64,
    start: DateTime<Utc>,
    end: DateTime<Utc>,
    num_steps: i32,
) -> PricerResult<Node> {
    let date_range = get_datetime_range(start, end, num_steps);
    Ok(Node {
        price: underlying_price,
        datetime: start,
        up: make_node(underlying_price, vol, 1, &date_range, Direction::UP),
        down: make_node(underlying_price, vol, 1, &date_range, Direction::DOWN),
    })
}
*/

pub fn get_next_layer(tree_positions: Vec<TreePosition>) -> Vec<TreePosition> {
    let mut next_layer: Vec<TreePosition> = tree_positions
        .into_iter()
        .map(|pos: TreePosition| -> TreePosition {
            TreePosition {
                num_ups: pos.num_ups + 1,
                num_downs: pos.num_downs,
            }
        })
        .collect();
    next_layer
        .last()
        .map(|pos: &TreePosition| -> TreePosition {
            TreePosition {
                num_ups: pos.num_ups - 1,
                num_downs: pos.num_downs + 1,
            }
        })
        .map(|true_last: TreePosition| {
            next_layer.push(true_last);
        });
    next_layer
}

fn get_node_value(underlying_price: f64, volatility: f64, position: &TreePosition) -> f64 {
    let multiplier = 1.0 + volatility;
    let up_multi = position.num_ups as f64 * multiplier;
    let down_multi = position.num_downs as f64 * multiplier;
    underlying_price * up_multi / down_multi
}

fn get_node(price: f64, datetime: DateTime<Utc>, volatility: f64, position: &TreePosition) -> Node {
    let price = get_node_value(price, volatility, position);
    let up = TreePosition {
        num_ups: position.num_ups + 1,
        num_downs: position.num_downs,
    };
    let down = TreePosition {
        num_ups: position.num_ups,
        num_downs: position.num_downs + 1,
    };
    Node {
        price,
        datetime,
        up,
        down,
    }
}

pub fn construct_tree(
    underlying_price: f64,
    volatility: f64,
    start: DateTime<Utc>,
    end: DateTime<Utc>,
    num_steps: i32,
) -> PricerResult<Tree> {
    let date_range = get_datetime_range(start, end, num_steps);
    let mut tree = new_tree(underlying_price, start);
    let mut current_layer = vec![
        TreePosition {
            num_ups: 1,
            num_downs: 0,
        },
        TreePosition {
            num_ups: 0,
            num_downs: 1,
        },
    ];
    for datetime in date_range {
        let nodes: Vec<Node> = current_layer
            .iter()
            .map(|position: &TreePosition| {
                get_node(underlying_price, datetime, volatility, position)
            })
            .collect();
        tree.nodes
            .extend(current_layer.clone().into_iter().zip(nodes.into_iter()));
        current_layer = get_next_layer(current_layer);
    }
    let mut tree = new_tree(underlying_price, start);
    Ok(tree)
}
