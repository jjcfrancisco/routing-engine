use std::collections::HashMap;
mod io;
mod network;

// For testing purposes
const TEST_ID: i64 = 966590652;

fn main() {
    let (ways, nodes) = io::open_osmpbf("melilla"); //wales
    let network = network::process(ways, nodes);
    network::save(network);
}

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
