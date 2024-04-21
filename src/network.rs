use geo::{coord, GeodesicDistance, Point};
use rusqlite::Connection;
use std::collections::HashMap;
//use std::io::{self, Read};
use crate::io::open_osmpbf;
use crate::utils::RoutyResult;

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

#[derive(Clone, Debug)]
pub struct Network {
    pub id: i64,
    pub way_id: i64,
    pub node1_id: i64,
    pub node1_lat: f64,
    pub node1_lon: f64,
    pub node2_id: i64,
    pub node2_lat: f64,
    pub node2_lon: f64,
    pub weight: f64,
}

pub fn create(pbf_file: &str) -> RoutyResult<HashMap<i64, Edge>> {
    let (osm_ways, osm_nodes) = open_osmpbf(pbf_file);
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
    Ok(routing_edges)
}

pub fn save(network: HashMap<i64, Edge>) {
    // get current working dir
    let conn = Connection::open("./routing.db").expect("Error opening db");
    conn.execute(
        "CREATE TABLE routing (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            way_id INTEGER,
            node1_id INTEGER,
            node1_lat REAL,
            node1_lon REAL,
            node2_id INTEGER,
            node2_lat REAL,
            node2_lon REAL,
            weight INTEGER
        )",
        (),
    )
    .expect("Error creating table");

    for (_, edge) in network.iter() {
        let n1 = edge.nodes.first().expect("Error getting node one");
        let n2 = edge.nodes.last().expect("Error getting node two");
        conn.execute(
            "INSERT INTO routing (way_id, node1_id, node1_lat, node1_lon, node2_id, node2_lat, node2_lon, weight) \
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
            (&edge.osm_id, &n1.osm_id,
             &n1.lat, &n1.lon,
             &n2.osm_id, &n2.lat,
             &n2.lon, &edge.weight),
        ).expect("Error inserting data");
    }
}

pub fn load(network_name: &str) -> Result<Vec<Network>, rusqlite::Error> {
    let conn = Connection::open(network_name).expect("Error opening db");
    let mut data = conn.prepare("SELECT id, way_id, node1_id, node1_lat, node1_lon, node2_id, node2_lat, node2_lon, weight FROM routing;")?;
    let data_iter = data.query_map([], |row| {
        Ok(Network {
            id: row.get(0)?,
            way_id: row.get(1)?,
            node1_id: row.get(2)?,
            node1_lat: row.get(3)?,
            node1_lon: row.get(4)?,
            node2_id: row.get(5)?,
            node2_lat: row.get(6)?,
            node2_lon: row.get(7)?,
            weight: row.get(8)?,
        })
    })?;
    let networks = data_iter.collect();
    networks
}
