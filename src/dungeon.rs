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
    pub grid: Grid,
    player_spawn: (f64, f64),
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
            DungeonSize::Small => Range::new(50, 81),
            DungeonSize::Med => Range::new(80, 121),
            DungeonSize::Large => Range::new(120, 151),
        };

        let dw = dungeon_bounds.ind_sample(&mut rng);
        let dh = dungeon_bounds.ind_sample(&mut rng);

        let mut grid = Grid::new(dw, dh, Tile::Wall);

        let attempts = rng.gen_range(20, 041);

        let max_rooms = match dungeon_size {
            DungeonSize::Small => {
                rng.gen_range(7, 11)
            },
            DungeonSize::Med => {
                rng.gen_range(10, 21)
            },
            DungeonSize::Large => {
                rng.gen_range(20, 40)
            },
        };

        let mut rooms: Vec<Room> = Vec::new();

        'create_rooms: for _ in 0..attempts {
            let mut room_sizes;

            match dungeon_size {
                DungeonSize::Small => {
                    room_sizes = [
                        Weighted { weight: 100, item: RoomSize::Small },
                        Weighted { weight: 200, item: RoomSize::Med },
                        Weighted { weight:   0, item: RoomSize::Large },
                    ];
                },
                DungeonSize::Med => {
                    room_sizes = [
                        Weighted { weight: 100, item: RoomSize::Small },
                        Weighted { weight: 250, item: RoomSize::Med },
                        Weighted { weight:  50, item: RoomSize::Large },
                    ];
                },
                DungeonSize::Large => {
                    room_sizes = [
                        Weighted { weight: 100, item: RoomSize::Small },
                        Weighted { weight: 250, item: RoomSize::Med },
                        Weighted { weight:  100, item: RoomSize::Large },
                    ];
                },
            };

            let room_size = WeightedChoice::new(&mut room_sizes).ind_sample(&mut rng);
            let room_bounds = match room_size {
                RoomSize::Small => Range::new(5, 11),
                RoomSize::Med => Range::new(10, 15),
                RoomSize::Large => Range::new(15, 21),
            };

            let rw = room_bounds.ind_sample(&mut rng);
            let rh = room_bounds.ind_sample(&mut rng);

            let rx = rng.gen_range(0, dw - rw);
            let ry = rng.gen_range(0, dh - rh);

            let room = Room::new(
                rx,
                ry,
                rw,
                rh,
            );

            for r in &rooms {
                if r.intersects(&room) {
                    continue 'create_rooms;
                }
            }

            match room_size {
                RoomSize::Small => small += 1,
                RoomSize::Med => med += 1,
                RoomSize::Large => large += 1,
            }

            rooms.push(room);
            if rooms.len() == max_rooms {
                break 'create_rooms;
            }
        }

        // println!("{}", seed);
        // println!("{}x{}", dw, dh);
        // println!("Max rooms: {}", max_rooms);
        // println!("Actual rooms: {}", rooms.len());
        // println!("Small rooms: {}", small);
        // println!("Medium rooms: {}", med);
        // println!("Large rooms: {}\n", large);

        for room in &rooms {
            for x in room.x1()..room.x2() {
                for y in room.y1()..room.y2() {
                    grid[(x,y)] = Tile::Floor;
                }
            }
        }

        for (prev, room) in rooms.iter().skip(1).enumerate() {
            let prev_room = &rooms[prev];

            // Random coin flip?
            match rng.gen::<bool>() {
                true => {
                    carve_h(
                        &mut grid,
                        prev_room.center_x() as u32,
                        room.center_x() as u32,
                        prev_room.center_y() as u32,
                    );
                    carve_v(
                        &mut grid,
                        prev_room.center_y() as u32,
                        room.center_y() as u32,
                        room.center_x() as u32,
                    );
                },
                false => {
                    carve_v(
                        &mut grid,
                        prev_room.center_y() as u32,
                        room.center_y() as u32,
                        prev_room.center_x() as u32,
                    );
                    carve_h(
                        &mut grid,
                        prev_room.center_x() as u32,
                        room.center_x() as u32,
                        room.center_y() as u32,
                    );
                },
            }
        }

        let (px, py) = rng.choose(&rooms).unwrap().center();

        Dungeon {
            grid: grid,
            player_spawn: (px, py),
        }
    }

    pub fn render_grid<P: AsRef<Path>>(&self, path: P) -> io::Result<()> {
        self.grid.render_image(path)
    }

    pub fn player_spawn(&self) -> (f64, f64) {
        self.player_spawn
    }
}

fn carve_h(grid: &mut Grid, x1: u32, x2: u32, y: u32) {
    let lowest = if x1 < x2 { x1 } else { x2 };
    let highest = if x1 > x2 { x1 } else { x2 };

    for x in lowest..highest + 1 {
        grid[(x, y)] = Tile::Floor;
    }
}

fn carve_v(grid: &mut Grid, y1: u32, y2: u32, x: u32) {
    let lowest = if y1 < y2 { y1 } else { y2 };
    let highest = if y1 > y2 { y1 } else { y2 };

    for y in lowest..highest + 1 {
        grid[(x, y)] = Tile::Floor;
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
    y: u32, // Y coordinate of top-right
    w: u32,
    h: u32,
}

impl Room {
    fn new(x: u32, y: u32, w: u32, h: u32) -> Self {
        Room {x: x, y: y, w: w, h: h}
    }

    fn x1(&self) -> u32 {
        self.x
    }

    fn x2(&self) -> u32 {
        self.x + self.w
    }

    fn y1(&self) -> u32 {
        self.y
    }

    fn y2(&self) -> u32 {
        self.y + self.h
    }

    fn center_x(&self) -> f64 {
        self.x as f64 + (self.w as f64 / 2.0)
    }

    fn center_y(&self) -> f64 {
        self.y as f64 + (self.h as f64 / 2.0)
    }

    fn center(&self) -> (f64, f64) {
        (self.center_x(), self.center_y())
    }

    fn intersects(&self, other: &Room) -> bool {
        self.x1() <= other.x2() && self.x2() >= other.x1() &&
            self.y1() <= other.y2() && self.y2() >= other.y1()
    }
}
