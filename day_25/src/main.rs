use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use petgraph::graph::NodeIndex;
use petgraph::dot::{Dot, Config};
use petgraph::prelude::StableUnGraph;

fn main() {
    part_1();
}

fn part_1() {
    let mut graph = Graph::from_reader(&mut read_file());

    let graphviz_representation = format!("{:?}", Dot::with_config(&graph.graph, &[Config::EdgeNoLabel]));
    println!("{}", graphviz_representation);
    // println!("Preview: https://dreampuf.github.io/GraphvizOnline/#{}", urlencoding::encode(&graphviz_representation));
}

fn read_file() -> impl BufRead {
    let path = std::env::current_dir().unwrap().join("day_25/input_test.txt");
    println!("Opening file: {}", path.display());
    let file = File::open(path).unwrap();
    BufReader::new(file)
}

struct Graph {
    graph: StableUnGraph<String, ()>,
    index_map: HashMap<String, NodeIndex>,
}

impl Graph {
    fn from_reader(reader: &mut impl BufRead) -> Self {
        let mut graph = StableUnGraph::<String, ()>::default();
        let mut index_map = HashMap::<String, NodeIndex>::default();
        for line_result in reader.lines() {
            let line = line_result.unwrap();
            let mut split = line.split(':');

            let cur_node_key = split.next().unwrap().trim().trim_start_matches("\u{feff}");
            if !index_map.contains_key(cur_node_key) {
                let index = graph.add_node(cur_node_key.to_owned());
                index_map.insert(cur_node_key.to_owned(), index);
            }
            let cur_node_index = index_map[cur_node_key];

            for other_node_key in split.next().unwrap().trim().split(' ') {
                if !index_map.contains_key(other_node_key) {
                    let index = graph.add_node(other_node_key.to_owned());
                    index_map.insert(other_node_key.to_owned(), index);
                }
                let other_node_index = index_map[other_node_key];
                graph.add_edge(cur_node_index, other_node_index, ());
            }
        }
        Self {
            graph,
            index_map,
        }
    }

    fn get_index(&self, key: &str) -> Option<&NodeIndex> {
        self.index_map.get(key)
    }

    fn remove_edge(&self, a: &str, b: &str) {
        let a_index = self.get_index(a).unwrap();
        let b_index = self.get_index(b).unwrap();
        // self.graph.remove_edge(a_index, b_index)
    }
}
