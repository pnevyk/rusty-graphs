use std::time::Instant;

use rusty_graphs::UF32;

use prepona::prelude::*;
use prepona::{algo::Dijkstra, graph::SimpleGraph, storage::List};

fn main() {
    let cities = rusty_graphs::load_cities();

    // Using a custom wrapper over f32, because `ordered_float::OrderedFloat`
    // does not implement `num_traits::Unsigned`.
    let mut graph = SimpleGraph::init(List::<UF32>::init());

    let started = Instant::now();

    for _ in cities.iter() {
        graph.add_vertex();
    }

    println!("adding vertices took {:?}", started.elapsed());
    println!("vertices = {}", graph.vertex_count());

    let started = Instant::now();

    for i in 0..graph.vertex_count() {
        let src = &cities[i];

        #[allow(clippy::needless_range_loop)]
        for j in (i + 1)..graph.vertex_count() {
            let dst = &cities[j];

            if let Some(edge) = src.are_connected(dst) {
                graph
                    .add_edge(i, j, UF32::new(edge).unwrap().into())
                    .unwrap();
            }
        }
    }

    println!("connecting vertices took {:?}", started.elapsed());
    println!("edges = {}", graph.edges_count());

    let started = Instant::now();

    let (start, target) = graph
        .vertices()
        .into_iter()
        .fold((0, 0), |(start, target), v| {
            let city = &cities[v];
            if city.name == rusty_graphs::DIJKSTRA_START {
                (v, target)
            } else if city.name == rusty_graphs::DIJKSTRA_TARGET {
                (start, v)
            } else {
                (start, target)
            }
        });

    println!("finding start and target took {:?}", started.elapsed());

    let started = Instant::now();

    let result = Dijkstra::init(&graph).execute(&graph, start);

    println!("dijkstra (without goal) took {:?}", started.elapsed());
    println!(
        "distance = {}",
        result.distance_to(target).unwrap().unwrap().get()
    );

    println!("dijkstra (with goal) not available");
}
