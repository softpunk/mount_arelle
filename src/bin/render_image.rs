use std::env::args;

extern crate mount_arelle;
use mount_arelle::dungeon::Dungeon;

fn main() {
    let seed = args().nth(1).expect("No seed specified");
    let dungeon = Dungeon::new_from_seed(&seed);

    let _ = dungeon.render_grid(format!("{}.png", seed));
}
