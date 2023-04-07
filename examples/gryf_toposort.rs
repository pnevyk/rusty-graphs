use std::time::Instant;

use gryf::algo::TopoSort;
use gryf::prelude::*;

fn main() {
    let packages = rusty_graphs::load_tree();

    let mut graph = Graph::new_directed();

    let started = Instant::now();

    for package in packages.iter() {
        graph.add_vertex(package);
    }

    println!("adding vertices took {:?}", started.elapsed());
    println!("vertices = {}", graph.vertex_count());

    let started = Instant::now();

    graph.connect_vertices(|u, v| v.has_dependency(u).then_some(()));

    println!("connecting vertices took {:?}", started.elapsed());
    println!("edges = {}", graph.edge_count());

    let started = Instant::now();

    let sorted = TopoSort::on(&graph)
        .run()
        .map(|r| r.map(|v| graph.vertex(v).unwrap().name.as_str()))
        .collect::<Result<Vec<_>, _>>()
        .unwrap();

    println!("topological sort took {:?}", started.elapsed());
    println!("{sorted:?}");
}
