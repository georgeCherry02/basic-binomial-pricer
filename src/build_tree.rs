use chrono::prelude::Utc;
use chrono::DateTime;

use crate::option::Call;
use crate::result::{make_not_implemented_error, PricerResult};

pub struct Node {
    underlying_price: f64,
    time: DateTime<Utc>,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

fn get_datetime_range(
    start: DateTime<Utc>,
    end: DateTime<Utc>,
    num_steps: i32,
) -> Vec<DateTime<Utc>> {
    let dur = end - start;
    let diff = dur / num_steps;
    let mut date_range = Vec::new();
    for i in 1..num_steps {
        date_range.push(start + (diff * i));
    }
    date_range
}

pub fn construct_tree(
    call: Call,
    start: DateTime<Utc>,
    end: DateTime<Utc>,
    num_steps: i32,
) -> PricerResult<Node> {
    let date_range = get_datetime_range(start, end, num_steps);
    Err(make_not_implemented_error())
}
