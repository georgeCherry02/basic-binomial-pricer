use chrono::prelude::Utc;
use chrono::DateTime;

use crate::option::Call;
use crate::result::PricerResult;

use std::collections::HashMap;

use log::{debug, info};

#[derive(Clone, Debug, Eq, Hash)]
pub struct TreePosition {
    pub num_ups: usize,
    pub num_downs: usize,
}

impl PartialEq for TreePosition {
    fn eq(&self, other: &Self) -> bool {
        self.num_ups == other.num_ups && self.num_downs == other.num_downs
    }
}

#[derive(Debug, Clone)]
pub struct Node {
    pub price: f64,
    pub datetime: DateTime<Utc>,
    pub pos: TreePosition,
    pub up: TreePosition,
    pub down: TreePosition,
}

#[derive(Debug)]
pub struct Tree {
    pub head: Node,
    pub nodes: HashMap<TreePosition, Node>,
    pub valuation_cache: HashMap<TreePosition, f64>,
}

const EULERS_NUMBER: f64 = std::f64::consts::E;

fn get_duration_in_years(t1: DateTime<Utc>, t2: DateTime<Utc>) -> f64 {
    let diff: chrono::Duration = t2 - t1;
    let diff_in_secs: i64 = diff.num_seconds();
    const NUMBER_OF_SECONDS_IN_A_YEAR: f64 = 31536000.0;
    diff_in_secs as f64 / NUMBER_OF_SECONDS_IN_A_YEAR
}

#[allow(dead_code)]
impl Tree {
    fn get_child_nodes(&self, node: &Node) -> Option<(Node, Node)> {
        let up_node = self.nodes.get(&node.up);
        if up_node.is_none() {
            return None;
        }
        self.nodes
            .get(&node.down)
            .map(|down_node: &Node| (up_node.unwrap().clone(), down_node.clone()))
    }

    fn calculate_node(&mut self, node: &Node, call: &Call, rfr: f64) -> f64 {
        let value = self
            .get_child_nodes(node)
            .map(|(up_node, down_node)| -> f64 {
                let duration = get_duration_in_years(node.datetime, up_node.datetime);
                let u = up_node.price / node.price;
                let d = down_node.price / node.price;
                debug!("Found u={}, d={}", u, d);
                let p = (EULERS_NUMBER.powf(rfr * duration) - d) / (u - d);
                debug!("Found p={}", p);
                let up_value = self.value_node(&up_node, call, rfr);
                let down_value = self.value_node(&down_node, call, rfr);
                EULERS_NUMBER.powf(-rfr * duration) * ((p * up_value) + ((1.0f64 - p) * down_value))
            })
            .unwrap_or(0.0f64.max(call.strike - node.price));
        self.valuation_cache.insert(node.pos.clone(), value);
        debug!(
            "Calculated u={}, d={}, v={}",
            node.pos.num_ups, node.pos.num_downs, value
        );
        value
    }
    fn value_node(&mut self, node: &Node, call: &Call, rfr: f64) -> f64 {
        self.valuation_cache
            .get(&node.pos)
            .map(|f| f.clone())
            .unwrap_or(self.calculate_node(node, call, rfr))
    }
    pub fn value(&mut self, call: &Call, rfr: f64) -> f64 {
        self.value_node(&self.head.clone(), call, rfr)
    }
}

fn new_tree(price: f64, datetime: DateTime<Utc>) -> Tree {
    Tree {
        head: Node {
            price,
            datetime,
            pos: TreePosition {
                num_ups: 0,
                num_downs: 0,
            },
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
        valuation_cache: HashMap::new(),
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
    let up_multi = f64::powf(1.0 + volatility, position.num_ups as f64);
    let down_multi = f64::powf(1.0 - volatility, position.num_downs as f64);
    underlying_price * up_multi * down_multi
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
        pos: position.clone(),
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
    let mut current_layer = vec![TreePosition {
        num_ups: 0,
        num_downs: 0,
    }];
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
    Ok(tree)
}

fn get_tree_depth(tree: &Tree) -> usize {
    let mut count = 0;
    let mut it = &tree.head;
    while tree.nodes.get(&it.up).is_some() {
        it = tree.nodes.get(&it.up).unwrap();
        count += 1;
    }
    return count + 1;
}

fn print_layer(tree: &Tree, layer: &Vec<TreePosition>, expected_width: &usize) {
    let mut out = String::from("[");
    let num_in_layer = layer.len();
    let expected_node_width = num_in_layer * 5;
    let expected_node_spacing = (num_in_layer - 1) * 3;
    let padding = (expected_width - expected_node_width - expected_node_spacing) / 2;
    let mut i = 0;
    while i < padding {
        out += " ";
        i += 1;
    }
    i = 0;
    for p in layer {
        tree.nodes.get(p).map(|n: &Node| {
            out += &format!("{:.3}", n.price);
            if i < layer.len() - 1 {
                out += "   ";
            }
            i += 1;
        });
    }
    i = 0;
    while i < padding {
        out += " ";
        i += 1;
    }
    out += "]";
    info!("{}", out);
}

pub fn print_tree(tree: &Tree) {
    let tree_depth = get_tree_depth(tree);
    let cumulative_node_width = tree_depth * 5;
    let cumulative_node_spacing_width = (tree_depth - 1) * 3;
    let expected_width = cumulative_node_width + cumulative_node_spacing_width;
    let mut current_layer = vec![TreePosition {
        num_ups: 0,
        num_downs: 0,
    }];
    let mut count = 0;
    while count < tree_depth {
        print_layer(&tree, &current_layer, &expected_width);
        count += 1;
        current_layer = get_next_layer(current_layer);
    }
}
