use crate::tree::build::get_next_layer;
use crate::tree::node::{Node, Position};
use crate::tree::Tree;

use log::info;

fn get_up_node(node: &Node) -> Position {
    Position {
        num_ups: node.pos.num_ups + 1,
        num_downs: node.pos.num_downs,
    }
}

fn get_tree_depth(tree: &Tree) -> usize {
    let mut count = 0;
    let mut it = &tree.head;
    while tree.nodes.get(&get_up_node(it)).is_some() {
        it = tree.nodes.get(&get_up_node(it)).unwrap();
        count += 1;
    }
    return count + 1;
}

fn print_layer(tree: &Tree, layer: &Vec<Position>, expected_width: &usize) {
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

#[allow(dead_code)]
pub fn print_tree(tree: &Tree) {
    let tree_depth = get_tree_depth(tree);
    let cumulative_node_width = tree_depth * 5;
    let cumulative_node_spacing_width = (tree_depth - 1) * 3;
    let expected_width = cumulative_node_width + cumulative_node_spacing_width;
    let mut current_layer = vec![Position {
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
