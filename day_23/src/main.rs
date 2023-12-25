use std::cmp::max;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::thread;
use aoc_shared::coords2d::Coords2D;
use aoc_shared::direction::Direction;
use aoc_shared::map2d::Map2D;

fn main() {
    // part_1();
    part_2();
}

fn part_1() {
    let map = TileMap::try_from_reader(&mut read_file());
    let start = map.get_start_position().unwrap();
    let end = map.get_end_position().unwrap();
    println!("Trying to find path between {:?} -> {:?}", start, end);

    let builder = thread::Builder::new()
        .name("Pathfinder".into())
        .stack_size(32 * 1024 * 1024); // 32MB of stack space

    let handler = builder.spawn(move || {
        // stack-intensive operations
        map.find_longest_path(start, Direction::South, end)
    }).unwrap();

    let result = handler.join().unwrap();
    println!("Result: {result:?}");
}

fn part_2() {
    let map = TileMap::try_from_reader(&mut read_file());
    println!("Building node graph.");
    let node_graph = map.calculate_node_graph();
    println!("{} nodes:", node_graph.nodes.len());
    // println!("{:#?}", node_graph);

    let start = map.get_start_position().unwrap();
    let end = map.get_end_position().unwrap();
    let longest = node_graph.find_longest_path(start, end);
    println!("Result: {longest:?}");
}

fn read_file() -> impl BufRead {
    let path = std::env::current_dir().unwrap().join("day_23/input.txt");
    println!("Opening file: {}", path.display());
    let file = File::open(path).unwrap();
    BufReader::new(file)
}

#[derive(Debug, Eq, PartialEq)]
enum Tile {
    Ground,
    Forest,
    Slope(Direction),
}

impl TryFrom<char> for Tile {
    type Error = char;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(Self::Ground),
            '#' => Ok(Self::Forest),
            '^' => Ok(Self::Slope(Direction::North)),
            '>' => Ok(Self::Slope(Direction::East)),
            'v' => Ok(Self::Slope(Direction::South)),
            '<' => Ok(Self::Slope(Direction::West)),
            _ => Err(value),
        }
    }
}

impl Tile {
    fn is_walkable(&self, direction: Direction) -> bool {
        // println!("Checking {:?} with {:?}", self, direction);
        match self {
            Tile::Ground => true,
            Tile::Forest => false,
            Tile::Slope(d) => *d == direction, 
        }
    }

    fn is_any_walkable(&self) -> bool {
        // println!("Checking {:?}", self);
        match self {
            Tile::Forest => false,
            _ => true,
        }
    }
}

struct TileMap(Map2D<Tile>);

impl TileMap {
    fn try_from_reader(reader: &mut impl BufRead) -> Self {
        let map = Map2D::try_from_reader(reader).unwrap().unwrap();
        Self(map)
    }
    
    fn get_start_position(&self) -> Option<Coords2D> {
        (0..self.0.width())
            .map(|x| Coords2D(x, 0))
            .find(|&point| self.0.get(point).is_some_and(|t| t == &Tile::Ground))
    }

    fn get_end_position(&self) -> Option<Coords2D> {
        let last_y = self.0.height() - 1;
        (0..self.0.width())
            .map(|x| Coords2D(x, last_y))
            .find(|&point| self.0.get(point).is_some_and(|t| t == &Tile::Ground))
    }
    
    fn find_longest_path(
        &self, from: Coords2D, direction: Direction, destination: Coords2D,
    ) -> Option<usize> {
        // println!("Starting path from {:?} towards {:?}", from, direction);
        let mut cur_pos = from;
        let mut steps = 0;
        let left = direction.turn_left();
        let right = direction.turn_right();
        loop {
            // println!("- {:?}", cur_pos);
            if cur_pos == destination {
                return Some(steps);
            }

            // Here the paths potentially branch and differ
            let left_walk = self.get_walkable_tile_from(cur_pos, left);
            let right_walk = self.get_walkable_tile_from(cur_pos, right);
            if left_walk.is_some() || right_walk.is_some() {
                let branches = [
                    left_walk.map(|branch| (left, branch)),
                    right_walk.map(|branch| (right, branch)),
                    cur_pos.try_move_one(direction).map(|branch| (direction, branch)),
                ];
                // println!("Branching: {:?}", branches);
                return branches
                    .iter()
                    .flatten()
                    .map(|(d, b)| self.find_longest_path(*b, *d, destination))
                    .flatten()
                    .max()
                    .map(|max_branch| max_branch + steps + 1)
            }
            
            if !self.is_walkable(cur_pos, direction) {
                return None;
            }
            
            cur_pos = match cur_pos.try_move_one(direction) {
                Some(p) => p,
                None => return None,
            };
            steps += 1;
        }
    }

