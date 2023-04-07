use std::time::Instant;

use graphific::{AnyGraph, BasicDirectedGraph, Vertex};

fn main() {
    let packages = rusty_graphs::load_tree();

    let mut graph = BasicDirectedGraph::new();

    let started = Instant::now();

    for (key, _) in packages.iter().enumerate() {
        let v = Vertex::with_value(key, ());
        graph = graph.add_vertex(v).unwrap();
    }

    println!("adding vertices took {:?}", started.elapsed());
    println!("vertices = {}", graph.vertices().len());

    let started = Instant::now();

    for i in 0..packages.len() {
        let src = &packages[i];

        #[allow(clippy::needless_range_loop)]
        for j in 0..packages.len() {
            let dst = &packages[j];

            if dst.has_dependency(src) {
                graph = graph.add_edge_between_keys(i, j).unwrap();
            }
        }
    }

    println!("connecting vertices took {:?}", started.elapsed());
    println!("edges = {}", graph.edges().len());

    println!("topological sort not available");
}
