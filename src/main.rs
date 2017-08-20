extern crate ggez;
use ggez::{Context, event};
use ggez::graphics::{self, Color, FullscreenType};
use ggez::conf::Conf;

use std::env::args;

extern crate mount_arelle;
use mount_arelle::dungeon::Dungeon;
use mount_arelle::game::Game;

fn main() {
    let seed = args().nth(1).expect("No seed specified");
    let dungeon = Dungeon::new_from_seed(&seed);

    let mut game = Game::new(dungeon);

    let config = Conf {
        window_title: "Mt. Arelle".to_string(),
        window_icon: "".to_string(),
        window_width: 800,
        window_height: 600,
        vsync: true,
        resizable: true,
    };

    let mut ctx = Context::load_from_conf("mtrl", "sector-f", config).unwrap();
    graphics::set_background_color(&mut ctx, Color::new(0.0, 0.0, 0.0, 1.0));

    &ctx.sdl_context.mouse().set_relative_mouse_mode(true);

    event::run(&mut ctx, &mut game).unwrap();
}
