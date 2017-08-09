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

        let dungeon_sizes = &mut [
            Weighted { weight: 300, item: DungeonSize::Small },
            Weighted { weight: 100, item: DungeonSize::Med },
            Weighted { weight: 50, item: DungeonSize::Large },
        ];
        let dungeon_size = WeightedChoice::new(dungeon_sizes).ind_sample(&mut rng);

        let dungeon_bounds = match dungeon_size {
            DungeonSize::Small => Range::new(70, 151),
            DungeonSize::Med => Range::new(150, 251),
            DungeonSize::Large => Range::new(250, 351),
        };

        let dw = dungeon_bounds.ind_sample(&mut rng);
        let dh = dungeon_bounds.ind_sample(&mut rng);

        println!("{}x{}", dw, dh);
        let mut grid = Grid::new(dw, dh, Tile::Wall);

        let attempts = rng.gen_range(500, 1000);
        let mut rooms: Vec<Room> = Vec::new();

        let room_sizes = &mut [
            Weighted { weight: 300, item: RoomSize::Small },
            Weighted { weight: 150, item: RoomSize::Med },
            Weighted { weight: 50, item: RoomSize::Large },
        ];

        'insert: for _ in 0..attempts {
            let room_size = WeightedChoice::new(room_sizes).ind_sample(&mut rng);
            let room_bounds = match room_size {
                RoomSize::Small => Range::new(3, 11),
                RoomSize::Med => Range::new(10, 21),
                RoomSize::Large => Range::new(20, 31),
            };

            let rw = room_bounds.ind_sample(&mut rng);
            let rh = room_bounds.ind_sample(&mut rng);

            let x = rng.gen_range(0, dw - rw + 1);
            let y = rng.gen_range(0, dh - rh + 1);

            let room = Room::new(
                x,
                y,
                rw,
                rh,
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

#[derive(Serialize, Deserialize, Clone)]
enum RoomSize {
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
