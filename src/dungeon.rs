use rand::{Rng, SeedableRng};
use rand::isaac::IsaacRng;
use rand::distributions::{Weighted, WeightedChoice, IndependentSample};
use rand::distributions::range::Range;

use std::path::Path;
use std::io;

use grid::Grid;
use grid::Tile;

#[derive(Serialize, Deserialize, Debug)]
pub struct Dungeon {
    grid: Grid,
}

impl Dungeon {
    pub fn new_from_seed(seed: &str) -> Self {
        let seed = seed.as_bytes().iter().map(|n| *n as u32).collect::<Vec<u32>>();
        let mut rng = IsaacRng::from_seed(&seed);

        let sizes = &mut [
            Weighted { weight: 300, item: DungeonSize::Small },
            Weighted { weight: 100, item: DungeonSize::Med },
            Weighted { weight: 50, item: DungeonSize::Large },
        ];
        let choice = WeightedChoice::new(sizes);
        let size = choice.ind_sample(&mut rng);

        let bounds = match size {
            DungeonSize::Small => Range::new(100, 301),
            DungeonSize::Med => Range::new(300, 501),
            DungeonSize::Large => Range::new(500, 701),
        };

        let w = bounds.ind_sample(&mut rng);
        let h = bounds.ind_sample(&mut rng);

        println!("{}x{}", w, h);
        let mut grid = Grid::new(w, h, Tile::Wall);

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
                    grid[(x,y)] = Tile::Floor;
                }
            }
        }

        Dungeon {
            grid: grid,
        }
    }

    pub fn render_grid<P: AsRef<Path>>(&self, path: P) -> io::Result<()> {
        self.grid.render_image(path)
    }
}

#[derive(Serialize, Deserialize, Clone)]
enum DungeonSize {
    Small,
    Med,
    Large,
}

struct Room {
    x: u32, // X coordinate of top-left
    y: u32, // Y coordinate of top-left
    w: u32,
    h: u32,
}

impl Room {
    pub fn new(x: u32, y: u32, w: u32, h: u32) -> Self {
        Room {
            x: x,
            y: y,
            w: w,
            h: h,
        }
    }

    pub fn x1(&self) -> u32 {
        self.x
    }

    pub fn x2(&self) -> u32 {
        self.x + self.w
    }

    pub fn y1(&self) -> u32 {
        self.y
    }

    pub fn y2(&self) -> u32 {
        self.y + self.h
    }

    fn intersects(&self, other: &Room) -> bool {
        self.x1() <= other.x2() && self.x2() >= other.x1() &&
            self.y1() <= other.y2() && other.y2() >= other.y1()
    }
}