    fn find_longest_path_ignoring_slopes(
        &self,
        from: Coords2D,
        direction: Direction,
        destination: Coords2D,
        visited: &mut HashSet<Coords2D>,
    ) -> Option<usize> {
        // println!("Starting path from {:?} towards {:?}", from, direction);
        let mut cur_pos = from;
        let mut steps = 0;
        let left = direction.turn_left();
        let right = direction.turn_right();
        loop {
            // println!("- {:?}", cur_pos);
            if cur_pos == destination {
                return Some(steps);
            }
            
            if visited.contains(&cur_pos) {
                return None;
            }
            visited.insert(cur_pos);

            // Here the paths potentially branch and differ
            let left_walk = self.is_any_walkable_tile_from(cur_pos, left);
            let right_walk = self.is_any_walkable_tile_from(cur_pos, right);
            if left_walk.is_some() || right_walk.is_some() {
                let branches = [
                    left_walk.map(|branch| (left, branch)),
                    right_walk.map(|branch| (right, branch)),
                    cur_pos.try_move_one(direction).map(|branch| (direction, branch)),
                ];
                // println!("Branching: {:?}", branches);
                return branches
                    .iter()
                    .flatten()
                    .map(|(d, b)| {
                        let mut local_visited = visited.clone();
                        self.find_longest_path_ignoring_slopes(*b, *d, destination, &mut local_visited)
                    })
                    .flatten()
                    .max()
                    .map(|max_branch| max_branch + steps + 1)
            }

            if !self.is_any_walkable(cur_pos) {
                return None;
            }

            cur_pos = match cur_pos.try_move_one(direction) {
                Some(p) => p,
                None => return None,
            };
            steps += 1;
        }
    }

    fn get_walkable_tile_from(&self, point: Coords2D, direction: Direction) -> Option<Coords2D> {
        point.try_move_one(direction)
            .and_then(|p| self.is_walkable(point, direction).then_some(p))
    }
    
    fn is_walkable(&self, point: Coords2D, direction: Direction) -> bool {
        self.0.get(point).is_some_and(|t| t.is_walkable(direction))
    }

    fn is_any_walkable_tile_from(&self, point: Coords2D, direction: Direction) -> Option<Coords2D> {
        point.try_move_one(direction)
            .filter(|&p| self.is_any_walkable(p))
    }

    fn is_any_walkable(&self, point: Coords2D) -> bool {
        // println!("Testing {:?} ({:?})", point, self.0.get(point));
        self.0.get(point).is_some_and(|t| t.is_any_walkable())
    }

    fn calculate_node_graph(&self) -> PointsGraph {
        let mut graph = PointsGraph::default();
        let starting_node = self.get_start_position().expect("No starting position.");
        graph.add_node(starting_node);
        self.calculate_node_graph_internal(starting_node, Direction::South, &mut graph);
        graph
    }
    
