use crate::map::{Map, Tile};
use macroquad::prelude::*;

const TILE_SIZE: f32 = 48.0;
const GRID_LINE_THICKNESS: f32 = 2.0;
const MAP_OFFSET_X: f32 = 80.0;
const MAP_OFFSET_Y: f32 = 48.0;

pub fn draw_map(map: &Map) {
    for y in 0..map.height() {
        for x in 0..map.width() {
            let tile = map
                .tile_at(x, y)
                .expect("drawing should only request in-bounds tiles");

            let screen_x = MAP_OFFSET_X + x as f32 * TILE_SIZE;
            let screen_y = MAP_OFFSET_Y + y as f32 * TILE_SIZE;

            draw_rectangle(screen_x, screen_y, TILE_SIZE, TILE_SIZE, tile_color(tile));
            draw_rectangle_lines(
                screen_x,
                screen_y,
                TILE_SIZE,
                TILE_SIZE,
                GRID_LINE_THICKNESS,
                DARKGRAY,
            );
        }
    }
}

fn tile_color(tile: Tile) -> Color {
    match tile {
        Tile::Wall => Color::from_rgba(50, 115, 220, 255),
        Tile::Empty => Color::from_rgba(200, 185, 145, 255),
    }
}
