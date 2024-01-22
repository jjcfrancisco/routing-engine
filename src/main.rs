//use osmpbf::{ElementReader, Element};
use geo::{coord, Coord, GeodesicDistance, LineString, Point};
use osmpbf::*;
use std::{collections::HashMap, os};

// For testing purposes
const TEST_ID:i64 = 966590652;

#[derive(Debug)]
struct OSMWay {
    id: i64,
    refs: Vec<i64>,
    tags: HashMap<String, String>,
}

#[derive(Clone, Debug)]
struct OSMNode {
    id: i64,
    lat: f64,
    lon: f64,
}

#[derive(Clone, Debug)]
struct Node {
    osm_id: i64,
    lat: f64,
    lon: f64,
}

#[derive(Debug)]
struct Edge {
    osm_id: i64,
    nodes: Vec<Node>,
    weight: f64,
}

fn open_osmpbf(area: &str) -> (Vec<OSMWay>, HashMap<i64, OSMNode>) {
    let filepath = format!("{}-latest.osm.pbf", area);
    let mut reader = IndexedReader::from_path(filepath).expect("Error opening file");
    let mut ways = Vec::<OSMWay>::new();
    let mut nodes = HashMap::<i64, OSMNode>::new();

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
                        for (key, value) in w.tags() {
                            tags.insert(key.to_owned(), value.to_owned());
                        }
                        ways.push(OSMWay {
                            id: w.id(),
                            refs,
                            tags,
                        });
                    }
                    Element::Node(n) => {
                        nodes.insert(
                            n.id(),
                            OSMNode {
                                id: n.id(),
                                lat: n.lat(),
                                lon: n.lon(),
                            },
                        );
                    }
                    Element::DenseNode(dn) => {
                        nodes.insert(
                            dn.id(),
                            OSMNode {
                                id: dn.id(),
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

    (ways, nodes)
}

fn process(osm_ways: Vec<OSMWay>, osm_nodes: HashMap<i64, OSMNode>) -> HashMap<i64, Edge> {

    let mut routing_edges = HashMap::<i64, Edge>::new();
    let mut id = 1;
    for osm_way in osm_ways.iter() {
        // For testing purposes
        if osm_way.id == TEST_ID {
            println!("Raw OSM: {:?}", osm_way);
            println!("");
        }
        let mut contains = 0;
        let mut previous_node: Option<OSMNode> = None;
        for r in osm_way.refs.iter() {
            if contains == 0 {
                previous_node = Some(osm_nodes.get(&r).expect("Error finding node").to_owned());
                contains += 1;
            } else if contains == 1 {
                if previous_node.is_some() {
                    let pn = previous_node.clone().unwrap();
                    let cn = osm_nodes.get(&r).expect("Error finding node").to_owned();
                    let n1 = Node{osm_id: pn.id, lat: pn.lat, lon: pn.lon};
                    let n2 = Node{osm_id: cn.id, lat: cn.lat, lon: cn.lon};
                    let p1:Point = coord! {x: pn.lat, y: pn.lon}.into();
                    let p2:Point = coord! {x: cn.lat, y: cn.lon}.into();
                    let weight = p1.geodesic_distance(&p2);
                    routing_edges.insert(
                        id,
                        Edge {
                            osm_id: osm_way.id,
                            nodes: vec![n1,n2],
                            weight,
                        },
                    );
                    id += 1;
                    contains = 1;
                    previous_node = Some(cn);
                    //if osm_way.id == 966521787 {println!("{:?}", routing_edges.get(&id))};
                }
            } else {
            }
        }
    }
    routing_edges
}

fn main() {
    let (ways, nodes) = open_osmpbf("melilla");
    let edges = process(ways, nodes);
    for (id, edge) in edges.iter() {
        // For testing purposes
        if edge.osm_id == TEST_ID {
            println!("Final: {}, {:?}", id, edge);
            println!("")
        }
    }
}
