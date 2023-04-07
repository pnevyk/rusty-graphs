use prepona::prelude::*;
use prepona::{graph::SimpleGraph, storage::List};

fn main() {
    let mut graph = SimpleGraph::init(List::<()>::init());

    let v1 = graph.add_vertex();
    let v2 = graph.add_vertex();
    let v3 = graph.add_vertex();
    let v4 = graph.add_vertex();
    let v5 = graph.add_vertex();
    let v6 = graph.add_vertex();

    graph.add_edge(v1, v2, ().into()).unwrap();
    graph.add_edge(v1, v5, ().into()).unwrap();
    graph.add_edge(v2, v3, ().into()).unwrap();
    graph.add_edge(v2, v5, ().into()).unwrap();
    graph.add_edge(v3, v4, ().into()).unwrap();
    graph.add_edge(v4, v5, ().into()).unwrap();
    graph.add_edge(v4, v6, ().into()).unwrap();
}
