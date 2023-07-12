use chrono::prelude::Utc;
use chrono::DateTime;

use crate::result::PricerResult;

use log::info;

pub struct Node {
    pub price: f64,
    pub datetime: DateTime<Utc>,
    pub up: Option<Box<Node>>,
    pub down: Option<Box<Node>>,
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

"""
Currently this is a very rudimentary tree and doesn't have any knowledge of when nodes overlap... which is very inefficient - this needs to be fixed
Additionally this just adds? Clearly wasn't concentrating - this should be a multiplier...
"""
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
