use chrono::prelude::Utc;
use chrono::DateTime;

use crate::result::PricerResult;
use std::collections::HashMap;

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
    price: f64,
    datetime: DateTime<Utc>,
    up: TreePosition,
    down: TreePosition,
}

pub struct Tree {
    head: Node,
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

pub fn construct_tree(
    underlying_price: f64,
    volatility: f64,
    start: DateTime<Utc>,
    end: DateTime<Utc>,
    num_steps: i32,
) -> PricerResult<Tree> {
    let date_range = get_datetime_range(start, end, num_steps);
    let mut next_tree_positions = [
        TreePosition {
            num_ups: 1,
            num_downs: 0,
        },
        TreePosition {
            num_ups: 0,
            num_downs: 1,
        },
    ];
    for date in date_range {}
    let mut tree = new_tree(underlying_price, start);
    Ok(tree)
}
