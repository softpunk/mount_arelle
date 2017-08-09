use image::{Rgba, DynamicImage, GenericImage};

use std::io;
use std::path::Path;
use std::ops::{Index, IndexMut};

#[derive(Serialize, Deserialize, Debug)]
pub struct Grid {
    tiles: Vec<Vec<Tile>>, // First Vec is x, second Vec is Y
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Tile {
    Wall,
    Floor,
}

impl Grid {
    pub fn new(w: usize, h: usize, tiles: Tile) -> Self {
        if w == 0 || h == 0 {
            panic!("Width and height must be greater than 0");
        }

        Grid {
            tiles: vec![vec![tiles; h]; w],
        }
    }

    pub fn get(&self, x: usize, y: usize) -> Option<&Tile> {
        self.tiles.get(x).and_then(|row| row.get(y))
    }

    pub fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut Tile> {
        self.tiles.get_mut(x).and_then(|row| row.get_mut(y))
    }

    pub fn openings(&self, x: usize, y: usize) -> Option<u32> {
        self.get(x, y).and_then(|tile| {
            match *tile {
                Tile::Floor => {
                    let mut count = 0;

                    if x == 0 {
                        count += 1;
                    } else {
                        if x == self.tiles.len() - 1 {
                            count += 1;
                        } else {
                            if let Tile::Floor = self[(x-1, y)] { count += 1; }
                            if let Tile::Floor = self[(x+1, y)] { count += 1; }
                        }
                    }

                    if y == 0 {
                        count += 1;
                    } else {
                        if y == self.tiles[0].len() - 1 {
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

impl Index<(usize, usize)> for Grid {
    type Output = Tile;

    fn index(&self, index: (usize, usize)) -> &Tile {
        let (x, y) = index;
        &self.tiles[x][y]
    }
}

impl IndexMut <(usize, usize)> for Grid {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Tile {
        let (x, y) = index;
        &mut self.tiles[x][y]
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
