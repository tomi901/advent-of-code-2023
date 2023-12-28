use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader};
use petgraph::csr::EdgeIndex;
use petgraph::graph::NodeIndex;
use petgraph::dot::{Dot, Config};
use petgraph::prelude::{EdgeRef, StableUnGraph};

fn main() {
    part_1();
}

fn part_1() {
    let mut graph = Graph::from_reader(&mut read_file());
    
    // graph.remove_edge("bvb", "cmg");
    // graph.remove_edge("hfx", "pzl");
    // graph.remove_edge("jqt", "nvd");

    println!("{:?}", graph.graphviz_representation());
    
    let clusters = graph.get_clusters();
    // println!("{:#?}", graph.get_clusters());
    let clusters_product = clusters
        .iter()
        .map(|c| c.len())
        .reduce(|acc, e| acc * e);
    println!("{:?}", clusters_product);
}

fn read_file() -> impl BufRead {
    let path = std::env::current_dir().unwrap().join("day_25/input_test.txt");
    println!("Opening file: {}", path.display());
    let file = File::open(path).unwrap();
    BufReader::new(file)
}

#[derive(Debug, Default)]
struct Graph {
    graph: StableUnGraph<String, ()>,
    node_map: HashMap<String, NodeIndex>,
    edge_map: HashMap<(String, String), NodeIndex>,
}

impl Graph {
    fn from_reader(reader: &mut impl BufRead) -> Self {
        let mut new = Self::default();
        
        for line_result in reader.lines() {
            let line = line_result.unwrap();
            let mut split = line.split(':');

            let cur_node_key = split.next().unwrap().trim().trim_start_matches("\u{feff}");
            let cur_node_index = new.get_or_add_node(cur_node_key);

            for other_node_key in split.next().unwrap().trim().split(' ') {
                let other_node_index = new.get_or_add_node(other_node_key);
                new.graph.add_edge(cur_node_index, other_node_index, ());
            }
        }
        new
    }
    
    fn get_or_add_node(&mut self, key: &str) -> NodeIndex {
        if !self.node_map.contains_key(key) {
            let index = self.graph.add_node(key.to_owned());
            self.node_map.insert(key.to_owned(), index);
            return index.clone();
        }
        return self.node_map[key].clone();
    }

    fn get_index(&self, key: &str) -> Option<&NodeIndex> {
        self.node_map.get(key)
    }

    fn remove_edge(&mut self, a: &str, b: &str) {
        let a_index = self.get_index(a).unwrap();
        let b_index = self.get_index(b).unwrap();
        let edge = self.graph.find_edge(*a_index, *b_index).unwrap();
        self.graph.remove_edge(edge);
    }
    
    fn graphviz_representation(&self) -> Dot<&StableUnGraph<String, ()>> {
        Dot::with_config(&self.graph, &[Config::EdgeNoLabel])
    }
    
    fn get_clusters(&self) -> Vec<HashSet<NodeIndex>> {
        // This is kinda of a mess, I'm pretty sure it can be more readable
        let mut queue = VecDeque::default();
        let mut clusters = Vec::default();
        let mut already_processed = HashSet::<NodeIndex>::default();
        
        for from_node in self.graph.node_indices() {
            if already_processed.contains(&from_node) {
                continue;
            }
            // println!("Processing from {:?}", self.graph.node_weight(from_node));
            
            let mut current_cluster = HashSet::default();
            current_cluster.insert(from_node);
            already_processed.insert(from_node);
            
            let next = self.graph.edges(from_node)
                .map(|r| [r.source(), r.target()])
                .flatten()
                .filter(|n| !already_processed.contains(&n));
            queue.extend(next);
            while let Some(next_node) = queue.pop_front() {
                if already_processed.contains(&next_node) {
                    continue;
                }
                current_cluster.insert(next_node);
                already_processed.insert(next_node);

                let next = self.graph.edges(next_node)
                    .map(|r| [r.source(), r.target()])
                    .flatten()
                    .filter(|n| !already_processed.contains(&n));
                queue.extend(next);
            }
            // println!("Found cluster of {} nodes.", current_cluster.len());
            clusters.push(current_cluster);
        }
        clusters
    }
}
