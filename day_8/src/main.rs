use std::collections::HashMap;
use std::io::{stdin, BufRead};
use regex_macro::regex;
use num::integer::lcm;

fn main() {
    let movements_line = stdin().lock().lines().next().unwrap().unwrap();
    let movements: Vec<_> = Movement::parse_many(&movements_line).collect();
    // println!("Movements: {movements:?}");
    
    let nodes = Node::parse_many(stdin().lock());
    let node_map = NodeMap::from_nodes(nodes, movements);
    
    let starting_nodes: Vec<_> = node_map.nodes
        .values()
        .filter(|n| n.id[2] == 'A')
        .collect();
    println!("Starting nodes: {starting_nodes:?}");

    let mut result = 1;
    for node in starting_nodes {
        println!("Calculating from node: {:?}...", node.id);
        let movements_required = node_map.get_movements_required(node);
        println!("Node {:?} requires {} movement/s", node.id, movements_required);
        result = lcm(result, movements_required);
    }
    println!("{}", result);
    /*
    let movements_required: Vec<_> = starting_nodes
        .into_iter()
        .map(|n| node_map.get_movements_required(n))
        .collect();
    println!("Starting nodes: {movements_required:?}");
     */
}

#[derive(Debug)]
struct NodeMap {
    movements: Vec<Movement>,
    nodes: HashMap<NodeId, Node>,
}

impl NodeMap {
    fn from_nodes(nodes: impl Iterator<Item = Node>, movements: Vec<Movement>) -> Self {
        Self {
            nodes: nodes.map(|node| (node.id, node)).collect(),
            movements,
        }
    }
    
    fn get_movements_required(&self, start_node: &Node) -> usize {
        let mut movements_count = 0;
        let mut cur_node = start_node;
        while cur_node.id[2] != 'Z' {
            let cur_movement = *self.movements
                .get(movements_count % self.movements.len())
                .unwrap();
            let next_node_id = cur_node.next_id(cur_movement);
            cur_node = self.nodes.get(&next_node_id).unwrap();
            // println!("{:?}", cur_node.id);
            movements_count += 1;
        }
        movements_count
    }
}

#[derive(Debug, Copy, Clone)]
enum Movement {
    Left,
    Right,
}

impl Movement {
    fn parse_many(s: &str) -> impl Iterator<Item = Movement> + '_ {
        s.chars().map(Movement::parse)
    }

    fn parse(c: char) -> Movement {
        match c {
            'L' => Movement::Left,
            'R' => Movement::Right,
            _ => panic!("Invalid char: {c}"),
        }
    }
}

type NodeId = [char; 3];

#[derive(Debug)]
struct Node {
    id: NodeId,
    left: NodeId,
    right: NodeId,
}

impl Node {
    fn parse_many(input: impl BufRead) -> impl Iterator<Item = Self> {
        input.lines()
            .map(|line| line.unwrap())
            .filter(|line| !line.is_empty())
            .map(|line| Self::parse(&line))
    }
    
    fn parse(s: &str) -> Self {
        let node_regex = regex!(r"(\w+) = \((\w+), (\w+)\)");
        let node_captures = node_regex.captures(s).unwrap();

        let id = Self::parse_id(node_captures.get(1).unwrap().as_str());
        let left = Self::parse_id(node_captures.get(2).unwrap().as_str());
        let right = Self::parse_id(node_captures.get(3).unwrap().as_str());
        
        Self { id, left, right }
    }
    
    fn parse_id(s: &str) -> NodeId {
        s.chars().take(3).collect::<Vec<_>>().try_into().unwrap()
    }
    
    fn next_id(&self, movement: Movement) -> NodeId {
        match movement {
            Movement::Left => self.left,
            Movement::Right => self.right,
        }
    }
}
