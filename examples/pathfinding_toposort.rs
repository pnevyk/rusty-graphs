use std::{collections::HashMap, time::Instant};

use pathfinding::directed::topological_sort::topological_sort;

fn main() {
    let packages = rusty_graphs::load_tree();
    let mut inverse_deps = HashMap::with_capacity(packages.len());
    let mut roots = Vec::new();

    let started = Instant::now();

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

    println!("adding vertices + edges took {:?}", started.elapsed());
    println!("vertices = {}", inverse_deps.len());

    let started = Instant::now();

    let sorted = topological_sort(&roots, |name| inverse_deps[name].iter().copied()).unwrap();

    println!("topological sort took {:?}", started.elapsed());
    println!("{sorted:?}");
}
