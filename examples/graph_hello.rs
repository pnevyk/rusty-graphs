#![allow(unused_variables)]

use graph::prelude::*;

fn main() {
    let graph: UndirectedCsrGraph<usize, i32> = GraphBuilder::new()
        .edges(vec![(0, 1), (0, 4), (1, 2), (1, 4), (2, 3), (3, 4), (3, 5)])
        .node_values(1..=6)
        .build();
}
