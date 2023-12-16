use std::io::BufRead;
use crate::coords2d::Coords2D;

pub type AsciiMap = Map2D<u8>;

pub struct Map2D<T> where T : TryFrom<u8> {
    tiles: Vec<T>,
    width: usize,
    height: usize,
}

#[derive(Debug)]
pub enum ParseMapError<T> where T : TryFrom<u8> {
    IOError(std::io::Error),
    ParseByteError(T::Error),
    InconsistentWidth { expected_width: usize },
}

impl<T: TryFrom<u8>> Map2D<T> {
    pub fn try_from_reader(reader: &mut impl BufRead) -> Option<Result<Self, ParseMapError<T>>> {
        let mut lines = reader.lines();
        let first_line = match lines.next() {
            Some(res) => match res.map_err(ParseMapError::IOError) {
                Ok(l) => l,
                Err(err) => return Some(Err(err)),
            },
            None => return None,
        };
        // We need to read the first line to determine the expected width
        let line_read_result = first_line
            .bytes()
            .map(T::try_from)
            .collect::<Result<Vec<_>, _>>()
            .map_err(ParseMapError::ParseByteError);
        let mut tiles: Vec<_> = match line_read_result {
            Ok(l) => l,
            Err(err) => return Some(Err(err)),
        };
        let width = tiles.len();
        let mut expected_length = width;
        let mut height = 1;
        
        for line_result in lines {
            let line = match line_result.map_err(ParseMapError::IOError) {
                Ok(l) => l,
                Err(err) => return Some(Err(err)),
            };
            
            for byte in line.bytes() {
                let new_tile = match T::try_from(byte) {
                    Ok(t) => t,
                    Err(err) => return Some(Err(ParseMapError::ParseByteError(err))),
                };
                tiles.push(new_tile);
            }
            
            expected_length += width;
            if tiles.len() != expected_length {
                return Some(Err(ParseMapError::InconsistentWidth { expected_width: width }));
            }
            height += 1;
        }
        
        Some(Ok(Self {
            tiles,
            width,
            height
        }))
    }
    
    pub fn coords_are_inside(&self, coords: Coords2D) -> bool {
        coords.0 < self.width && coords.1 < self.height
    }
    
    pub fn get_index(&self, coords: Coords2D) -> Option<usize> {
        if self.coords_are_inside(coords) {
            Some(coords.0 + (self.width * coords.1))
        } else {
            None
        }
    }
    
    pub fn get(&self, coords: Coords2D) -> Option<&T> {
        self.get_index(coords)
            .map(|i| self.tiles.get(i))
            .flatten()
    }

    pub fn get_mut(&mut self, coords: Coords2D) -> Option<&mut T> {
        self.get_index(coords)
            .map(|i| self.tiles.get_mut(i))
            .flatten()
    }
}

#[cfg(test)]
mod tests {
    use std::io::{BufReader, Cursor};
    use crate::coords2d::Coords2D;
    use crate::map2d::AsciiMap;
    
    fn parse_map(s: &str) -> AsciiMap {
        let input = s;
        let mut reader = BufReader::new(Cursor::new(input));
        AsciiMap::try_from_reader(&mut reader).unwrap().unwrap()
    }

    #[test]
    fn can_build_map_correctly() {
        let map = parse_map(&"ABC\nDEF");

        assert_eq!(map.width, 3);
        assert_eq!(map.height, 2);
        assert_eq!(&map.tiles[..], &[b'A', b'B', b'C', b'D', b'E', b'F']);
    }

    #[test]
    fn gets_tile_correctly() {
        let map = parse_map(&"ABC\nDEF");

        let tile = map.get(Coords2D(0, 1)).unwrap();
        
        assert_eq!(tile, &b'D');
    }

    #[test]
    fn gets_mutable_tile_correctly() {
        let mut map = parse_map(&"ABC\nDEF");

        let mut tile = map.get_mut(Coords2D(0, 1)).unwrap();
        *tile = b'Z';

        assert_eq!(tile, &b'Z');
    }
}
