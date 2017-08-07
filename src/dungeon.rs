use rand::{Rng, SeedableRng};
use rand::isaac::IsaacRng;
use rand::distributions::{Weighted, WeightedChoice, IndependentSample};

#[derive(Serialize, Deserialize)]
pub struct Dungeon {
    tiles: Vec<Vec<Tile>>,
}

impl Dungeon {
    pub fn new_from_seed(seed: String) -> Self {
        let seed = seed.into_bytes().iter().map(|n| *n as u32).collect::<Vec<u32>>();
        let mut rng = IsaacRng::from_seed(&seed);

        let sizes = &mut [
            Weighted { weight: 100, item: DungeonSize::Small },
            Weighted { weight: 300, item: DungeonSize::Med },
            Weighted { weight: 50, item: DungeonSize::Large },
        ];
        let choice = WeightedChoice::new(sizes);
        let size = choice.ind_sample(&mut rng);

        let h: usize;
        let w: usize;

        match size {
            DungeonSize::Small => {
                h = rng.gen_range(100, 201);
                w = rng.gen_range(100, 201);
            },
            DungeonSize::Med => {
                h = rng.gen_range(200, 301);
                w = rng.gen_range(200, 301);
            },
            DungeonSize::Large => {
                h = rng.gen_range(300, 401);
                w = rng.gen_range(300, 401);
            },
        }
        let mut tiles = vec![vec![Tile::Wall; h]; w];

        let attempts = rng.gen_range(70, 201);
        let mut rooms: Vec<Room> = Vec::new();

        'insert: for _ in 0..attempts {
            let width = rng.gen_range(30, 51);
            let height = rng.gen_range(30, 51);

            let room = Room::new(
                rng.gen_range(0, w - width + 1),
                rng.gen_range(0, h - height + 1),
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
            for (x, y) in (room.x1()..(room.y2() + 1)).zip(room.y2()..(room.x1() + 1)) {
                tiles[x][y] = Tile::Floor;
            }
        }

        Dungeon {
            tiles: tiles,
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
enum DungeonSize {
    Small,
    Med,
    Large,
}

#[derive(Serialize, Deserialize, Clone)]
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
