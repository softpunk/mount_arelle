use image::{Rgba, DynamicImage, GenericImage};

use std::io;
use std::path::Path;
use std::ops::{Index, IndexMut};

#[derive(Serialize, Deserialize, Debug)]
pub struct Grid {
    // First Vec is x, second Vec is Y:
    // [[(0,0), (0,1), (0,2)],
    //  [(1,0), (1,1), (1,2)],
    //  [(2,0), (1,1), (2,2)]]
    tiles: Vec<Vec<Tile>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Tile {
    Wall,
    Floor,
}

impl Grid {
    pub fn new(w: u32, h: u32, tiles: Tile) -> Self {
        if w == 0 || h == 0 {
            panic!("Width and height must be greater than 0");
        }

        Grid {
            tiles: vec![vec![tiles; h as usize]; w as usize],
        }
    }

    pub fn get(&self, x: u32, y: u32) -> Option<&Tile> {
        self.tiles.get(x as usize).and_then(|row| row.get(y as usize))
    }

    pub fn get_mut(&mut self, x: u32, y: u32) -> Option<&mut Tile> {
        self.tiles.get_mut(x as usize).and_then(|row| row.get_mut(y as usize))
    }

    pub fn openings(&self, x: u32, y: u32) -> Option<u32> {
        self.get(x, y).and_then(|tile| {
            match *tile {
                Tile::Floor => {
                    let mut count = 0;

                    if x == 0 {
                        count += 1;
                    } else {
                        if x == self.tiles.len() as u32 - 1 {
                            count += 1;
                        } else {
                            if let Tile::Floor = self[(x-1, y)] { count += 1; }
                            if let Tile::Floor = self[(x+1, y)] { count += 1; }
                        }
                    }

                    if y == 0 {
                        count += 1;
                    } else {
                        if y == self.tiles[0].len() as u32 - 1 {
                            count += 1;
                        } else {
                            if let Tile::Floor = self[(x, y+1)] { count += 1; }
                            if let Tile::Floor = self[(x, y-1)] { count += 1; }
                        }
                    }

                    Some(count)
                },
                Tile::Wall => {
                    None
                },
            }
        })
    }

    pub fn render_image<P: AsRef<Path>>(&self, path: P) -> io::Result<()> {
        let path = path.as_ref();

        let width = self.tiles.len() as u32;
        let height = self.tiles[0].len() as u32;
        let mut image = DynamicImage::new_rgb8(width, height);

        let white = Rgba { data: [255u8, 255u8, 255u8, 255u8] };

        for (x, row) in self.tiles.iter().enumerate() {
            for (y, tile) in row.iter().enumerate() {
                if let &Tile::Floor = tile {
                    image.put_pixel(x as u32, y as u32, white);
                }
            }
        }

        image.to_rgba().save(path)
    }
}

impl Index<(u32, u32)> for Grid {
    type Output = Tile;

    fn index(&self, (x, y): (u32, u32)) -> &Tile {
        &self.tiles[x as usize][y as usize]
    }
}

impl IndexMut <(u32, u32)> for Grid {
    fn index_mut(&mut self, (x, y): (u32, u32)) -> &mut Tile {
        &mut self.tiles[x as usize][y as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_openings() {
        let mut grid = Grid::new(3, 3, Tile::Wall);
        assert_eq!(grid.openings(1, 1), None);

        grid[(1, 0)] = Tile::Floor;
        grid[(1, 1)] = Tile::Floor;
        grid[(2, 1)] = Tile::Floor;

        assert_eq!(grid.openings(1, 1), Some(2));
        assert_eq!(grid.openings(1, 0), Some(1));
        assert_eq!(grid.openings(2, 1), Some(1));

        grid[(1,2)] = Tile::Floor;
        assert_eq!(grid.openings(1, 1), Some(3));

        grid[(0,0)] = Tile::Floor;
        assert_eq!(grid.openings(1, 1), Some(3));
    }
}
