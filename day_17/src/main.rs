use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::fmt::{Debug, Display, Formatter};
use std::fs::File;
use std::hash::Hash;
use std::io::BufReader;
use std::time::{Duration, Instant};
use byteorder::BE;
use aoc_shared::coords2d::Coords2D;
use aoc_shared::direction::{Direction, DIRECTIONS};
use aoc_shared::map2d::Map2D;
use sorted_vec::SortedVec;
use aoc_shared::vector2d::Vector2D;

const COST_PENALTY: usize = 10;
const MIN_STRAIGHT_LINE: usize = 4; // Min is 1
const MAX_STRAIGHT_LINE: usize = 10;

fn main() {
    let path = std::env::current_dir().unwrap().join("day_17/input.txt");
    println!("Opening file: {}", path.display());
    println!();
    let file = File::open(path).unwrap();
    let mut reader = BufReader::new(file);
    
    let map = TileMap::try_from_reader(&mut reader).unwrap().unwrap();
    // println!("{}", map);
    
    part_1(&map);
}

fn part_1(map: &TileMap) {
    let start = Coords2D::ZERO;
    let destination = Coords2D(map.width() - 1, map.height() - 1);
    let path = pathfind_map(&map, start, destination).expect("No path found!");
    // println!("{path:?}");

    println!();
    display_path(map, &path);
    println!();
    
    println!("Path length: {}", path.len());
    let heat_loss: usize = path
        .iter()
        .map(|&pos| map.get(pos).unwrap().cost)
        .sum();
    println!("Heat loss: {}", heat_loss);

}

fn display_path(map: &TileMap, path: &Vec<Coords2D>) {
    let mask = HashSet::<Coords2D>::from_iter(path.iter().cloned());
    for y in 0..map.height() {
        for x in 0..map.width() {
            let point = Coords2D(x, y);
            if mask.contains(&point) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn pathfind_map(map: &TileMap, start: Coords2D, destination: Coords2D) -> Option<Vec<Coords2D>> {
    println!("Travelling from {:?} to {:?}. Distance: {}", start, destination,
             start.manhattan_distance_to(destination));

    let mut open_list = SortedVec::<ReverseCostOrder>::default();
    let mut closed_list = HashMap::<Movement, Breadcrumb>::default();

    let mut insert_instant = Instant::now();
    open_list.extend(DIRECTIONS
        .iter()
        .flat_map(|&d| get_next_points_for_direction(map, start, destination, d))
        .map(ReverseCostOrder)
    );
    let mut open_list_insert_duration = insert_instant.elapsed();
    let mut open_list_insert_amount = open_list.len() as u32;
    // println!("{:?}", open_list.iter().rev().collect::<Vec<_>>());
    
    let mut best_candidate: Option<Breadcrumb> = None;
    

    let mut i = 0;
    'outer_while: while let Some(entry) = open_list.pop() {
        let breadcrumb = entry.0;

        if closed_list.contains_key(&breadcrumb.movement) {
            continue;
        }
        closed_list.insert(breadcrumb.movement, breadcrumb.clone());

        for new_breadcrumb in get_next_points_from_breadcrumb(map, &breadcrumb, destination) {
            if closed_list.contains_key(&new_breadcrumb.movement) {
                continue;
            }
            
            if new_breadcrumb.heuristic_cost != 0 {
                let time = std::time::Instant::now();
                open_list.insert(ReverseCostOrder(new_breadcrumb));
                open_list_insert_duration += time.elapsed();
                open_list_insert_amount += 1;
                continue;
            }

            if best_candidate.as_ref().is_some_and(|c| c.move_cost <= new_breadcrumb.move_cost) {
                continue;
            }
            
            println!("Reached candidate in {} iteration/s!", i + 1);
            println!("Candidate has cost: {}", new_breadcrumb.move_cost / COST_PENALTY);
            closed_list.insert(new_breadcrumb.movement, new_breadcrumb.clone());
            best_candidate = Some(new_breadcrumb.clone());
            break 'outer_while;
        }

        closed_list.insert(breadcrumb.movement, breadcrumb.clone());
        i += 1;

        if (i % 10000) == 0 {
            println!("Iteration {i}:");
            println!("- Open list len: {}", open_list.len());
            println!("- Insert time average: {:?}", open_list_insert_duration / open_list_insert_amount);
            println!("- Insert time total: {:?}", open_list_insert_duration);
            println!("- Closed list len: {}", closed_list.len());
            println!("- Closest node: {:?}", closed_list.values().map(|x| x.heuristic_cost).min());
            println!("- Cheapest next node: {:?}", open_list.iter().map(|x| x.0.final_cost).min());
        }
    }
    
    return best_candidate.map(|b| create_path(&b, closed_list));
    
    fn create_path(
        final_breadcrumb: &Breadcrumb,
        breadcrumbs: HashMap<Movement, Breadcrumb>,
    ) -> Vec<Coords2D> {
        let mut path = vec![];
        let mut cur_breadcrumb = final_breadcrumb;
        loop {
            let mut position = cur_breadcrumb.position();
            let reverse = Vector2D::from(cur_breadcrumb.direction.reverse());
            path.push(position);
            for _ in 1..cur_breadcrumb.moved_amount {
                position = position.try_move(reverse).unwrap();
                path.push(position);
            }
            
            cur_breadcrumb = match cur_breadcrumb.parent {
                Some(movement) => breadcrumbs.get(&movement).unwrap(),
                None => break,
            };
        }
        path.reverse();
        path
    }
}

fn get_next_points_from_breadcrumb<'a>(
    map: &'a TileMap,
    from: &'a Breadcrumb,
    destination: Coords2D,
) -> impl Iterator<Item = Breadcrumb> + 'a {
    get_next_points_for_direction(map, from.position(), destination, from.direction.turn_left())
        .chain(get_next_points_for_direction(map, from.position(), destination, from.direction.turn_right()))
        .map(|b| b.with_previous(from))
}

