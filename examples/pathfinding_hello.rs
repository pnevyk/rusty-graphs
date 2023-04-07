#![allow(dead_code, unused_variables)]

struct Vertex {
    label: usize,
    connections: Vec<usize>,
}

impl Vertex {
    fn new(label: usize, connections: Vec<usize>) -> Self {
        Self { label, connections }
    }
}

fn main() {
    let graph = vec![
        Vertex::new(1, vec![2, 5]),
        Vertex::new(2, vec![1, 3, 5]),
        Vertex::new(3, vec![2, 4]),
        Vertex::new(4, vec![3, 5, 6]),
        Vertex::new(5, vec![1, 2, 4]),
        Vertex::new(6, vec![4]),
    ];
}
