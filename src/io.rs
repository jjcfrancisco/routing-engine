use std::collections::HashMap;
use osmpbf::{Element, IndexedReader};
use crate::network::{OSMNode, OSMWay};

pub fn open_osmpbf(pbf_file: &str) -> (Vec<OSMWay>, HashMap<i64, OSMNode>) {
    let mut reader = IndexedReader::from_path(pbf_file).expect("Error opening file");
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

// mod tests {
//     use crate::io::open_osmpbf;
//
//     #[test]
//     fn test_open_osmpbf() {
//         let result = open_osmpbf("../tests/fixtures/example.osm.pbf");
//         assert_eq!(result, 4);
//     }
// }
