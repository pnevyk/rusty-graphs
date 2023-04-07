#![allow(unused_variables)]

use std::time::Instant;

use graph::prelude::*;
use rusty_graphs::City;

fn main() {
    let cities = rusty_graphs::load_cities();

    let started = Instant::now();

    let mut edges = Vec::with_capacity(cities.len() * 5);
    for i in 0..cities.len() {
        let src = &cities[i];

        #[allow(clippy::needless_range_loop)]
        for j in (i + 1)..cities.len() {
            let dst = &cities[j];

            if let Some(edge) = src.are_connected(dst) {
                edges.push((i, j, edge));
            }
        }
    }

    let graph: UndirectedCsrGraph<usize, &City, f32> = GraphBuilder::new()
        .edges_with_values(edges)
        .node_values(cities.iter())
        .build();

    println!("adding vertices + edges took {:?}", started.elapsed());
    println!("vertices = {}", graph.node_count());
    println!("edges = {}", graph.edge_count());

    let started = Instant::now();

    let (start, target) = (0..graph.node_count()).fold((0, 0), |(start, target), v| {
        let city = graph.node_value(v);
        if city.name == rusty_graphs::DIJKSTRA_START {
            (v, target)
        } else if city.name == rusty_graphs::DIJKSTRA_TARGET {
            (start, v)
        } else {
            (start, target)
        }
    });

    println!("finding start and target took {:?}", started.elapsed());

    println!("dijkstra (without goal) not available");
    println!("dijkstra (with goal) not available");
}
