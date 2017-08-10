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
        let mut small = 0;
        let mut med = 0;
        let mut large = 0;

        let seed_bytes = seed.as_bytes().iter().map(|n| *n as u32).collect::<Vec<u32>>();
        let mut rng = IsaacRng::from_seed(&seed_bytes);

        let dungeon_sizes = &mut [
            Weighted { weight: 300, item: DungeonSize::Small },
            Weighted { weight: 250, item: DungeonSize::Med },
            Weighted { weight: 100, item: DungeonSize::Large },
        ];
        let dungeon_size = WeightedChoice::new(dungeon_sizes).ind_sample(&mut rng);

        let dungeon_bounds = match dungeon_size {
            DungeonSize::Small => Range::new(30, 51),
            DungeonSize::Med => Range::new(50, 76),
            DungeonSize::Large => Range::new(75, 101),
        };

        // let dw = dungeon_bounds.ind_sample(&mut rng);
        // let dh = dungeon_bounds.ind_sample(&mut rng);

        let dw = 1000;
        let dh = 1000;

        let mut grid = Grid::new(dw, dh, Tile::Wall);

        let attempts = rng.gen_range(1000, 3001);
        let mut rooms: Vec<Room> = Vec::new();

        'create_rooms: for _ in 0..attempts {
            let mut room_sizes;

            match dungeon_size {
                DungeonSize::Small => {
                    room_sizes = [
                        Weighted { weight: 200, item: RoomSize::Small },
                        Weighted { weight: 150, item: RoomSize::Med },
                        Weighted { weight:   0, item: RoomSize::Large },
                    ];
                },
                DungeonSize::Med => {
                    room_sizes = [
                        Weighted { weight: 200, item: RoomSize::Small },
                        Weighted { weight: 200, item: RoomSize::Med },
                        Weighted { weight:  50, item: RoomSize::Large },
                    ];
                },
                DungeonSize::Large => {
                    room_sizes = [
                        Weighted { weight: 200, item: RoomSize::Small },
                        Weighted { weight: 220, item: RoomSize::Med },
                        Weighted { weight:  80, item: RoomSize::Large },
                    ];
                },
            };

            let room_size = WeightedChoice::new(&mut room_sizes).ind_sample(&mut rng);
            let room_bounds = match room_size {
                RoomSize::Small => Range::new(5, 11),
                RoomSize::Med => Range::new(10, 16),
                RoomSize::Large => Range::new(15, 26),
            };

            let rw = room_bounds.ind_sample(&mut rng);
            let rh = room_bounds.ind_sample(&mut rng);

            'place_room: for _ in 0..20 {
                let x_range = Range::new(rw / 2, dw - (rw / 2) + 1);
                let y_range = Range::new(rh / 2, dh - (rh / 2) + 1);

                let rx = x_range.ind_sample(&mut rng);
                let ry = y_range.ind_sample(&mut rng);

                let room = Room::new(
                    rx,
                    ry,
                    rw,
                    rh,
                );

                for r in &rooms {
                    if r.intersects(&room) {
                        continue 'place_room;
                    }
                }

                match room_size {
                    RoomSize::Small => small += 1,
                    RoomSize::Med => med += 1,
                    RoomSize::Large => large += 1,
                }

                rooms.push(room);
                break 'place_room;
            }
        }

        println!("{}", seed);
        println!("{}x{}", dw, dh);
        println!("Attempts: {}", attempts);
        println!("Small rooms: {}", small);
        println!("Medium rooms: {}", med);
        println!("Large rooms: {}\n", large);

        for room in rooms {
            for x in room.x1()..room.x2() {
                for y in room.y1()..room.y2() {
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
    x: u32, // X coordinate of center
    y: u32, // Y coordinate of center
    w: u32,
    h: u32,
}

impl Room {
    pub fn new(x: u32, y: u32, w: u32, h: u32) -> Self {
        Room {x: x, y: y, w: w, h: h}
    }

    pub fn x1(&self) -> u32 {
        self.x - (self.w / 2)
    }

    pub fn x2(&self) -> u32 {
        self.x + (self.w / 2)
    }

    pub fn y1(&self) -> u32 {
        self.y - (self.h / 2)
    }

    pub fn y2(&self) -> u32 {
        self.y + (self.h / 2)
    }

    fn intersects(&self, other: &Room) -> bool {
        self.x1() <= other.x2() && self.x2() >= other.x1() &&
            self.y1() <= other.y2() && other.y2() >= other.y1()
    }
}
