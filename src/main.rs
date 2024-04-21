use std::collections::HashMap;
use std::process;
mod io;
mod utils;
mod network;

fn main() {
    let network = network::create("melilla-latest.osm.pbf");
    if let Err(e) = network::save(network.expect("Error creating a routing network")) {  
        eprintln!("{e}");
        process::exit(1);
    };
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