    fn calculate_node_graph_internal(&self, from: Coords2D, direction: Direction, graph: &mut PointsGraph) {
        // println!("Starting node calculation {:?} towards {:?}", from, direction);
        let mut cur_pos = from.try_move_one(direction).unwrap();
        let mut cur_direction = direction;
        let mut steps = 1;
        
        loop {
            let branches = self.get_possible_branches(cur_pos, cur_direction);
            // println!("{:?} -> {:?}", (cur_pos, cur_direction), branches.iter().flatten().collect::<Vec<_>>());
            
            let branches_count = branches.iter().flatten().count();
            if branches_count > 1 {
                let new_node = cur_pos;
                let node_inserted = graph.add_node(new_node);
                graph.set_two_way_connection(from, new_node, steps);
                if node_inserted {
                    for &(_, new_dir) in branches.iter().flatten() {
                        // println!("{cur_pos} -> {new_dir:?}");
                        self.calculate_node_graph_internal(
                            new_node,
                            new_dir,
                            graph,
                        );
                    }
                }
                return;
            }
            
            if branches_count == 0 {
                graph.add_node(cur_pos);
                graph.set_two_way_connection(from, cur_pos, steps);
                return;
            }
            
            let &(next_pos, next_direction) = branches.iter().flatten().next().unwrap();
            cur_pos = next_pos;
            cur_direction = next_direction;
            
            steps += 1;
        }
    }
    
    fn get_possible_branches(
        &self, from: Coords2D, direction: Direction,
    ) -> [Option<(Coords2D, Direction)>; 3] {
        [
            self.try_get_possible_path_towards(from, direction),
            self.try_get_possible_path_towards(from, direction.turn_left()),
            self.try_get_possible_path_towards(from, direction.turn_right()),
        ]
    }
    
    fn try_get_possible_path_towards(
        &self, from: Coords2D, direction: Direction,
    ) -> Option<(Coords2D, Direction)> {
        self.is_any_walkable_tile_from(from, direction)
            .map(|p| (p, direction))
    }
}

#[derive(Debug, Default)]
struct PointsGraph {
    nodes: HashMap<Coords2D, NodeInfo>
}

impl PointsGraph {
    fn add_node(&mut self, key: Coords2D) -> bool {
        if self.nodes.contains_key(&key) {
            return false;
        }
        self.nodes.insert(key, NodeInfo::default());
        return true;
    }
    
    fn contains_key(&self, key: Coords2D) -> bool {
        self.nodes.contains_key(&key)
    }

    fn set_two_way_connection(&mut self, a: Coords2D, b: Coords2D, cost: usize) {
        if !self.nodes.contains_key(&a) {
            panic!("Point \"a\" not found.");
        }
        if !self.nodes.contains_key(&b) {
            panic!("Point \"b\" not found.");
        }
        self.nodes.get_mut(&a).unwrap().connections.insert(b, cost);
        self.nodes.get_mut(&b).unwrap().connections.insert(a, cost);
    }
    
    fn find_longest_path(&self, from: Coords2D, destination: Coords2D) -> Option<(usize, Vec<Coords2D>)> {
        let mut visited = HashSet::default();
        self.find_longest_path_internal(from, destination, &mut visited)
    }

    fn find_longest_path_internal(
        &self, from: Coords2D, destination: Coords2D, visited: &mut HashSet<Coords2D>,
    ) -> Option<(usize, Vec<Coords2D>)> {
        visited.insert(from);

        let mut max_found: Option<usize> = None;
        let cur_node = self.nodes.get(&from).expect("Node with given key not found.");
        let mut path = vec![];
        for (&next_pos, &cost) in cur_node.connections.iter() {
            if next_pos == destination {
                path.clear();
                path.push(from);
                path.push(destination);
                return Some((cost, path));
            }

            if visited.contains(&next_pos) {
                continue;
            }

            if let Some(found_path) = self.find_longest_path_internal(next_pos, destination, &mut visited.clone()) {
                let actual_cost = cost + found_path.0;
                if max_found.is_none() || max_found.is_some_and(|m| m <= actual_cost) {
                    // println!("{:?} -> {} ({} + {})", max_found, actual_cost, cost, found_path.0);
                    max_found = Some(actual_cost);
                    path.clear();
                    path.push(from);
                    path.extend(found_path.1);
                }
            }
        }
        max_found.map(|m| (m, path))
    }
}

#[derive(Debug, Default)]
struct NodeInfo {
    connections: HashMap<Coords2D, usize>,
}

#[derive(Debug, Default)]
struct NodeBreadcrumb {
    total_cost: usize,
    previous: Option<Coords2D>,
}
