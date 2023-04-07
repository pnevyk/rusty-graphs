use std::time::Instant;

use petgraph::{
    algo::toposort,
    graph::{Graph, NodeIndex},
};

fn main() {
    let packages = rusty_graphs::load_tree();

    let mut graph = Graph::new();

    let started = Instant::now();

    for package in packages.iter() {
        graph.add_node(package);
    }

    println!("adding vertices took {:?}", started.elapsed());
    println!("vertices = {}", graph.node_count());

    let started = Instant::now();

    for i in 0..graph.node_count() {
        for j in 0..graph.node_count() {
            let i = NodeIndex::from(i as u32);
            let j = NodeIndex::from(j as u32);

            let src = &graph[i];
            let dst = &graph[j];

            if dst.has_dependency(src) {
                graph.add_edge(i, j, ());
            }
        }
    }

    println!("connecting vertices took {:?}", started.elapsed());
    println!("edges = {}", graph.edge_count());

    let started = Instant::now();

    let sorted = toposort(&graph, None)
        .unwrap()
        .into_iter()
        .map(|v| graph[v].name.as_str())
        .collect::<Vec<_>>();

    println!("topological sort took {:?}", started.elapsed());
    println!("{sorted:?}");
}
