extern crate rand;
extern crate serde;
#[macro_use] extern crate serde_derive;
extern crate serde_json;
extern crate image;

use std::env::args;

pub mod dungeon;
use dungeon::Dungeon;

fn main() {
    let seed = args().nth(1).expect("No seed specified");
    let dungeon = Dungeon::new_from_seed(&seed);

    let _ = dungeon.render_image(format!("{}.png", seed));
}
