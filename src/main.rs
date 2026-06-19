mod debug_view;
mod map;
mod player;
mod raycast;
mod view_3d;

use debug_view::draw_debug_view;
use macroquad::prelude::*;
use map::{Map, TILE_SIZE};
use player::{Player, PlayerIntent};
use raycast::cast_view_rays;
use std::f32::consts::FRAC_PI_3;
use view_3d::{draw_view_3d, project_view_spans};

const WINDOW_WIDTH: i32 = 640;
const WINDOW_HEIGHT: i32 = 480;
const SIMULATION_STEP_SECONDS: f32 = 1.0 / 60.0;
const MAX_FRAME_SECONDS: f32 = 0.25;
const DEBUG_RAY_COUNT: usize = 91;
const DEBUG_FOV_RADIANS: f32 = FRAC_PI_3;
const MAIN_VIEW_WIDTH_RATIO: f32 = 0.7;
const LEVEL_ROWS: [&str; 8] = [
    "1111111111",
    "1........1",
    "1..22....1",
    "1........1",
    "1....11..1",
    "1....22..1",
    "1........1",
    "1111111111",
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
    let mut player = Player::new(TILE_SIZE * 1.5, TILE_SIZE * 1.5, 0.0);
    let mut accumulated_time = 0.0;

    loop {
        let frame_seconds = get_frame_time().min(MAX_FRAME_SECONDS);
        let input = read_player_input();
        accumulated_time += frame_seconds;

        while accumulated_time >= SIMULATION_STEP_SECONDS {
            player.step(input, &map, SIMULATION_STEP_SECONDS);
            accumulated_time -= SIMULATION_STEP_SECONDS;
        }

        clear_background(BLACK);
        let view_rays = cast_view_rays(
            &map,
            player.x(),
            player.y(),
            player.facing_angle(),
            DEBUG_RAY_COUNT,
            DEBUG_FOV_RADIANS,
        );
        let main_view_rect = Rect::new(
            0.0,
            0.0,
            WINDOW_WIDTH as f32 * MAIN_VIEW_WIDTH_RATIO,
            WINDOW_HEIGHT as f32,
        );
        let debug_view_rect = Rect::new(
            main_view_rect.w,
            0.0,
            WINDOW_WIDTH as f32 - main_view_rect.w,
            WINDOW_HEIGHT as f32,
        );
        let spans = project_view_spans(
            &view_rays,
            player.facing_angle(),
            main_view_rect.w,
            main_view_rect.h,
        );

        draw_view_3d(&spans, main_view_rect);
        draw_debug_view(&map, &player, &view_rays, debug_view_rect);

        next_frame().await;
    }
}

fn read_player_input() -> PlayerIntent {
    let mut intent = PlayerIntent::default();

    if is_key_down(KeyCode::W) {
        intent.forward = 1.0;
    }

    if is_key_down(KeyCode::S) {
        intent.forward = -1.0;
    }

    if is_key_down(KeyCode::A) {
        intent.turn = -1.0;
    }

    if is_key_down(KeyCode::D) {
        intent.turn = 1.0;
    }

    intent
}
