use std::time::Instant;

use graph::prelude::*;
use rusty_graphs::Package;

fn main() {
    let packages = rusty_graphs::load_tree();

    let started = Instant::now();

    let mut edges = Vec::with_capacity(packages.len() * 5);
    for i in 0..packages.len() {
        let src = &packages[i];

        #[allow(clippy::needless_range_loop)]
        for j in 0..packages.len() {
            let dst = &packages[j];

            if dst.has_dependency(src) {
                edges.push((i, j));
            }
        }
    }

    let graph: DirectedCsrGraph<usize, &Package> = GraphBuilder::new()
        .edges(edges)
        .node_values(packages.iter())
        .build();

    println!("adding vertices + edges took {:?}", started.elapsed());
    println!("vertices = {}", graph.node_count());
    println!("edges = {}", graph.edge_count());

    println!("topological sort not available");
}
