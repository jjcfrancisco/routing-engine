use geo::{coord, GeodesicDistance, Point};
use rusqlite::Connection;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Node {
    osm_id: i64,
    lat: f64,
    lon: f64,
}

#[derive(Debug)]
pub struct Edge {
    pub osm_id: i64,
    nodes: Vec<Node>,
    weight: f64,
}

#[derive(Debug)]
pub struct OSMWay {
    pub id: i64,
    pub refs: Vec<i64>,
    pub tags: HashMap<String, String>,
}

#[derive(Clone, Debug)]
pub struct OSMNode {
    pub id: i64,
    pub lat: f64,
    pub lon: f64,
}


pub fn process(osm_ways: Vec<OSMWay>, osm_nodes: HashMap<i64, OSMNode>) -> HashMap<i64, Edge> {
    let mut routing_edges = HashMap::<i64, Edge>::new();
    let mut id = 1;
    for osm_way in osm_ways.iter() {
        // For testing purposes
        //if osm_way.id == TEST_ID {
        //    println!("Raw OSM: {:?}", osm_way);
        //    println!("");
        //}
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
                    let n1 = Node {
                        osm_id: pn.id,
                        lat: pn.lat,
                        lon: pn.lon,
                    };
                    let n2 = Node {
                        osm_id: cn.id,
                        lat: cn.lat,
                        lon: cn.lon,
                    };
                    let p1: Point = coord! {x: pn.lat, y: pn.lon}.into();
                    let p2: Point = coord! {x: cn.lat, y: cn.lon}.into();
                    let weight = p1.geodesic_distance(&p2);
                    routing_edges.insert(
                        id,
                        Edge {
                            osm_id: osm_way.id,
                            nodes: vec![n1, n2],
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

pub fn save(network: HashMap<i64, Edge>) {
    let conn = Connection::open("./routing.db").expect("Error opening db");
    conn.execute(
        "CREATE TABLE routing (
            id INTEGER AUTO_INCREMENT PRIMARY KEY,
            way_id INTEGER,
            node1_id INTEGER,
            lat1 REAL,
            lon1 REAL,
            node2_id INTEGER,
            lat2 REAL,
            lon2 REAL,
            weight INTEGER
        )",
        (),
    ).expect("Error creating table");

    for (_, edge) in network.iter() {
        let n1 = edge.nodes.first().expect("Error getting node one");
        let n2 = edge.nodes.last().expect("Error getting node two");
        conn.execute(
            "INSERT INTO routing (way_id, node1_id, lat1, lon1, node2_id, lat2, lon2, weight) \
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
            (&edge.osm_id, &n1.osm_id,
             &n1.lat, &n1.lon,
             &n2.osm_id, &n2.lat,
             &n2.lon, &edge.weight),
        ).expect("Error inserting data");
    }
}
