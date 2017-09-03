use picto::pixel::Read;
use picto::buffer::Buffer;
use picto::color::Rgba;
use picto::{write, read};
use picto::processing::prelude::*;
use picto::buffer::Rgba as RgbaImage;

use std::io;
use std::path::Path;
use std::ops::{Index, IndexMut};

#[derive(Debug)]
pub struct Grid {
    width: u32,
    height: u32,
    tiles: Vec<Tile>,
    textures: Vec<RgbaImage>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Tile {
    Wall(usize),
    Floor,
}

impl Grid {
    pub fn new(w: u32, h: u32) -> Self {
        if w == 0 || h == 0 {
            panic!("Width and height must be greater than 0");
        }

        let wall_bytes = include_bytes!("../resources/Tileable10c.png").to_vec();
        let wall_image = read::from_memory(wall_bytes).unwrap();

        Grid {
            width: w,
            height: h,
            tiles: vec![Tile::Wall(0); (w * h) as usize],
            textures: vec![wall_image],
        }
    }

    pub fn get(&self, x: u32, y: u32) -> Option<&Tile> {
        self.tiles.get((x + (self.width * y)) as usize)
    }

    pub fn get_mut(&mut self, x: u32, y: u32) -> Option<&mut Tile> {
        self.tiles.get_mut((x + (self.width * y)) as usize)
    }

    pub fn openings(&self, x: u32, y: u32) -> Option<u32> {
        self.get(x, y).and_then(|tile| {
            match *tile {
                Tile::Floor => {
                    let mut count = 0;

                    if x == 0 {
                        count += 1;
                    } else {
                        if x == self.width - 1{
                            count += 1;
                        } else {
                            if let Tile::Floor = self[(x-1, y)] { count += 1; }
                            if let Tile::Floor = self[(x+1, y)] { count += 1; }
                        }
                    }

                    if y == 0 {
                        count += 1;
                    } else {
                        if y == self.height - 1{
                            count += 1;
                        } else {
                            if let Tile::Floor = self[(x, y+1)] { count += 1; }
                            if let Tile::Floor = self[(x, y-1)] { count += 1; }
                        }
                    }

                    Some(count)
                },
                Tile::Wall(_texture) => {
                    None
                },
            }
        })
    }

    pub fn render_image<P: AsRef<Path>>(&self, path: P) -> io::Result<()> {
        let white = Rgba::new(1.0, 1.0, 1.0, 1.0);
        let black = Rgba::new(0.0, 0.0, 0.0, 1.0);
        let mut image: Buffer<_, f32, _> = Buffer::from_pixel(self.width, self.height, &black);

        for x in 0..self.width {
            for y in 0..self.height {
                if let Tile::Floor = self[(x, y)] {
                    image.set(x as u32, y as u32, &white);
                }
            }
        }

        let resized = image.scale_by::<scaler::Nearest>(5.0);
        write::to_path(path, &resized);

        Ok(())
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }
}

impl Index<(u32, u32)> for Grid {
    type Output = Tile;

    fn index(&self, (x, y): (u32, u32)) -> &Tile {
        self.get(x, y).unwrap()
    }
}

impl IndexMut <(u32, u32)> for Grid {
    fn index_mut(&mut self, (x, y): (u32, u32)) -> &mut Tile {
        self.get_mut(x, y).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get() {
        let grid = Grid::new(3, 3, Tile::Wall);
        assert_eq!(grid.get(1, 1), Some(&Tile::Wall));
        assert_eq!(grid.get(2, 2), Some(&Tile::Wall));
        assert_eq!(grid.get(2, 3), None);
        assert_eq!(grid.get(3, 2), None);
    }

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
