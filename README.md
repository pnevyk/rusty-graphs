# Comparing Rust graph libraries, while introducing [gryf](https://github.com/pnevyk/gryf)

This is a collection of examples for showcasing various Rust graph data
structure libraries. It is code heavy, with only little commentary. The main
focus is put on the user experience. Performance is also considered, but without
proper benchmarks. Aspects like completeness, correctness, documentation or
usage and maintenance metrics are ignored whatsoever. Therefore, this document
should _not_ be considered as a guide to what dependency for your project to
choose.

The list of libraries discussed here is the following<sup>1</sup>:

* [petgraph](https://github.com/petgraph/petgraph)
* [prepona](https://github.com/maminrayej/prepona)
* [pathfinding](https://crates.io/crates/pathfinding)
* [graph](https://crates.io/crates/graph)
* [graphlib](https://crates.io/crates/graphlib)
* [graphific](https://crates.io/crates/graphific)
* [gryf](https://github.com/pnevyk/gryf)

**Disclaimer (bias):** I am the author of `gryf` library, which is in the list.
I tried to be fair, but parts of the commentary are subjective.

**Disclaimer (maintenance):** I do _not_ plan to keep this document up-to-date.
But I will accept PRs doing that.

**Disclaimer (fairness):** There may be multiple ways how to achieve something
in a library. If you think that an example can be improved, feel free to open a
PR.

**Disclaimer (benchmarks):** All run times presented in this document are only
orientational. They were _not_ collected in scientific manner. The examples were
compiled in `--release` mode.

<sup>1</sup>If you know of some other, feel free to tell me about it in an issue
or pull request.

## Exercises

1. Recreate an example graph from [Wikipedia article](https://en.wikipedia.org/wiki/Graph_(discrete_mathematics)) (see below). [jump](#example-graph)
2. Find [shortest paths](https://en.wikipedia.org/wiki/Shortest_path_problem)
   from a vertex in a non-trivial graph (_n = ~17.5k_) of cities ([data
   source](https://data.opendatasoft.com/explore/dataset/geonames-all-cities-with-a-population-1000%2540public/export/)),
   where neighboring cities are connected via (hypothetical) air transport.
   [jump](#shortest-paths)
3. Get a [topologically
   sorted](https://en.wikipedia.org/wiki/Topological_sorting) sequence of
   vertices in a (`cargo tree`) dependency graph to get a (hypothetical) valid
   compilation order. [jump](#topological-order)

<img src="https://upload.wikimedia.org/wikipedia/commons/thumb/5/5b/6n-graf.svg/1920px-6n-graf.svg.png" alt="Example graph from Wikipedia" width="400" style="background: white;" />

Vertices _V = { 1 , 2 , 3 , 4 , 5 , 6 }_ and edges _E = { { 1 , 2 } , { 1 , 5 } , { 2 , 3
} , { 2 , 5 } , { 3 , 4 } , { 4 , 5 } , { 4 , 6 } }_.

---

I believe that the chosen exercises are to an extent realistic and not
cherry-picked. Note that `graph` crate has a different focus and does not
implement shortest paths and topological order algorithms at the time of the
writing.

## Example graph

#### Table of contents

* [petgraph](#example-graph-in-petgraph)
* [prepona](#example-graph-in-prepona)
* [pathfinding](#example-graph-in-pathfinding)
* [graph](#example-graph-in-graph)
* [graphlib](#example-graph-in-graphlib)
* [graphific](#example-graph-in-graphific)
* [gryf](#example-graph-in-gryf)

### Example graph in `petgraph`

[source](examples/petgraph_hello.rs)

```rust
use petgraph::Graph;

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
```

As straightforward as one can get.

For adding edges, there is an alternative

```rust
graph.extend_with_edges([
    (v1, v2, ()),
    (v1, v5, ()),
    (v2, v3, ()),
    (v2, v5, ()),
    (v3, v4, ()),
    (v4, v5, ()),
    (v4, v6, ()),
]);
```

or even

```rust
graph.extend_with_edges([
    (v1, v2),
    (v1, v5),
    (v2, v3),
    (v2, v5),
    (v3, v4),
    (v4, v5),
    (v4, v6),
]);
```

(but the latter would require specifying the generic type of edge weight).

I don't have a strong opinion on which alternative is the best. It is also
better to discuss API for adding edges on a more realistic example.

`Graph::new_undirected()` creates an undirected graph, while `Graph::new()`
would create a directed graph (there is not `new_directed` constructor).
Defaulting to a graph variant (directed/undirected) when using common `new`
constructor could lead to surprising behavior for a user not expecting this
default, as I don't think there is a consensus on whether directed is more
common than undirected or vice versa.

### Example graph in `prepona`

[source](examples/prepona_hello.rs)

```rust
use prepona::prelude::*;
use prepona::{graph::SimpleGraph, storage::List};

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
```

We see a separation of graph storage (`List`, aka [adjacency
list](https://en.wikipedia.org/wiki/Adjacency_list)) and graph semantics
(`SimpleGraph`, i.e. without parallel edges). What is unfortunate is that we are
_required_ to make a choice, even if we don't care.

An interesting paper cut is the need for specifying the edge weight in this
example (`List::<()>`). Without it, we get this compile error:

```
error[E0282]: type annotations needed for `SimpleGraph<(), DefaultEdge<()>, Dir, AdjList<(), DefaultEdge<()>, Dir>>`
 --> examples/prepona_hello.rs:5:9
  |
5 |     let mut graph = SimpleGraph::init(List::init());
  |         ^^^^^^^^^
  |
help: consider giving `graph` an explicit type, where the type for type parameter `Dir` is specified
  |
5 |     let mut graph: SimpleGraph<(), DefaultEdge<()>, Dir, AdjList<(), DefaultEdge<()>, Dir>> = SimpleGraph::init(List::init());
  |                  ++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++

For more information about this error, try `rustc --explain E0282`.
```

It is interesting that according to the error message Rust can't infer `Dir`
(directed vs undirected), not edge weight (first generic parameter of `List`).
It is counter-intuitive to see that a possible (and likely the simplest) fix is
to specify the edge weight.

The need for `into` in `().into()` is due to the fact that the graph holds `E:
Edge<W>` instead of `W` directly, where `E` is a type that implements
[`Edge`](https://docs.rs/prepona/0.1.0/prepona/prelude/trait.Edge.html) trait.
This could be more ergonomic, as it looks like an unnecessary clutter, but not a
big deal.

Lastly, we can see that adding edge returns `Result`, hence the `unwrap`. This
is reasonable, as there are cases when this operation can fail (non-existent
vertex or already present edge for storages that do not support that). For
comparison, `petgraph` panics if a duplicate edge is added to adjacency matrix
graph. Ideally, there should be both a fallible and panicking method for such
operations which allows the user to choose according to their preference.

### Example graph in `pathfinding`

[source](examples/pathfinding_hello.rs)

```rust
struct Vertex {
    label: usize,
    connections: Vec<usize>,
}

impl Vertex {
    fn new(label: usize, connections: Vec<usize>) -> Self {
        Self { label, connections }
    }
}

let graph = vec![
    Vertex::new(1, vec![2, 5]),
    Vertex::new(2, vec![1, 3, 5]),
    Vertex::new(3, vec![2, 4]),
    Vertex::new(4, vec![3, 5, 6]),
    Vertex::new(5, vec![1, 2, 4]),
    Vertex::new(6, vec![4]),
];
```

`pathfinding` crate has a very different API than the rest, which is a good fit
for graph-like data or [implicit
graphs](https://en.wikipedia.org/wiki/Implicit_graph) without the need of
storing it in a graph data structure. The graph representation is basically
completely on the user, and the algorithms only ask for getting neighbors of a
given vertex. Therefore, **this example is not ideal to showcase the crate's
strengths**.

### Example graph in `graph`

[source](examples/graph_hello.rs)

```rust
use graph::prelude::*;

let graph: UndirectedCsrGraph<usize, i32> = GraphBuilder::new()
    .edges(vec![(0, 1), (0, 4), (1, 2), (1, 4), (2, 3), (3, 4), (3, 5)])
    .node_values(1..=6)
    .build();
```

`graph` crate uses the builder pattern for creating a graph (in fact, they have
a dedicated crate
[graph_builder](https://docs.rs/graph_builder/latest/graph_builder/) for it).
The only required method is `edges`, which gets an iterable over pairs of vertex
indices defining the graph structure. The vertex index type (first generic,
required parameter) must implement `Idx` trait, which is implemented for all
standard integer types. As the example graph has labels from 1 to 6, I also use
`node_values` to specify the labels for the vertices (whose type is specified by
the second generic parameter).

The builder pattern does a good job on requiring only what is really necessary
-- the graph structure. Vertex and edge weights/labels, which are needed only on
per case basis, are optional and do not clutter the code where unnecessary. Put
it in contrast with `petgraph`, where we would need to use unit types in
`graph.add_node(())` and `graph.add_edge(u, v, ())`.

Because the
[signature](https://docs.rs/graph_builder/0.3.1/graph_builder/builder/struct.GraphBuilder.html#method.build)
of the `build` method is generic over the return type, has several variants and
the type is where-constrained by `From<...>`, doing a slight mistake in typing
ends up in quite an overwhelming error message. For example in our case, if we
forgot to specify the node label type

```rust
let graph: UndirectedCsrGraph<usize> = ...
```

we would get

```
error[E0277]: the trait bound `graph::prelude::UndirectedCsrGraph<usize>: From<(graph_builder::graph::csr::NodeValues<{integer}>, EdgeList<{integer}, ()>, CsrLayout)>` is not satisfied
   --> examples/graph_hello.rs:6:44
    |
6   |       let graph: UndirectedCsrGraph<usize> = GraphBuilder::new()
    |  ____________________________________________^
7   | |         .edges(vec![(0, 1), (0, 4), (1, 2), (1, 4), (2, 3), (3, 4), (3, 5)])
8   | |         .node_values(1..=6)
    | |___________________________^ the trait `From<(graph_builder::graph::csr::NodeValues<{integer}>, EdgeList<{integer}, ()>, CsrLayout)>` is not implemented for `graph::prelude::UndirectedCsrGraph<usize>`
9   |           .build();
    |            ----- required by a bound introduced by this call
    |
    = help: the following other types implement trait `From<T>`:
              <graph::prelude::UndirectedCsrGraph<NI, (), EV> as From<(E, CsrLayout)>>
              <graph::prelude::UndirectedCsrGraph<NI, (), EV> as From<graph_builder::graph::csr::Csr<NI, NI, EV>>>
              <graph::prelude::UndirectedCsrGraph<NI, Label> as From<(DotGraph<NI, Label>, CsrLayout)>>
              <graph::prelude::UndirectedCsrGraph<NI, NV, EV> as From<(graph_builder::graph::csr::NodeValues<NV>, E, CsrLayout)>>
              <graph::prelude::UndirectedCsrGraph<NI> as From<(DotGraph<NI, Label>, CsrLayout)>>
              <graph::prelude::UndirectedCsrGraph<NI> as From<(Graph500<NI>, CsrLayout)>>
note: required by a bound in `graph::prelude::GraphBuilder::<graph_builder::builder::FromEdgeListAndNodeValues<NI, NV, EV>>::build`
   --> /local/path/to/graph_builder-0.3.0/src/builder.rs:480:16
    |
480 |         Graph: From<(NodeValues<NV>, EdgeList<NI, EV>, CsrLayout)>,
    |                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `graph::prelude::GraphBuilder::<graph_builder::builder::FromEdgeListAndNodeValues<NI, NV, EV>>::build`

For more information about this error, try `rustc --explain E0277`.
```

### Example graph in `graphlib`

[source](examples/graphlib_hello.rs)

```rust
use graphlib::Graph;

let mut graph = Graph::new();

let v1 = graph.add_vertex(1);
let v2 = graph.add_vertex(2);
let v3 = graph.add_vertex(3);
let v4 = graph.add_vertex(4);
let v5 = graph.add_vertex(5);
let v6 = graph.add_vertex(6);

graph.add_edge(&v1, &v2).unwrap();
graph.add_edge(&v1, &v5).unwrap();
graph.add_edge(&v2, &v3).unwrap();
graph.add_edge(&v2, &v5).unwrap();
graph.add_edge(&v3, &v4).unwrap();
graph.add_edge(&v4, &v5).unwrap();
graph.add_edge(&v4, &v6).unwrap();
```

This is very similar to what we have already seen. The only difference is that
passing vertex index to `add_edge` is done by reference. Under the hoods, the
vertex index is a 16-byte array, which is arguably unnecessarily large number
space, but probably chosen to decrease risk of conflict for
`VertexId::random()`.

One major disadvantage of `graphlib` is that -- at the time of writing -- it
only supports directed graphs. The example code is therefore imprecise as the
the goal was an undirected graph.

### Example graph in `graphific`

[source](examples/graphific_hello.rs)

```rust
use graphific::{AnyGraph, BasicUndirectedGraph, Edge, Vertex};

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
```

`graphific` takes a fundamentally different approach. Graph manipulation methods
take `&self` and return new instance of accordingly modified graph, without
mutating the original instance. It is essentially a [persistent data
structure](https://en.wikipedia.org/wiki/Persistent_data_structure)
implementation, although at the time of writing it simply clones the original
instance, which is a massive performance hit for larger graphs.

Another difference is explicit usage of a `Vertex` and `Edge` types that wrap
corresponding weights. I am not sure about the advantages.

Unfortunate is the need for importing `AnyGraph` trait, without which we can't
use neither `add_vertex` nor `add_edge`, which are essential for graph building.

### Example graph in `gryf`

[source](examples/gryf_hello.rs)

```rust
use gryf::prelude::*;

let mut graph = Graph::new_undirected();

let v1 = graph.add_vertex(1);
let v2 = graph.add_vertex(2);
let v3 = graph.add_vertex(3);
let v4 = graph.add_vertex(4);
let v5 = graph.add_vertex(5);
let v6 = graph.add_vertex(6);

graph.add_edge(v1, v2, ());
graph.add_edge(v1, v5, ());
graph.add_edge(v2, v3, ());
graph.add_edge(v2, v5, ());
graph.add_edge(v3, v4, ());
graph.add_edge(v4, v5, ());
graph.add_edge(v4, v6, ());
```

Almost identical to `petgraph`. For both `add_vertex` and `add_edge` there
exists a fallible `try_*` counterpart, which could be used if desired.

`gryf` has both `new_undirected` and `new_directed` constructors. There is also the `new` constructor, which does not specify the directionality of edges. And so if we used it in our example

```rust
let mut graph = Graph::new();
```

we would get

```
error[E0282]: type annotations needed for `gryf::Graph<i32, (), Ty>`
 --> examples/gryf_hello.rs:4:9
  |
4 |     let mut graph = Graph::new();
  |         ^^^^^^^^^
  |
help: consider giving `graph` an explicit type, where the type for type parameter `Ty` is specified
  |
4 |     let mut graph: gryf::Graph<i32, (), Ty> = Graph::new();
  |                  ++++++++++++++++++++++++++

For more information about this error, try `rustc --explain E0282`.
```

Nevertheless, it could be fixed by specifying the `Ty` generic parameter (as
suggested by the error message) and only it:

```rust
use gryf::core::marker::Undirected;

let mut graph = Graph::<_, _, Undirected>::new();
```

## Shortest paths

Support code:

```rust
#[derive(Debug, Clone, PartialEq)]
pub struct City {
    pub id: u64,
    pub name: String,
    pub pop: u64,
    pub lat: f32,
    pub lon: f32,
}

impl City {
    pub const MAX_DISTANCE: f32 = 300.0;

    pub fn dist(&self, other: &Self) -> f32 {
        // ...
    }

    pub fn are_connected(&self, other: &Self) -> Option<f32> {
        let d = self.dist(other);
        (d <= Self::MAX_DISTANCE && d > 0.0).then_some(d)
    }
}
```

Data downloaded from
[here](https://data.opendatasoft.com/explore/dataset/geonames-all-cities-with-a-population-1000%2540public/export/).

#### Table of contents

* [petgraph](#shortest-paths-in-petgraph)
* [prepona](#shortest-paths-in-prepona)
* [pathfinding](#shortest-paths-in-pathfinding)
* [graph](#shortest-paths-in-graph)
* [graphlib](#shortest-paths-in-graphlib)
* [graphific](#shortest-paths-in-graphific)
* [gryf](#shortest-paths-in-gryf)

### Shortest paths in `petgraph`

[source](examples/petgraph_dijkstra.rs)

```rust
use petgraph::{
    algo::dijkstra,
    graph::{Graph, NodeIndex},
    visit::{IntoNodeReferences, NodeRef},
};

let cities = load_cities();

let mut graph = Graph::new_undirected();

for city in cities.iter() {
    graph.add_node(city);
}

println!("vertices = {}", graph.node_count());

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

println!("edges = {}", graph.edge_count());

let (start, target) = graph.node_references().fold(
    (NodeIndex::default(), NodeIndex::default()),
    |(start, target), v| {
        if v.weight().name == START {
            (v.id(), target)
        } else if v.weight().name == TARGET {
            (start, v.id())
        } else {
            (start, target)
        }
    },
);

let result = dijkstra(&graph, start, None, |e| *e.weight());
println!("distance = {}", result[&target]);

let result = dijkstra(&graph, start, Some(target), |e| *e.weight());
println!("distance = {}", result[&target]);
```

```
adding vertices took 146.112µs
vertices = 17695
connecting vertices took 7.201238525s
edges = 1232956
finding start and target took 157.563µs
dijkstra (without goal) took 45.668564ms
distance = 12782.68
dijkstra (with goal) took 31.75331ms
distance = 12782.68
```

For connecting the cities, we need to manually iterate over all pairs of
vertices and add edges where the cities are connected according to our
criterion. This is a pattern that we will see in most examples. We need to use a
for loop with `usize` indices and create `NodeIndex` manually from them. This
assumes that vertices are stored in contiguous order without gaps (which is
reasonable to expect).

For finding the start and target vertex indices we use `Iterator::fold` over
vertex references. This is again a pattern that we will see in most examples.
There might be a cleaner way, but for the purposes of this document it does not
matter the most.

Running the algorithm is done via calling `dijkstra` function. This approach
requires to specify all parameters, including the goal vertex and edge weight
function, even in cases when not needed (no goal) or where a reasonable default
exists (the edge itself). The advantage of edge weight function is that it gets
edge reference which also contains source and target indices. This allows to
calculate the weight based on the vertices without the need of storing the
weight during the graph creation.

The function returns `HashMap<NodeId, W>`. It is impossible to reconstruct the
path.

There is no way how to indicate an error (e.g, when an edge weight is negative).
In fact, the implementation at the time of writing does not detect that and
returns potentially invalid output.

### Shortest paths in `prepona`

[source](examples/prepona_dijkstra.rs)

```rust
use prepona::prelude::*;
use prepona::{algo::Dijkstra, graph::SimpleGraph, storage::List};

let cities = load_cities();

// Using a custom wrapper over f32, because `ordered_float::OrderedFloat`
// does not implement `num_traits::Unsigned`.
let mut graph = SimpleGraph::init(List::<UF32>::init());

for _ in cities.iter() {
    graph.add_vertex();
}

println!("vertices = {}", graph.vertex_count());

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

println!("edges = {}", graph.edges_count());

let (start, target) = graph
    .vertices()
    .into_iter()
    .fold((0, 0), |(start, target), v| {
        let city = &cities[v];
        if city.name == START {
            (v, target)
        } else if city.name == TARGET {
            (start, v)
        } else {
            (start, target)
        }
    });

let result = Dijkstra::init(&graph).execute(&graph, start);
println!(
    "distance = {}",
    result.distance_to(target).unwrap().unwrap().get()
);

println!("dijkstra (with goal) not available");
```

```
adding vertices took 253.299µs
vertices = 17695
connecting vertices took 7.401539732s
edges = 1232956
finding start and target took 359.142µs
dijkstra (without goal) took 528.269438ms
distance = 12782.68
dijkstra (with goal) not available
```

Fundamentally equivalent way how to construct the graph and get the start and
target vertices. Vertex in `prepona` can't hold any data, so we need to index
into the `cities` "database".

Running the algorithm consists of two steps: first the algorithm state is
initialized (`init`) and then the algorithm is executed (`execute`). The
advantages of this separation are not clear (to me). Moreover, it allows
misusing the API by passing different (or modified) instances of a graph into
each step, causing unspecified behavior. There is no way how to specify a goal
vertex or use a custom edge weight calculation.

The algorithm returns `ShortestPathSubgraph`. This type provides a `distance_to`
method. It could also provide a path reconstruction method, although it is not
present at the time of writing. A new type allows to change internals without it
being a breaking change.

Working with floats is not very convenient. The bounds on the generic type
requires the weight to implement both `std::cmp::Ord` and
`num_traits::Unsigned`, which rules out standard floats (`f32`, `f64`) as well
as common "ordered floats"
([`ordered_float::OrderedFloat`](https://crates.io/crates/ordered-float),
[`float_ord::FloatOrd`](https://crates.io/crates/float-ord)).

Method `distance_to` returns `Option<Magnitude<W>>`. `Magnitude<W>` is an enum
with `Finite(W)`, `PosInfinite` and `NegInfinite`, which seems unnecessary (as
that should be encoded in `W` itself or as `None`) and creates the necessity to
use double `unwrap`. The `get` call originates from using a custom type for
unsigned float.

### Shortest paths in `pathfinding`

[source](examples/pathfinding_dijkstra.rs)

```rust
use std::collections::HashMap;

use ordered_float::OrderedFloat;
use pathfinding::directed::dijkstra::{dijkstra, dijkstra_all};

struct Vertex {
    id: u64,
    neighbors: Vec<(u64, OrderedFloat<f32>)>,
}

impl Vertex {
    fn new(city: &City, cities: &[City]) -> Self {
        let neighbors = cities
            .iter()
            .filter_map(|other| {
                city.are_connected(other)
                    .map(|dist| (other.id, OrderedFloat(dist)))
            })
            .collect();

        Self {
            id: city.id,
            neighbors,
        }
    }
}

let cities = load_cities();

let mut vertices = HashMap::with_capacity(cities.len());

for city in cities.iter() {
    let v = Vertex::new(city, &cities);
    vertices.insert(v.id, v);
}

println!("vertices = {}", vertices.len());

let (start, target) = cities.iter().fold((0, 0), |(start, target), city| {
    if city.name == START {
        (city.id, target)
    } else if city.name == TARGET {
        (start, city.id)
    } else {
        (start, target)
    }
});

let result = dijkstra_all(&start, |n| vertices[n].neighbors.iter().copied());
println!("distance = {}", *result[&target].1);

let result = dijkstra(
    &start,
    |n| vertices[n].neighbors.iter().copied(),
    |n| *n == target,
)
.unwrap();
println!("distance = {}", *result.1);
```

```
adding vertices + edges took 13.78980296s
vertices = 17695
finding start and target took 122.664µs
dijkstra (without goal) took 23.785947ms
distance = 12782.68
dijkstra (with goal) took 15.334814ms
distance = 12782.68
```

Again, due to a different approach to the API, we need to construct the graph of
connections between the cities ourselves. Nevertheless, it is reasonably
straightforward.

For running the algorithm, a function is called. There are two<sup>3</sup>
variants: without a goal (`dijkstra_all`) and with a goal (`dijkstra`). The
functions require the start vertex and a function returning an iterable over
neighbors of given vertex along with the edge weights. We need to use
[`ordered_float::OrderedFloat`](https://crates.io/crates/ordered-float) for the
edge weight as the algorithm requires it to be `std::cmp::Ord`. The way of
specifying the goal is more flexible than in `petgraph` as it is determined by a
closure with custom logic instead of passing a vertex index.

Functions `dijkstra_all` and `dijkstra` return `HashMap<Vertex, (Vertex, W)>`
and `Option<(Vec<Vertex>, W)>`, respectively. Reconstructing the path can be
done using `build_path` function, given that `dijkstra_partial` is used instead
of `dijkstra`.

<sup>3</sup>Technically there is a third variant `dijkstra_partial`, but it is not
important for our example.

### Shortest paths in `graph`

[source](examples/graph_dijkstra.rs)

```rust
use graph::prelude::*;

let cities = load_cities();

let mut edges = Vec::with_capacity(cities.len() * 5);
for i in 0..cities.len() {
    let src = &cities[i];

    #[allow(clippy::needless_range_loop)]
    for j in (i + 1)..cities.len() {
        let dst = &cities[j];

        if let Some(edge) = src.are_connected(dst) {
            edges.push((i, j, edge));
        }
    }
}

let graph: UndirectedCsrGraph<usize, &City, f32> = GraphBuilder::new()
    .edges_with_values(edges)
    .node_values(cities.iter())
    .build();

println!("vertices = {}", graph.node_count());
println!("edges = {}", graph.edge_count());

let (start, target) = (0..graph.node_count()).fold((0, 0), |(start, target), v| {
    let city = graph.node_value(v);
    if city.name == START {
        (v, target)
    } else if city.name == TARGET {
        (start, v)
    } else {
        (start, target)
    }
});

println!("dijkstra (without goal) not available");
println!("dijkstra (with goal) not available");
```

```
adding vertices + edges took 6.941942926s
vertices = 17695
edges = 1232956
finding start and target took 9.656µs
dijkstra (without goal) not available
dijkstra (with goal) not available
```

`graph` crate has different focus: high-performant algorithms on large (sparse)
graphs. It does not provide an implementation of Dijkstra algorithm. For
single-source shortest paths, there is `delta_stepping` function, which requires
a directed graph and a `delta` parameter.

### Shortest paths in `graphlib`

[source](examples/graphlib_dijkstra.rs)

```rust
use graphlib::{iterators::Dijkstra, Graph, VertexId};

let cities = load_cities();

let mut graph = Graph::new();
let mut vertices = Vec::with_capacity(cities.len());

for city in cities.iter() {
    let v = graph.add_vertex(city);
    vertices.push(v);
}

println!("vertices = {}", graph.vertex_count());

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

println!("edges = {}", graph.edge_count());

let (start, target) = graph.vertices().fold(
    (VertexId::random(), VertexId::random()),
    |(start, target), v| {
        let city = graph.fetch(v).unwrap();
        if city.name == START {
            (*v, target)
        } else if city.name == TARGET {
            (start, *v)
        } else {
            (start, target)
        }
    },
);

let mut result = Dijkstra::new(&graph, &start).unwrap();
println!(
    "distance = {}",
    result.get_distance(&target).unwrap() * City::MAX_DISTANCE
);

println!("dijkstra (with goal) not available");
```

```
adding vertices took 32.564949ms
vertices = 17695
connecting vertices took 51.361698183s
edges = 2465912
finding start and target took 911.451µs
dijkstra (without goal) took 1.571849687s
distance = 12782.681
dijkstra (with goal) not available
```

There are two inconveniences in `graphlib` for this example. First, it does not
provide an implementation of undirected graph, thus we need to add every edge
twice going forward and backward. Second, it requires the edge weights to be in
range [0, 1].

Running the algorithm is done via `Dijkstra::new` constructor. It does not allow
to specify a goal or custom edge weight. The return type is `Result`, so that
encountered errors can be handled in a graceful manner. There is possibility to
recompute the distances from a new start using `set_source`.

The type has `get_distance_to` and `get_path_to` (for path reconstruction).
However, the former takes `&mut self` and the latter takes `self`, which are
both somewhat unexpected and inconvenient choices.

### Shortest paths in `graphific`

[source](examples/graphific_dijkstra.rs)

```rust
use graphific::{AnyGraph, BasicUndirectedGraph, Vertex};

let cities = load_cities();

let mut graph = BasicUndirectedGraph::new();

for (key, _) in cities.iter().enumerate() {
    let v = Vertex::with_value(key, ());
    graph = graph.add_vertex(v).unwrap();
}

println!("vertices = {}", graph.vertices().len());

for i in 0..cities.len() {
    let src = &cities[i];

    #[allow(clippy::needless_range_loop)]
    for j in (i + 1)..cities.len() {
        let dst = &cities[j];

        if src.are_connected(dst).is_some() {
            graph = graph.add_edge_between_keys(i, j).unwrap();
        }
    }
}

println!("edges = {}", graph.edges().len());

let (start, target) = graph
    .vertices()
    .into_iter()
    .fold((0, 0), |(start, target), v| {
        let city = &cities[*v.key()];
        if city.name == START {
            (*v.key(), target)
        } else if city.name == TARGET {
            (start, *v.key())
        } else {
            (start, target)
        }
    });

println!("dijkstra (without goal) not available");
println!("dijkstra (with goal) not available");
```

```
adding vertices took 64.639742ms
vertices = 17695
^C
```

### Shortest paths in `gryf`

[source](examples/gryf_dijkstra.rs)

```rust
use gryf::algo::ShortestPaths;
use gryf::core::index::VertexIndex;
use gryf::prelude::*;

let cities = load_cities();

let mut graph = Graph::new_undirected();

for city in cities.iter() {
    graph.add_vertex(city);
}

println!("vertices = {}", graph.vertex_count());

graph.connect_vertices(|src, dst| src.are_connected(dst));

println!("edges = {}", graph.edge_count());

let (start, target) = graph.vertices().fold(
    (VertexIndex::null(), VertexIndex::null()),
    |(start, target), v| {
        if v.data().name == START {
            (*v.index(), target)
        } else if v.data().name == TARGET {
            (start, *v.index())
        } else {
            (start, target)
        }
    },
);

let result = ShortestPaths::on(&graph).dijkstra().run(start).unwrap();
println!("distance = {}", result[target]);

let result = ShortestPaths::on(&graph)
    .goal(target)
    .dijkstra()
    .run(start)
    .unwrap();
println!("distance = {}", result[target]);
```

```
adding vertices took 404.781µs
vertices = 17695
connecting vertices took 7.245386004s
edges = 1232956
finding start and target took 220.374µs
dijkstra (without goal) took 48.9059ms
distance = 12782.68
dijkstra (with goal) took 33.414195ms
distance = 12782.68
```

In `gryf`, there is a small quality-of-life improvement in the form of
`connect_vertices` which allows to specify edges by a predicate that takes a
pair of vertices and returns the weight or nothing.

Running the algorithm is done via the builder pattern. It consists of
initializing the builder for given graph, setting what should be set, and
finally calling `run` with the start vertex (the only fundamentally required
parameter). It is possible to specify the goal vertex with `goal` builder method
or custom edge weight using `edge_weight_fn` builder method.

One important difference is that the builder is generically named
`ShortestPaths`, without any reference to a specific algorithm. (Note: calling
the `dijkstra` builder method is not technically necessary to get the result,
but it forces to use the Dijkstra algorithm to be comparable with other
libraries.) This allows the user not care about the underlying algorithm (unless
wanted or necessary) to get the solution of their problem.

The algorithm returns `Result<ShortestPaths, ...>`. Thus, encountered errors can
be handled in a graceful manner. The type provides `dist` method to get a
distance to a vertex, and `reconstruct` to get an iterator of vertices on the
shortest path. It also implements `Index` trait as a shortcut for `dist` (but
panicking on `None`).

## Topological order

Support code:

```rust
#[derive(Debug, Clone)]
pub struct Package {
    pub name: String,
    pub deps: BTreeSet<String>,
}

impl Package {
    pub fn has_dependency(&self, other: &Self) -> bool {
        self.deps.contains(other.name.as_str())
    }
}
```

Dependencies parsed from the output of `cargo --no-dedupe --prefix depth`.

#### Table of contents

* [petgraph](#topological-order-in-petgraph)
* [prepona](#topological-order-in-prepona)
* [pathfinding](#topological-order-in-pathfinding)
* [graph](#topological-order-in-graph)
* [graphlib](#topological-order-in-graphlib)
* [graphific](#topological-order-in-graphific)
* [gryf](#topological-order-in-gryf)

### Topological order in `petgraph`

[source](examples/petgraph_toposort.rs)

```rust
use petgraph::{
    algo::toposort,
    graph::{Graph, NodeIndex},
};

let packages = load_tree();

let mut graph = Graph::new();

for package in packages.iter() {
    graph.add_node(package);
}

println!("vertices = {}", graph.node_count());

for i in 0..graph.node_count() {
    for j in 0..graph.node_count() {
        let i = NodeIndex::from(i as u32);
        let j = NodeIndex::from(j as u32);

        let src = &graph[i];
        let dst = &graph[j];

        if dst.has_dependency(src) {
            graph.add_edge(i, j, ());
        }
    }
}

println!("edges = {}", graph.edge_count());

let sorted = toposort(&graph, None)
    .unwrap()
    .into_iter()
    .map(|v| graph[v].name.as_str())
    .collect::<Vec<_>>();
println!("{sorted:?}");
```

```
adding vertices took 2.644µs
vertices = 106
connecting vertices took 361.425µs
edges = 200
topological sort took 10.763µs
```

Making the graph and running the algorithm is done the same way as in the
shortest path example. The `toposort` function also accepts optional mutable
reference to `DfsSpace`. As far as I understand it, its purpose is to reuse
allocated stack and discovered set, so that there is no need for buffer
reallocation if used on the same or only slightly changed graph. Using
`language:rust path:*.rs /toposort\(.+,\s*Some\(/` query on GitHub gave me 4
relevant code instances of using some space, from which just one was using it in
the intended way. It is a good idea to be able to exploit some knowledge from
the past to avoid unnecessary performance hits, but it seems that it isn't used
widely but still adds clutter to the API.

The function returns `Vec<NodeId>` if successful and `Cycle<NodeId>` if a cycle
is detected. It is nice to have an information about where the cycle is,
although it may be more useful to get an edge that participates in the cycle
rather than just the vertex.

### Topological order in `prepona`

[source](examples/prepona_toposort.rs)

```rust
use prepona::prelude::*;
use prepona::{algo::TopologicalSort, graph::SimpleGraph, storage::DiList};

let packages = load_tree();

let mut graph = SimpleGraph::init(DiList::<()>::init());

for _ in packages.iter() {
    graph.add_vertex();
}

println!("vertices = {}", graph.vertex_count());

for i in 0..graph.vertex_count() {
    let src = &packages[i];

    #[allow(clippy::needless_range_loop)]
    for j in 0..graph.vertex_count() {
        let dst = &packages[j];

        if dst.has_dependency(src) {
            graph.add_edge(i, j, ().into()).unwrap();
        }
    }
}

println!("edges = {}", graph.edges_count());

let sorted = TopologicalSort::init()
    .execute(&graph)
    .into_iter()
    .map(|v| packages[v].name.as_str())
    .collect::<Vec<_>>();
println!("{sorted:?}");
```

```
adding vertices took 3.286µs
vertices = 106
connecting vertices took 260.131µs
edges = 200
topological sort took 52.559µs
```

The code is again very similar to the example for shortest path. The only
interesting fact is that the algorithm does not return an error in case of
cycle. In fact, the implementation at the time of writing does not detect
cycles.

### Topological order in `pathfinding`

[source](examples/pathfinding_toposort.rs)

```rust
use std::collections::HashMap;

use pathfinding::directed::topological_sort::topological_sort;

let packages = rusty_graphs::load_tree();
let mut inverse_deps = HashMap::with_capacity(packages.len());
let mut roots = Vec::new();

for package in packages.iter() {
    let deps = packages
        .iter()
        .filter(|other| other.has_dependency(package))
        .map(|other| other.name.as_str())
        .collect::<Vec<_>>();

    if package.deps.is_empty() {
        roots.push(package.name.as_str());
    }

    inverse_deps.insert(package.name.as_str(), deps);
}

println!("vertices = {}", inverse_deps.len());

let sorted = topological_sort(&roots, |name| inverse_deps[name].iter().copied()).unwrap();

println!("{sorted:?}");
```

```
adding vertices + edges took 177.503µs
vertices = 106
topological sort took 88.716µs
```

For `pathfinding` we see again custom handling of graph storage. The API is
consistent with the shortest path example. Similar to `petgraph`, the return
type is `Vec<Vertex>` if successful and `Vertex` if a cycle is detected.

### Topological order in `graph`

[source](examples/graph_toposort.rs)

```rust
use graph::prelude::*;

let packages = load_tree();

let mut edges = Vec::with_capacity(packages.len() * 5);
for i in 0..packages.len() {
    let src = &packages[i];

    #[allow(clippy::needless_range_loop)]
    for j in 0..packages.len() {
        let dst = &packages[j];

        if dst.has_dependency(src) {
            edges.push((i, j));
        }
    }
}

let graph: DirectedCsrGraph<usize, &Package> = GraphBuilder::new()
    .edges(edges)
    .node_values(packages.iter())
    .build();

println!("vertices = {}", graph.node_count());
println!("edges = {}", graph.edge_count());

println!("topological sort not available");
```

```
adding vertices + edges took 941.31µs
vertices = 106
edges = 200
topological sort not available
```

Again, for different focus of the crate, the implementation of topological order
is not available.

### Topological order in `graphlib`

[source](examples/graphlib_toposort.rs)

```rust
use graphlib::Graph;

let packages = load_tree();
let mut vertices = Vec::with_capacity(packages.len());

let mut graph = Graph::new();

for package in packages.iter() {
    let v = graph.add_vertex(package);
    vertices.push(v);
}

println!("vertices = {}", graph.vertex_count());

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

println!("edges = {}", graph.edge_count());

let sorted = graph
    .topo()
    .map(|v| graph.fetch(v).unwrap().name.as_str())
    .collect::<Vec<_>>();
println!("{sorted:?}");
```

```
adding vertices took 187.755µs
vertices = 106
connecting vertices took 550.037µs
edges = 200
topological sort took 19.083µs
```

In `graphlib`, the `topo` method returns a custom type `Topo` which implements
an iterator over vertex indices in topological order. Being lazy, it allows to
take only subset of vertices if desired in the application without the need of
finding a topological order between the rest of the vertices. This is a nice
feature.

Unfortunately, the algorithm panics if it encounters a cycle. Or you could use
`Topo::is_cyclic(&mut self) -> bool`, which exhausts the underlying iterator,
and then call `topo` again.

### Topological order in `graphific`

[source](examples/graphific_toposort.rs)

```rust
use graphific::{AnyGraph, BasicDirectedGraph, Vertex};

let packages = load_tree();

let mut graph = BasicDirectedGraph::new();

for (key, _) in packages.iter().enumerate() {
    let v = Vertex::with_value(key, ());
    graph = graph.add_vertex(v).unwrap();
}

println!("vertices = {}", graph.vertices().len());

for i in 0..packages.len() {
    let src = &packages[i];

    #[allow(clippy::needless_range_loop)]
    for j in 0..packages.len() {
        let dst = &packages[j];

        if dst.has_dependency(src) {
            graph = graph.add_edge_between_keys(i, j).unwrap();
        }
    }
}

println!("edges = {}", graph.edges().len());

println!("topological sort not available");
```

```
adding vertices took 13.191µs
vertices = 106
connecting vertices took 317.225µs
edges = 200
topological sort not available
```

Topological order is not available in `graphific`.

### Topological order in `gryf`

[source](examples/gryf_toposort.rs)

```rust
use gryf::algo::TopoSort;
use gryf::prelude::*;

let packages = load_tree();

let mut graph = Graph::new_directed();

for package in packages.iter() {
    graph.add_vertex(package);
}

println!("vertices = {}", graph.vertex_count());

graph.connect_vertices(|u, v| v.has_dependency(u).then_some(()));

println!("edges = {}", graph.edge_count());

let sorted = TopoSort::on(&graph)
    .run()
    .map(|r| r.map(|v| graph[v].name.as_str()))
    .collect::<Result<Vec<_>, _>>()
    .unwrap();
println!("{sorted:?}");
```

```
adding vertices took 2.044µs
vertices = 106
connecting vertices took 290.569µs
edges = 200
topological sort took 37.061µs
```

In `gryf` a custom type is returned which implements iterator over
`Result<VertexId, Error>`. This allows lazy behavior but still makes it possible
to react on a cycle. Thanks to the design of
[`FromIterator`](https://doc.rust-lang.org/nightly/core/iter/trait.FromIterator.html#implementors)
std trait, it is also possible to collect such iterator into
`Result<Vec<VertexId>, Error>`, which effectively becomes the same API as in
`petgraph` for example. If one wants to stick with the lazy iterator semantics,
the `Result` item type makes the usage a little bit awkward. If the graph is
guaranteed to be acyclic, the experience can be improved by adding
`.map(Result::unwrap)` just after `run()`. The cycle error contains an edge that
is part of the cycle. There is also a helper routine to collect all edges of
that cycle.

## Conclusion

It's awesome to see that each crate has unique idea(s) and that one can take
inspiration from, and so I did. Obviously, some libraries are developed more
than others (which may be even abandoned). Different libraries have different
design goals too.

I am satisfied with how `gryf` "scores" in this comparison. There are definitely
parts to refine and features to add, but I believe that the base is strong. Any
feedback is very appreciated. And if you like what you see, contribution (be it
code or just an idea) is welcome too of course.
