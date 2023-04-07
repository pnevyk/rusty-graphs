use std::time::Instant;

use petgraph::{
    algo::dijkstra,
    graph::{Graph, NodeIndex},
    visit::{IntoNodeReferences, NodeRef},
};

fn main() {
    let cities = rusty_graphs::load_cities();

    let mut graph = Graph::new_undirected();

    let started = Instant::now();

    for city in cities.iter() {
        graph.add_node(city);
    }

    println!("adding vertices took {:?}", started.elapsed());
    println!("vertices = {}", graph.node_count());

    let started = Instant::now();

    for i in 0..graph.node_count() {
        for j in (i + 1)..graph.node_count() {
            let i = NodeIndex::from(i as u32);
            let j = NodeIndex::from(j as u32);

            let src = &graph[i];
            let dst = &graph[j];

            if let Some(edge) = src.are_connected(dst) {
                graph.add_edge(i, j, edge);
            }
        }
    }

    println!("connecting vertices took {:?}", started.elapsed());
    println!("edges = {}", graph.edge_count());

    let started = Instant::now();

    let (start, target) = graph.node_references().fold(
        (NodeIndex::default(), NodeIndex::default()),
        |(start, target), v| {
            if v.weight().name == rusty_graphs::DIJKSTRA_START {
                (v.id(), target)
            } else if v.weight().name == rusty_graphs::DIJKSTRA_TARGET {
                (start, v.id())
            } else {
                (start, target)
            }
        },
    );

    println!("finding start and target took {:?}", started.elapsed());

    let started = Instant::now();

    let result = dijkstra(&graph, start, None, |e| *e.weight());

    println!("dijkstra (without goal) took {:?}", started.elapsed());
    println!("distance = {}", result[&target]);

    let started = Instant::now();

    let result = dijkstra(&graph, start, Some(target), |e| *e.weight());

    println!("dijkstra (with goal) took {:?}", started.elapsed());
    println!("distance = {}", result[&target]);
}
