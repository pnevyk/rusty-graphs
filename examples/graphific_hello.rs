#![allow(unused_assignments)]

use graphific::{AnyGraph, BasicUndirectedGraph, Edge, Vertex};

fn main() {
    let mut graph = BasicUndirectedGraph::<_, ()>::new();

    graph = graph.add_vertex(Vertex::new(1)).unwrap();
    graph = graph.add_vertex(Vertex::new(2)).unwrap();
    graph = graph.add_vertex(Vertex::new(3)).unwrap();
    graph = graph.add_vertex(Vertex::new(4)).unwrap();
    graph = graph.add_vertex(Vertex::new(5)).unwrap();
    graph = graph.add_vertex(Vertex::new(6)).unwrap();

    graph = graph.add_edge(Edge::new(0, 1)).unwrap();
    graph = graph.add_edge(Edge::new(0, 4)).unwrap();
    graph = graph.add_edge(Edge::new(1, 2)).unwrap();
    graph = graph.add_edge(Edge::new(1, 4)).unwrap();
    graph = graph.add_edge(Edge::new(2, 3)).unwrap();
    graph = graph.add_edge(Edge::new(3, 4)).unwrap();
    graph = graph.add_edge(Edge::new(3, 5)).unwrap();
}
