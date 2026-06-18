use crate::{
    map::{Map, MaterialId, Tile, TILE_SIZE},
    player::Player,
    raycast::DebugRay,
};
use macroquad::prelude::*;

const GRID_LINE_THICKNESS: f32 = 2.0;
const MAP_OFFSET_X: f32 = 80.0;
const MAP_OFFSET_Y: f32 = 48.0;
const PLAYER_RADIUS: f32 = 8.0;
const PLAYER_DIRECTION_LENGTH: f32 = 18.0;
const RAY_THICKNESS: f32 = 1.5;

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

pub fn draw_rays(rays: &[DebugRay]) {
    for ray in rays {
        draw_line(
            MAP_OFFSET_X + ray.origin_x,
            MAP_OFFSET_Y + ray.origin_y,
            MAP_OFFSET_X + ray.end_x,
            MAP_OFFSET_Y + ray.end_y,
            RAY_THICKNESS,
            Color::from_rgba(255, 245, 140, 190),
        );
    }
}

pub fn draw_player(player: &Player) {
    let screen_x = MAP_OFFSET_X + player.x();
    let screen_y = MAP_OFFSET_Y + player.y();
    let direction_x = screen_x + player.facing_angle().cos() * PLAYER_DIRECTION_LENGTH;
    let direction_y = screen_y + player.facing_angle().sin() * PLAYER_DIRECTION_LENGTH;

    draw_circle(
        screen_x,
        screen_y,
        PLAYER_RADIUS,
        Color::from_rgba(230, 80, 60, 255),
    );
    draw_line(
        screen_x,
        screen_y,
        direction_x,
        direction_y,
        3.0,
        Color::from_rgba(255, 245, 200, 255),
    );
}

fn tile_color(tile: Tile) -> Color {
    match tile {
        Tile::Wall(wall) => color_for_material(wall.material_id),
        Tile::Empty => Color::from_rgba(200, 185, 145, 255),
    }
}

fn color_for_material(material_id: MaterialId) -> Color {
    match material_id {
        1 => Color::from_rgba(50, 115, 220, 255),
        2 => Color::from_rgba(210, 120, 40, 255),
        _ => MAGENTA,
    }
}
