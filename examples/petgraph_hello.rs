use petgraph::Graph;

fn main() {
    let mut graph = Graph::new_undirected();

    let v1 = graph.add_node(1);
    let v2 = graph.add_node(2);
    let v3 = graph.add_node(3);
    let v4 = graph.add_node(4);
    let v5 = graph.add_node(5);
    let v6 = graph.add_node(6);

    graph.add_edge(v1, v2, ());
    graph.add_edge(v1, v5, ());
    graph.add_edge(v2, v3, ());
    graph.add_edge(v2, v5, ());
    graph.add_edge(v3, v4, ());
    graph.add_edge(v4, v5, ());
    graph.add_edge(v4, v6, ());
}
