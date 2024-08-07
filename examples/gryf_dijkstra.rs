use std::time::Instant;

use gryf::{
    algo::ShortestPaths,
    core::id::{IdType, VertexId},
    graph::Graph,
    prelude::*,
};

fn main() {
    let cities = rusty_graphs::load_cities();

    let mut graph = Graph::new_undirected();

    let started = Instant::now();

    for city in cities.iter() {
        graph.add_vertex(city);
    }

    println!("adding vertices took {:?}", started.elapsed());
    println!("vertices = {}", graph.vertex_count());

    let started = Instant::now();

    graph.connect_vertices(|src, dst| src.are_connected(dst));

    println!("connecting vertices took {:?}", started.elapsed());
    println!("edges = {}", graph.edge_count());

    let started = Instant::now();

    let (start, target) = graph.vertices().fold(
        (VertexId::sentinel(), VertexId::sentinel()),
        |(start, target), v| {
            if v.attr().name == rusty_graphs::DIJKSTRA_START {
                (*v.id(), target)
            } else if v.attr().name == rusty_graphs::DIJKSTRA_TARGET {
                (start, *v.id())
            } else {
                (start, target)
            }
        },
    );

    println!("finding start and target took {:?}", started.elapsed());

    let started = Instant::now();

    let result = ShortestPaths::on(&graph).dijkstra().run(start).unwrap();

    println!("dijkstra (without goal) took {:?}", started.elapsed());
    println!("distance = {}", result[target]);

    let started = Instant::now();

    let result = ShortestPaths::on(&graph)
        .goal(target)
        .dijkstra()
        .run(start)
        .unwrap();

    println!("dijkstra (with goal) took {:?}", started.elapsed());
    println!("distance = {}", result[target]);
}
