use std::collections::HashMap;
mod io;
mod network;

fn main() {
    let network = network::create("melilla-latest.osm.pbf");
    network::save(network);
    // let result = network::load("routing.db");
}

// Tests
const TEST_ID: i64 = 966590652;

fn test(network: HashMap<i64, network::Edge>) {
    // Test
    for (id, edge) in network.iter() {
        // For testing purposes
        if edge.osm_id == TEST_ID {
            println!("Final: {}, {:?}", id, edge);
            println!("")
        }
    }
}
