use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader};
use petgraph::graph::NodeIndex;
use petgraph::dot::{Dot, Config};
use petgraph::prelude::{EdgeRef, StableUnGraph};
use petgraph::stable_graph::Edges;
use petgraph::Undirected;

fn main() {
    part_1();
}

fn part_1() {
    let mut graph = Graph::from_reader(&mut read_file());
    // println!("{:?}", graph.graphviz_representation());

    // Resolved with graphviz, I wish I knew how to programmatically solve this
    graph.remove_edge("gzr", "qnz");
    graph.remove_edge("pgz", "hgk");
    graph.remove_edge("lmj", "xgs");

    let largest_connection = graph.get_largest_amount_of_connections().unwrap();
    println!("Largest connection is {}", largest_connection);

    let clusters = graph.get_clusters(|e| true);
    let cluster_counts= clusters.iter().map(|c| c.len()).collect::<Vec<_>>();
    println!("{:?}", cluster_counts);
    println!("Result: {:?}", cluster_counts.into_iter().reduce(|acc, e| acc * e));
}

fn read_file() -> impl BufRead {
    let path = std::env::current_dir().unwrap().join("day_25/input.txt");
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

    fn get_largest_amount_of_connections(&self) -> Option<usize> {
        self.graph.node_indices()
            .map(|n| self.graph.edges(n).count())
            .max()
    }
    
    fn get_clusters<P: Fn(Edges<(), Undirected>) -> bool>(&self, filter: P) -> Vec<HashSet<NodeIndex>> {
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
            queue.push_front(from_node);
            while let Some(next_node) = queue.pop_front() {
                if already_processed.contains(&next_node) {
                    continue;
                }
                current_cluster.insert(next_node);
                already_processed.insert(next_node);

                let next_edges = self.graph.edges(next_node);
                // println!("{} has {} edge/s.", self.graph.node_weight(next_node).unwrap(), next_edges.clone().count());
                if !filter(next_edges.clone()) {
                    continue;
                }

                let next_nodes = next_edges
                    .map(|r| r.target())
                    .filter(|n| !already_processed.contains(&n));
                queue.extend(next_nodes);
            }
            // println!("Found cluster of {} nodes.", current_cluster.len());
            clusters.push(current_cluster);
        }
        clusters
    }
}