fn get_next_points_for_direction<'a>(
    map: &'a TileMap,
    from: Coords2D,
    destination: Coords2D,
    direction: Direction,
) -> impl Iterator<Item = Breadcrumb> + 'a {
    let mut current: Option<Coords2D> = Some(from);
    let mut moved_amount = MIN_STRAIGHT_LINE - 1;
    let mut move_cost = 0;
    // println!("{:?} -> {:?} {:?}", from, direction, MIN_STRAIGHT_LINE..MAX_STRAIGHT_LINE);
    for _ in 0..moved_amount {
        let next_pos = match current.and_then(|c| c.try_move_one(direction)) {
            Some(coords) => coords,
            None => {
                current = None;
                break;
            },
        };

        if let Some(tile) = map.get(next_pos) {
            current = Some(next_pos);
            move_cost += tile.cost * COST_PENALTY;
            // println!("Precalculating {:?} cost to be {}", next_pos, move_cost);
        } else {
            current = None;
            break;
        }
    }
    
    std::iter::from_fn(move || {
        if moved_amount >= MAX_STRAIGHT_LINE {
            return None;
        }

        let next_pos = match current.and_then(|c| c.try_move_one(direction)) {
            Some(coords) => coords,
            None => return None,
        };

        if let Some(tile) = map.get(next_pos) {
            current = Some(next_pos);
            move_cost += tile.cost * COST_PENALTY;
            moved_amount += 1;
            // println!("Calculated {:?} cost is {} and moved {} times", next_pos, move_cost, moved_amount);
            let heuristic_cost = next_pos.manhattan_distance_to(destination);
            let cur_movement = Breadcrumb {
                movement: Movement(from, next_pos),
                move_cost,
                heuristic_cost,
                moved_amount,
                direction,
                final_cost: move_cost + heuristic_cost,
                parent: None
            };
            Some(cur_movement)
        } else {
            None
        }
    })
}

#[derive(Debug, Clone)]
struct Breadcrumb {
    movement: Movement,
    move_cost: usize,
    heuristic_cost: usize,
    final_cost: usize,
    moved_amount: usize,
    direction: Direction,
    parent: Option<Movement>,
}

impl Breadcrumb {
    fn position(&self) -> Coords2D {
        self.movement.1
    }
    
    fn with_previous(&self, previous: &Breadcrumb) -> Self {
        let extra_cost = previous.move_cost;
        Self {
            move_cost: self.move_cost + extra_cost,
            final_cost: self.final_cost + extra_cost,
            parent: Some(previous.movement),
            ..self.clone()
        }
    }
}

#[derive(Debug)]
struct ReverseCostOrder(pub Breadcrumb);

impl Eq for ReverseCostOrder {}

impl PartialEq<Self> for ReverseCostOrder {
    fn eq(&self, other: &Self) -> bool {
        self.0.final_cost.eq(&other.0.final_cost)
    }
}

impl PartialOrd<Self> for ReverseCostOrder {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

impl Ord for ReverseCostOrder {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.final_cost.cmp(&other.0.final_cost).reverse()
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
struct Movement(Coords2D, Coords2D);

impl Movement {
    fn heuristic_cost(&self, destination: Coords2D) -> usize {
        self.1.manhattan_distance_to(destination)
    }
}

impl Debug for Movement {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({:?} -> {:?})", self.0, self.1)
    }
}

#[derive(Debug)]
struct Tile {
    cost: usize,
}

impl Display for Tile {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.cost)
    }
}

impl TryFrom<char> for Tile {
    type Error = &'static str;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        const RADIX: u32 = 10;
        let cost = value.to_digit(RADIX).ok_or("Expected a digit")?;
        Ok(Self { cost: cost as usize })
    }
}

type TileMap = Map2D<Tile>;
