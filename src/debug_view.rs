use crate::{
    map::{Map, MaterialId, Tile, TILE_SIZE},
    player::Player,
    raycast::{ViewRaySample, WallFaceOrientation},
};
use macroquad::prelude::*;

const GRID_LINE_THICKNESS: f32 = 1.0;
const HIT_MARKER_RADIUS: f32 = 2.5;
const PLAYER_RADIUS: f32 = 6.0;
const PLAYER_DIRECTION_LENGTH: f32 = 14.0;
const VIEWPORT_PADDING: f32 = 12.0;

pub fn draw_debug_view(map: &Map, player: &Player, rays: &[ViewRaySample], viewport: Rect) {
    draw_rectangle(
        viewport.x,
        viewport.y,
        viewport.w,
        viewport.h,
        Color::from_rgba(30, 26, 20, 255),
    );

    let layout = DebugViewLayout::new(map, viewport);
    draw_map(map, &layout);
    draw_rays(player, rays, &layout);
    draw_player(player, &layout);
}

struct DebugViewLayout {
    offset_x: f32,
    offset_y: f32,
    tile_size: f32,
}

impl DebugViewLayout {
    fn new(map: &Map, viewport: Rect) -> Self {
        let available_width = (viewport.w - VIEWPORT_PADDING * 2.0).max(1.0);
        let available_height = (viewport.h - VIEWPORT_PADDING * 2.0).max(1.0);
        let tile_size = (available_width / map.width() as f32)
            .min(available_height / map.height() as f32)
            .max(1.0);
        let map_width = map.width() as f32 * tile_size;
        let map_height = map.height() as f32 * tile_size;
        let offset_x = viewport.x + (viewport.w - map_width) * 0.5;
        let offset_y = viewport.y + (viewport.h - map_height) * 0.5;

        Self {
            offset_x,
            offset_y,
            tile_size,
        }
    }

    fn world_to_screen_x(&self, world_x: f32) -> f32 {
        self.offset_x + world_x / TILE_SIZE * self.tile_size
    }

    fn world_to_screen_y(&self, world_y: f32) -> f32 {
        self.offset_y + world_y / TILE_SIZE * self.tile_size
    }
}

fn draw_map(map: &Map, layout: &DebugViewLayout) {
    for y in 0..map.height() {
        for x in 0..map.width() {
            let tile = map
                .tile_at(x, y)
                .expect("drawing should only request in-bounds tiles");

            let screen_x = layout.offset_x + x as f32 * layout.tile_size;
            let screen_y = layout.offset_y + y as f32 * layout.tile_size;

            draw_rectangle(
                screen_x,
                screen_y,
                layout.tile_size,
                layout.tile_size,
                tile_color(tile),
            );
            draw_rectangle_lines(
                screen_x,
                screen_y,
                layout.tile_size,
                layout.tile_size,
                GRID_LINE_THICKNESS,
                DARKGRAY,
            );
        }
    }
}

fn draw_rays(player: &Player, rays: &[ViewRaySample], layout: &DebugViewLayout) {
    let origin_x = layout.world_to_screen_x(player.x());
    let origin_y = layout.world_to_screen_y(player.y());
    let ray_style = ray_style_for_count(rays.len());

    for ray in rays {
        draw_line(
            origin_x,
            origin_y,
            layout.world_to_screen_x(ray.end_x),
            layout.world_to_screen_y(ray.end_y),
            ray_style.thickness,
            ray_style.color,
        );

        if let Some(hit) = ray.hit {
            draw_circle(
                layout.world_to_screen_x(hit.hit_x),
                layout.world_to_screen_y(hit.hit_y),
                HIT_MARKER_RADIUS,
                hit_marker_color(hit.face_orientation),
            );
        }
    }
}

fn draw_player(player: &Player, layout: &DebugViewLayout) {
    let screen_x = layout.world_to_screen_x(player.x());
    let screen_y = layout.world_to_screen_y(player.y());
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

#[derive(Clone, Copy, Debug, PartialEq)]
struct RayStyle {
    thickness: f32,
    color: Color,
}

fn ray_style_for_count(ray_count: usize) -> RayStyle {
    if ray_count >= 80 {
        RayStyle {
            thickness: 1.0,
            color: Color::from_rgba(255, 245, 140, 110),
        }
    } else if ray_count >= 48 {
        RayStyle {
            thickness: 1.25,
            color: Color::from_rgba(255, 245, 140, 150),
        }
    } else {
        RayStyle {
            thickness: 1.5,
            color: Color::from_rgba(255, 245, 140, 190),
        }
    }
}

fn hit_marker_color(face_orientation: WallFaceOrientation) -> Color {
    match face_orientation {
        WallFaceOrientation::Vertical => Color::from_rgba(255, 250, 210, 255),
        WallFaceOrientation::Horizontal => Color::from_rgba(255, 150, 110, 255),
    }
}

#[cfg(test)]
mod tests {
    use super::{hit_marker_color, ray_style_for_count};
    use crate::raycast::WallFaceOrientation;

    #[test]
    fn dense_ray_fans_use_thinner_more_transparent_lines() {
        let sparse = ray_style_for_count(31);
        let dense = ray_style_for_count(91);

        assert!(dense.thickness < sparse.thickness);
        assert!(dense.color.a < sparse.color.a);
    }

    #[test]
    fn medium_ray_fans_use_intermediate_styling() {
        let sparse = ray_style_for_count(31);
        let medium = ray_style_for_count(64);
        let dense = ray_style_for_count(91);

        assert!(sparse.thickness > medium.thickness);
        assert!(medium.thickness > dense.thickness);
        assert!(sparse.color.a > medium.color.a);
        assert!(medium.color.a > dense.color.a);
    }

    #[test]
    fn face_hit_markers_use_different_colors() {
        let vertical = hit_marker_color(WallFaceOrientation::Vertical);
        let horizontal = hit_marker_color(WallFaceOrientation::Horizontal);

        assert_ne!(vertical, horizontal);
    }
}
