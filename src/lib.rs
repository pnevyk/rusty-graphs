use std::collections::{BTreeMap, BTreeSet};

use serde::Deserialize;

mod unsigned_float;

pub use unsigned_float::UF32;

pub const DIJKSTRA_START: &str = "Cape Town";
pub const DIJKSTRA_TARGET: &str = "Murmansk";

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
        const EARTH_RADIUS: f32 = 6371.0;

        let delta_lat = (other.lat - self.lat).to_radians();
        let delta_lon = (other.lon - self.lon).to_radians();

        let lat1 = self.lat.to_radians();
        let lat2 = other.lat.to_radians();

        let a = (delta_lat * 0.5).sin().powi(2)
            + (delta_lon * 0.5).sin().powi(2) * lat1.cos() * lat2.cos();

        let c = 2.0 * a.sqrt().atan2((1.0 - a).sqrt());

        EARTH_RADIUS * c
    }

    pub fn are_connected(&self, other: &Self) -> Option<f32> {
        let d = self.dist(other);
        (d <= Self::MAX_DISTANCE && d > 0.0).then_some(d)
    }
}

#[derive(Debug, Deserialize)]
struct CityRaw {
    #[serde(rename = "Geoname ID")]
    id: u64,
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "Population")]
    pop: u64,
    #[serde(rename = "Coordinates")]
    coords: String,
}

pub fn load_cities() -> Vec<City> {
    let mut rdr = csv::ReaderBuilder::new()
        .delimiter(b';')
        .from_reader(std::fs::File::open("data/cities.csv").unwrap());
    rdr.deserialize()
        .map(|result| result.unwrap())
        .map(|record: CityRaw| {
            let mut coords = record.coords.split(',');
            let lat = coords.next().unwrap().parse().unwrap();
            let lon = coords.next().unwrap().parse().unwrap();

            City {
                id: record.id,
                name: record.name,
                pop: record.pop,
                lat,
                lon,
            }
        })
        .filter(|city| city.pop > 25_000)
        .collect()
}

#[derive(Debug, Clone)]
pub struct Package {
    pub name: String,
    pub deps: BTreeSet<String>,
}

impl Package {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            deps: BTreeSet::new(),
        }
    }

    fn add_dep(&mut self, other: String) {
        if self.name != other {
            self.deps.insert(other);
        }
    }

    pub fn has_dependency(&self, other: &Self) -> bool {
        self.deps.contains(other.name.as_str())
    }
}

pub fn load_tree() -> Vec<Package> {
    let output = std::process::Command::new("cargo")
        .arg("tree")
        .arg("--no-dedupe")
        .args(["--prefix", "depth"])
        .output()
        .unwrap()
        .stdout;
    let output = std::str::from_utf8(&output).unwrap();

    fn parse_line(line: &str) -> (usize, &str) {
        let split = line
            .char_indices()
            .find(|(_, c)| !c.is_ascii_digit())
            .unwrap()
            .0;

        let level = line[..split].parse().unwrap();
        let name = line[split..].split_whitespace().next().unwrap();

        (level, name)
    }

    let mut lines = output.split('\n');
    let root_line = lines.next().unwrap();

    let (mut last_level, name) = parse_line(root_line);
    assert_eq!(last_level, 0);

    let mut stack = vec![Package::new(name)];
    let mut packages = BTreeMap::new();

    for line in lines {
        if line.is_empty() {
            continue;
        }

        let (level, name) = parse_line(line);

        if level <= last_level {
            while stack.len() > level {
                let p = stack.pop().unwrap();
                let q = packages
                    .entry(p.name.clone())
                    .or_insert_with(|| Package::new(&p.name));

                for other in p.deps {
                    q.add_dep(other);
                }
            }
        }

        stack.last_mut().unwrap().add_dep(name.to_string());
        stack.push(Package::new(name));

        last_level = level;
    }

    let p = stack.remove(0);
    packages.insert(p.name.clone(), p);

    packages.into_values().collect()
}
