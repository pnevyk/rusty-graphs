use std::time::Instant;

use prepona::prelude::*;
use prepona::{algo::TopologicalSort, graph::SimpleGraph, storage::DiList};

fn main() {
    let packages = rusty_graphs::load_tree();

    let mut graph = SimpleGraph::init(DiList::<()>::init());

    let started = Instant::now();

    for _ in packages.iter() {
        graph.add_vertex();
    }

    println!("adding vertices took {:?}", started.elapsed());
    println!("vertices = {}", graph.vertex_count());

    let started = Instant::now();

    for i in 0..graph.vertex_count() {
        let src = &packages[i];

        #[allow(clippy::needless_range_loop)]
        for j in 0..graph.vertex_count() {
            let dst = &packages[j];

            if dst.has_dependency(src) {
                graph.add_edge(i, j, ().into()).unwrap();
            }
        }
    }

    println!("connecting vertices took {:?}", started.elapsed());
    println!("edges = {}", graph.edges_count());

    let started = Instant::now();

    let sorted = TopologicalSort::init()
        .execute(&graph)
        .into_iter()
        .map(|v| packages[v].name.as_str())
        .collect::<Vec<_>>();

    println!("topological sort took {:?}", started.elapsed());
    println!("{sorted:?}");
}
