use std::collections::HashMap;
use std::io::{stdin, BufRead};
use regex_macro::regex;

fn main() {
    let movements_line = stdin().lock().lines().next().unwrap().unwrap();
    let movements: Vec<_> = Movement::parse_many(&movements_line).collect();
    println!("Movements: {movements:?}");
    
    let nodes: HashMap<NodeId, Node> = Node::parse_many(stdin().lock())
        .map(|node| (node.id, node))
        .collect();

    const START_NODE_ID: NodeId = ['A', 'A', 'A'];
    const FINISH_NODE_ID: NodeId = ['Z', 'Z', 'Z'];

    let mut cur_node = nodes.get(&START_NODE_ID).unwrap();
    let mut movements_count = 0;
    
    while cur_node.id != FINISH_NODE_ID {
        let cur_movement = *movements.get(movements_count % movements.len()).unwrap();
        let next_node_id = cur_node.next_id(cur_movement);
        cur_node = nodes.get(&next_node_id).unwrap();
        // println!("{:?}", cur_node.id);
        movements_count += 1;
    }
    
    println!("{movements_count}");
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
