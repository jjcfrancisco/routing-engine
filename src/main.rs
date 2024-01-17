//use osmpbf::{ElementReader, Element};
use geo::{coord, Coord, LineString, Point, GeodesicDistance};
use osmpbf::*;
use std::collections::HashMap;

#[derive(Debug)]
struct Highway {
    id: i64,
    refs: Vec<i64>,
    tags: HashMap<String, String>,
    geom: Option<LineString>,
}

#[derive(Clone, Debug)]
struct Dependencies {
    lat: f64,
    lon: f64,
}

#[derive(Debug)]
struct Edge {
    id: usize,
    osm_id: i64,
    geom: LineString,
    weight: f64,
}

fn open_osmpbf(area: &str) -> Vec<Edge> {
    let filepath = format!("{}-latest.osm.pbf", area);
    let mut reader = IndexedReader::from_path(filepath).expect("Error opening file");
    let mut ways: Vec<Highway> = Vec::new();
    let mut deps: HashMap<i64, Dependencies> = HashMap::new();

    reader
        .read_ways_and_deps(
            |way| way.tags().any(|(key, _)| key == "highway"),
            |element| {
                match element {
                    Element::Way(w) => {
                        let mut refs = Vec::new();
                        let mut tags = HashMap::new();
                        for r in w.refs() {
                            refs.push(r)
                        }
                        for t in w.tags() {
                            tags.insert(t.0.to_owned(), t.1.to_owned());
                        }
                        ways.push(Highway {
                            id: w.id(),
                            refs,
                            tags,
                            geom: None,
                        });
                    }
                    Element::Node(n) => {
                        deps.insert(
                            n.id(),
                            Dependencies {
                                lat: n.lat(),
                                lon: n.lon(),
                            },
                        );
                    }
                    Element::DenseNode(dn) => {
                        deps.insert(
                            dn.id(),
                            Dependencies {
                                lat: dn.lat(),
                                lon: dn.lon(),
                            },
                        );
                    }
                    Element::Relation(_) => (),
                };
            },
        )
        .expect("Error reading file");

    let mut edges: Vec<Edge> = Vec::new();
    for way in ways.iter_mut() {
        let mut nodes: Vec<Dependencies> = Vec::new();
        let mut contains = 0;
        let mut temp_geom: Vec<Coord> = Vec::new();
        // imagine it has 3 items
        for (id, r) in way.refs.iter().enumerate() {
            // First: contains = 0
            // Second: contains = 1
            // Third: contains = 1
            if contains == 0 {
                // First: pushes 1+0 node
                nodes.push(deps.get(r).expect("Error finding node").to_owned());
                contains += 1;
            } else if contains == 1 {
                // Second: pushes 1+1 node
                // Third: pushes 1+1 (previously 0+1 which became 1+0)
                nodes.push(deps.get(r).expect("Error finding node").to_owned());
                // Second: creates the ls
                // Third: creates the ls
                for node in nodes.iter() {
                    temp_geom.push(coord! {x: node.lat, y: node.lon})
                }
                let p1: Point = temp_geom
                    .first()
                    .expect("Error getting geom element")
                    .to_owned()
                    .into();
                let p2: Point = temp_geom
                    .last()
                    .expect("Error getting geom element")
                    .to_owned()
                    .into();
                edges.push(Edge {
                    id,
                    osm_id: way.id,
                    geom: LineString::new(temp_geom.to_owned()),
                    weight: p1.geodesic_distance(&p2),
                });
                // Second: removes node so it's again 0+1 which gets pushed to 1+0 and contains 1.
                // Third: removes node so it's again 0+1
                nodes.remove(0);
                temp_geom.clear();
                contains = 1;
            } else {
            }
        }
        break
    }

    edges

}

fn main() {
    let edges = open_osmpbf("melilla");
    for edge in edges.iter() {
        println!("{:?}", edge);
        println!("")
    }
}

//if w.id() == 966521787 {
//    let refs = w.refs();
//    for r in refs.into_iter() {
//        println!("{:?}", r)
//    }
//    println!("{:?} - {:?}", w.id(), w.refs());
//    1
//} else {
//    1
//}
