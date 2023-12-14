use std::{fmt::Debug, io::{BufRead, stdin}};

fn main() {
    let maps = read_maps(&mut stdin().lock());
    println!("Read {} map/s", maps.len());

    let mut sum = 0;
    for map in maps.iter() {
        if let Some(vertical_value) = map.find_vertical_mirror_index() {
            sum += vertical_value;
        } else if let Some(horizontal_value) = map.find_horizontal_mirror_index() {
            sum += horizontal_value * 100;
        } else {
            panic!("No vertical or horizontal");
        }
    }

    println!("{sum}");
}

fn read_maps(input: &mut impl BufRead) -> Vec<Map> {
    let mut maps = vec![];
    while let Some(map) = Map::try_from(input) {
        maps.push(map);
    }
    maps
}

struct Point2D(usize, usize);

struct Map {
    tiles: Vec<Tile>,
    width: usize,
    height: usize,
}

impl Map {
    fn try_from(input: &mut impl BufRead) -> Option<Self> {
        let mut lines = input.lines().flatten().take_while(|l| !l.is_empty());
        let first_line = match lines.next() {
            Some(line) => line,
            None => return None,
        };

        let mut tiles: Vec<_> = first_line.chars().map(|c| Tile::try_from(c).unwrap()).collect();
        let width = tiles.len();
        let mut height = 1;
        for line in lines {
            tiles.extend(line.chars().map(|c| Tile::try_from(c).unwrap()));
            height += 1;
        }
        Some(Self {
            tiles,
            width,
            height,
        })
    }

    fn get_index(&self, point: Point2D) -> usize {
        point.0 + (self.width * point.1)
    }

    fn get(&self, point: Point2D) -> Option<&Tile> {
        if point.0 >= self.width || point.1 >= self.height {
            return None
        }
        self.tiles.get(self.get_index(point))
    }

    fn find_vertical_mirror_index(&self) -> Option<usize> {
        for i in 1..self.width {
            if self.is_vertical_mirror_index(i) {
                return Some(i);
            }
        }
        None
    }

    fn is_vertical_mirror_index(&self, i: usize) -> bool {
        let mut diffs = 0;
        for (left, right) in Self::mirror_iter(i, self.width) {
            diffs += self.vertical_check(left, right);
            if diffs > 1 {
                return false;
            }
        }
        diffs == 1
    }

    fn vertical_check(&self, i1: usize, i2: usize) -> usize {
        let mut diffs = 0;
        for y in 0..self.height {
            let left_tile = self.get(Point2D(i1, y));
            let right_tile = self.get(Point2D(i2, y));
            if left_tile.is_none() || right_tile.is_none() {
                return diffs;
            }

            if *left_tile.unwrap() != *right_tile.unwrap() {
                diffs += 1;
            }
        }
        return diffs;
    }

    fn find_horizontal_mirror_index(&self) -> Option<usize> {
        for i in 1..self.height {
            if self.is_horizontal_mirror_index(i) {
                return Some(i);
            }
        }
        None
    }

    fn is_horizontal_mirror_index(&self, i: usize) -> bool {
        let mut diffs = 0;
        for (upper, bottom) in Self::mirror_iter(i, self.height) {
            diffs += self.horizontal_check(upper, bottom);
            if diffs > 1 {
                return false;
            }
        }
        diffs == 1
    }

    fn horizontal_check(&self, i1: usize, i2: usize) -> usize {
        let mut diffs = 0;
        for x in 0..self.width {
            let tile1 = self.get(Point2D(x, i1));
            let tile2 = self.get(Point2D(x, i2));
            if tile1.is_none() || tile2.is_none() {
                return diffs;
            }

            if *tile1.unwrap() != *tile2.unwrap() {
                diffs += 1;
            }
        }
        return diffs;
    }

    fn mirror_iter(start: usize, width: usize) -> impl Iterator<Item = (usize, usize)> {
        (0..(width - start))
            .map(move |i| {
                let right = start + i;
                if i < start && right < width {
                    Some((start - i - 1, right))
                } else {
                    None
                }
            })
            .flatten()
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Tile {
    Ground,
    Rock,
}

impl Debug for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Ground => write!(f, "."),
            Self::Rock => write!(f, "#"),
        }
    }
}

impl TryFrom<char> for Tile {
    type Error = char;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(Self::Ground),
            '#' => Ok(Self::Rock),
            _ => Err(value),
        }
    }
}
