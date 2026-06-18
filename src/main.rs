mod debug_view;
mod map;

use debug_view::draw_map;
use macroquad::prelude::*;
use map::Map;

const WINDOW_WIDTH: i32 = 640;
const WINDOW_HEIGHT: i32 = 480;
const LEVEL_ROWS: [&str; 8] = [
    "##########",
    "#........#",
    "#..##....#",
    "#........#",
    "#....##..#",
    "#........#",
    "#........#",
    "##########",
];

fn window_conf() -> Conf {
    Conf {
        window_title: "Retro FPS Debug Map".to_owned(),
        window_width: WINDOW_WIDTH,
        window_height: WINDOW_HEIGHT,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let map = Map::from_rows(&LEVEL_ROWS).expect("initial level rows should form a valid map");

    loop {
        clear_background(BLACK);
        draw_map(&map);

        next_frame().await;
    }
}
