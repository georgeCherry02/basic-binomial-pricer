pub mod build;
pub mod debug;
pub mod node;

use node::{Node, Position};

use crate::option::FinancialOption;
use crate::utils::date as date_utils;

use log::debug;

use std::collections::HashMap;

const EULERS_NUMBER: f64 = std::f64::consts::E;

#[derive(Debug)]
pub struct Tree {
    pub head: Node,
    pub nodes: HashMap<Position, Node>,
    pub valuation_cache: HashMap<Position, f64>,
}

impl Tree {
    fn get_child_nodes(&self, node: &Node) -> Option<(Node, Node)> {
        let (up_pos, down_pos) = node.pos.get_branches();
        let up_node = self.nodes.get(&up_pos);
        if up_node.is_none() {
            return None;
        }
        self.nodes
            .get(&down_pos)
            .map(|down_node: &Node| (up_node.unwrap().clone(), down_node.clone()))
    }

    fn calculate_node<O: FinancialOption>(&mut self, node: &Node, option: &O, rfr: f64) -> f64 {
        let value = self
            .get_child_nodes(node)
            .map(|(up_node, down_node)| -> f64 {
                let duration = date_utils::get_duration_in_years(node.datetime, up_node.datetime);
                let u = up_node.price / node.price;
                let d = down_node.price / node.price;
                debug!("Found u={}, d={}", u, d);
                let p = (EULERS_NUMBER.powf(rfr * duration) - d) / (u - d);
                debug!("Found p={}", p);
                let up_value = self.value_node(&up_node, option, rfr);
                let down_value = self.value_node(&down_node, option, rfr);
                EULERS_NUMBER.powf(-rfr * duration) * ((p * up_value) + ((1.0f64 - p) * down_value))
            })
            .unwrap_or(0.0f64.max(option.value_if_executed(node.price)));
        self.valuation_cache.insert(node.pos.clone(), value);
        debug!(
            "Calculated u={}, d={}, v={}",
            node.pos.num_ups, node.pos.num_downs, value
        );
        value
    }
    fn value_node<O: FinancialOption>(&mut self, node: &Node, option: &O, rfr: f64) -> f64 {
        self.valuation_cache
            .get(&node.pos)
            .map(|f| f.clone())
            .unwrap_or(self.calculate_node(node, option, rfr))
    }
    pub fn value<O: FinancialOption>(&mut self, option: &O, rfr: f64) -> f64 {
        self.value_node(&self.head.clone(), option, rfr)
    }
}
