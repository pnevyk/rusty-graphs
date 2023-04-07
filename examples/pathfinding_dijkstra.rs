use std::{collections::HashMap, time::Instant};

use ordered_float::OrderedFloat;
use pathfinding::directed::dijkstra::{dijkstra, dijkstra_all};
use rusty_graphs::City;

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

fn main() {
    let cities = rusty_graphs::load_cities();

    let mut vertices = HashMap::with_capacity(cities.len());

    let started = Instant::now();

    for city in cities.iter() {
        let v = Vertex::new(city, &cities);
        vertices.insert(v.id, v);
    }

    println!("adding vertices + edges took {:?}", started.elapsed());
    println!("vertices = {}", vertices.len());

    let started = Instant::now();

    let (start, target) = cities.iter().fold((0, 0), |(start, target), city| {
        if city.name == rusty_graphs::DIJKSTRA_START {
            (city.id, target)
        } else if city.name == rusty_graphs::DIJKSTRA_TARGET {
            (start, city.id)
        } else {
            (start, target)
        }
    });

    println!("finding start and target took {:?}", started.elapsed());

    let started = Instant::now();

    let result = dijkstra_all(&start, |n| vertices[n].neighbors.iter().copied());

    println!("dijkstra (without goal) took {:?}", started.elapsed());
    println!("distance = {}", *result[&target].1);

    let started = Instant::now();

    let result = dijkstra(
        &start,
        |n| vertices[n].neighbors.iter().copied(),
        |n| *n == target,
    )
    .unwrap();

    println!("dijkstra (with goal) took {:?}", started.elapsed());
    println!("distance = {}", *result.1);
}
