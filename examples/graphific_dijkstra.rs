#![allow(unused_variables)]

use std::time::Instant;

use graphific::{AnyGraph, BasicUndirectedGraph, Vertex};

fn main() {
    let cities = rusty_graphs::load_cities();

    let mut graph = BasicUndirectedGraph::new();

    let started = Instant::now();

    for (key, _) in cities.iter().enumerate() {
        let v = Vertex::with_value(key, ());
        graph = graph.add_vertex(v).unwrap();
    }

    println!("adding vertices took {:?}", started.elapsed());
    println!("vertices = {}", graph.vertices().len());

    let started = Instant::now();

    for i in 0..cities.len() {
        let src = &cities[i];

        #[allow(clippy::needless_range_loop)]
        for j in (i + 1)..cities.len() {
            let dst = &cities[j];

            if src.are_connected(dst).is_some() {
                graph = graph.add_edge_between_keys(i, j).unwrap();
            }
        }
    }

    println!("connecting vertices took {:?}", started.elapsed());
    println!("edges = {}", graph.edges().len());

    let started = Instant::now();

    let (start, target) = graph
        .vertices()
        .into_iter()
        .fold((0, 0), |(start, target), v| {
            let city = &cities[*v.key()];
            if city.name == rusty_graphs::DIJKSTRA_START {
                (*v.key(), target)
            } else if city.name == rusty_graphs::DIJKSTRA_TARGET {
                (start, *v.key())
            } else {
                (start, target)
            }
        });

    println!("finding start and target took {:?}", started.elapsed());

    println!("dijkstra (without goal) not available");
    println!("dijkstra (with goal) not available");
}
