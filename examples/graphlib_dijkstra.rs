use std::time::Instant;

use rusty_graphs::City;

use graphlib::{iterators::Dijkstra, Graph, VertexId};

fn main() {
    let cities = rusty_graphs::load_cities();

    let mut graph = Graph::new();
    let mut vertices = Vec::with_capacity(cities.len());

    let started = Instant::now();

    for city in cities.iter() {
        let v = graph.add_vertex(city);
        vertices.push(v);
    }

    println!("adding vertices took {:?}", started.elapsed());
    println!("vertices = {}", graph.vertex_count());

    let started = Instant::now();

    for i in 0..graph.vertex_count() {
        for j in (i + 1)..graph.vertex_count() {
            let i = &vertices[i];
            let j = &vertices[j];

            let src = graph.fetch(i).unwrap();
            let dst = graph.fetch(j).unwrap();

            if let Some(edge) = src.are_connected(dst) {
                // Weights are required to be in range [0, 1].
                let edge_norm = edge / City::MAX_DISTANCE;

                graph.add_edge_with_weight(i, j, edge_norm).unwrap();

                // Graph is directed and that cannot be changed.
                graph.add_edge_with_weight(j, i, edge_norm).unwrap();
            }
        }
    }

    println!("connecting vertices took {:?}", started.elapsed());
    println!("edges = {}", graph.edge_count());

    let started = Instant::now();

    let (start, target) = graph.vertices().fold(
        (VertexId::random(), VertexId::random()),
        |(start, target), v| {
            let city = graph.fetch(v).unwrap();
            if city.name == rusty_graphs::DIJKSTRA_START {
                (*v, target)
            } else if city.name == rusty_graphs::DIJKSTRA_TARGET {
                (start, *v)
            } else {
                (start, target)
            }
        },
    );

    println!("finding start and target took {:?}", started.elapsed());

    let started = Instant::now();

    let mut result = Dijkstra::new(&graph, &start).unwrap();

    println!("dijkstra (without goal) took {:?}", started.elapsed());
    println!(
        "distance = {}",
        result.get_distance(&target).unwrap() * City::MAX_DISTANCE
    );

    println!("dijkstra (with goal) not available");
}
