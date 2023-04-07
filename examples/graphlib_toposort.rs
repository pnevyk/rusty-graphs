use std::time::Instant;

use graphlib::Graph;

fn main() {
    let packages = rusty_graphs::load_tree();
    let mut vertices = Vec::with_capacity(packages.len());

    let mut graph = Graph::new();

    let started = Instant::now();

    for package in packages.iter() {
        let v = graph.add_vertex(package);
        vertices.push(v);
    }

    println!("adding vertices took {:?}", started.elapsed());
    println!("vertices = {}", graph.vertex_count());

    let started = Instant::now();

    for i in 0..graph.vertex_count() {
        for j in 0..graph.vertex_count() {
            let i = &vertices[i];
            let j = &vertices[j];

            let src = graph.fetch(i).unwrap();
            let dst = graph.fetch(j).unwrap();

            if dst.has_dependency(src) {
                graph.add_edge(i, j).unwrap();
            }
        }
    }

    println!("connecting vertices took {:?}", started.elapsed());
    println!("edges = {}", graph.edge_count());

    let started = Instant::now();

    let sorted = graph
        .topo()
        .map(|v| graph.fetch(v).unwrap().name.as_str())
        .collect::<Vec<_>>();

    println!("topological sort took {:?}", started.elapsed());
    println!("{sorted:?}");
}
