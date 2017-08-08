use rand::{Rng, SeedableRng};
use rand::isaac::IsaacRng;
use rand::distributions::{Weighted, WeightedChoice, IndependentSample};

use image::{Rgba, DynamicImage, GenericImage};

use std::path::Path;

#[derive(Serialize, Deserialize, Debug)]
pub struct Dungeon {
    tiles: Vec<Vec<Tile>>,
}

impl Dungeon {
    pub fn new_from_seed(seed: &str) -> Self {
        let seed = seed.as_bytes().to_owned().iter().map(|n| *n as u32).collect::<Vec<u32>>();
        let mut rng = IsaacRng::from_seed(&seed);

        let sizes = &mut [
            Weighted { weight: 300, item: DungeonSize::Small },
            Weighted { weight: 100, item: DungeonSize::Med },
            Weighted { weight: 50, item: DungeonSize::Large },
        ];
        let choice = WeightedChoice::new(sizes);
        let size = choice.ind_sample(&mut rng);

        let h: usize;
        let w: usize;

        match size {
            DungeonSize::Small => {
                h = rng.gen_range(100, 301);
                w = rng.gen_range(100, 301);
            },
            DungeonSize::Med => {
                h = rng.gen_range(300, 501);
                w = rng.gen_range(300, 501);
            },
            DungeonSize::Large => {
                h = rng.gen_range(500, 701);
                w = rng.gen_range(500, 701);
            },
        }
        println!("{}x{}", h, w);
        let mut tiles = vec![vec![Tile::Wall; h]; w];

        let attempts = rng.gen_range(150, 301);
        let mut rooms: Vec<Room> = Vec::new();

        'insert: for _ in 0..attempts {
            let width = rng.gen_range(10, 51);
            let height = rng.gen_range(10, 51);
            let x = rng.gen_range(0, w - width + 1);
            let y = rng.gen_range(0, h - height + 1);

            let room = Room::new(
                x,
                y,
                width,
                height,
            );

            for r in &rooms {
                if r.intersects(&room) {
                    continue 'insert;
                }
            }

            rooms.push(room);
        }

        for room in rooms {
            for x in room.x1()..(room.x1() + room.w) {
                for y in room.y1()..(room.y1() + room.h) {
                    tiles[x][y] = Tile::Floor;
                }
            }
        }

        Dungeon {
            tiles: tiles,
        }
    }

    pub fn render_image<P: AsRef<Path>>(&self, path: P) {
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

        let _ = image.to_rgba().save(path);
    }
}

#[derive(Serialize, Deserialize, Clone)]
enum DungeonSize {
    Small,
    Med,
    Large,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
enum Tile {
    Wall,
    Floor,
}

struct Room {
    x: usize, // X coordinate of top-left
    y: usize, // Y coordinate of top-left
    w: usize,
    h: usize,
}

impl Room {
    pub fn new(x: usize, y: usize, w: usize, h: usize) -> Self {
        Room {
            x: x,
            y: y,
            w: w,
            h: h,
        }
    }

    pub fn x1(&self) -> usize {
        self.x
    }

    pub fn x2(&self) -> usize {
        self.x + self.w
    }

    pub fn y1(&self) -> usize {
        self.y
    }

    pub fn y2(&self) -> usize {
        self.y + self.h
    }

    fn intersects(&self, other: &Room) -> bool {
        self.x1() <= other.x2() && self.x2() >= other.x1() &&
            self.y1() <= other.y2() && other.y2() >= other.y1()
    }
}
