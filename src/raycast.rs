use crate::map::{Map, Tile, Wall, TILE_SIZE};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RayHit {
    pub hit_x: f32,
    pub hit_y: f32,
    pub distance: f32,
    pub tile_x: usize,
    pub tile_y: usize,
    pub wall: Wall,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ViewRaySample {
    pub index: usize,
    pub angle: f32,
    pub hit: Option<RayHit>,
    pub end_x: f32,
    pub end_y: f32,
}

pub fn cast_ray(map: &Map, origin_x: f32, origin_y: f32, angle: f32) -> Option<RayHit> {
    let direction_x = angle.cos();
    let direction_y = angle.sin();

    let mut tile_x = (origin_x / TILE_SIZE).floor() as i32;
    let mut tile_y = (origin_y / TILE_SIZE).floor() as i32;

    if !is_in_bounds(map, tile_x, tile_y) {
        return None;
    }

    let step_x = if direction_x < 0.0 { -1 } else { 1 };
    let step_y = if direction_y < 0.0 { -1 } else { 1 };

    let delta_distance_x = axis_delta_distance(direction_x);
    let delta_distance_y = axis_delta_distance(direction_y);

    let mut side_distance_x =
        first_vertical_boundary_distance(origin_x, tile_x, direction_x, delta_distance_x);
    let mut side_distance_y =
        first_horizontal_boundary_distance(origin_y, tile_y, direction_y, delta_distance_y);

    loop {
        let hit_vertical_side = side_distance_x <= side_distance_y;
        let hit_distance = if hit_vertical_side {
            tile_x += step_x;
            let distance = side_distance_x;
            side_distance_x += delta_distance_x;
            distance
        } else {
            tile_y += step_y;
            let distance = side_distance_y;
            side_distance_y += delta_distance_y;
            distance
        };

        if !is_in_bounds(map, tile_x, tile_y) {
            return None;
        }

        let hit_tile_x = tile_x as usize;
        let hit_tile_y = tile_y as usize;

        match map.tile_at(hit_tile_x, hit_tile_y) {
            Some(Tile::Wall(wall)) => {
                return Some(RayHit {
                    hit_x: origin_x + direction_x * hit_distance,
                    hit_y: origin_y + direction_y * hit_distance,
                    distance: hit_distance,
                    tile_x: hit_tile_x,
                    tile_y: hit_tile_y,
                    wall,
                });
            }
            Some(Tile::Empty) => {}
            None => return None,
        }
    }
}

pub fn cast_view_rays(
    map: &Map,
    origin_x: f32,
    origin_y: f32,
    facing_angle: f32,
    ray_count: usize,
    fov_radians: f32,
) -> Vec<ViewRaySample> {
    if ray_count == 0 {
        return Vec::new();
    }

    let start_angle = facing_angle - fov_radians * 0.5;
    let angle_step = if ray_count == 1 {
        0.0
    } else {
        fov_radians / (ray_count - 1) as f32
    };

    (0..ray_count)
        .map(|index| {
            let angle = start_angle + angle_step * index as f32;
            let direction_x = angle.cos();
            let direction_y = angle.sin();
            let hit = cast_ray(map, origin_x, origin_y, angle);
            let (end_x, end_y) = match hit {
                Some(ray_hit) => (ray_hit.hit_x, ray_hit.hit_y),
                None => map_exit_point(map, origin_x, origin_y, direction_x, direction_y)
                    .unwrap_or((origin_x, origin_y)),
            };

            ViewRaySample {
                index,
                angle,
                hit,
                end_x,
                end_y,
            }
        })
        .collect()
}

fn axis_delta_distance(direction: f32) -> f32 {
    if direction.abs() <= f32::EPSILON {
        f32::INFINITY
    } else {
        TILE_SIZE / direction.abs()
    }
}

fn first_vertical_boundary_distance(
    origin_x: f32,
    tile_x: i32,
    direction_x: f32,
    delta_distance_x: f32,
) -> f32 {
    if delta_distance_x.is_infinite() {
        return f32::INFINITY;
    }

    let next_boundary_x = if direction_x < 0.0 {
        tile_x as f32 * TILE_SIZE
    } else {
        (tile_x + 1) as f32 * TILE_SIZE
    };

    (next_boundary_x - origin_x).abs() / direction_x.abs().max(f32::EPSILON)
}

fn first_horizontal_boundary_distance(
    origin_y: f32,
    tile_y: i32,
    direction_y: f32,
    delta_distance_y: f32,
) -> f32 {
    if delta_distance_y.is_infinite() {
        return f32::INFINITY;
    }

    let next_boundary_y = if direction_y < 0.0 {
        tile_y as f32 * TILE_SIZE
    } else {
        (tile_y + 1) as f32 * TILE_SIZE
    };

    (next_boundary_y - origin_y).abs() / direction_y.abs().max(f32::EPSILON)
}

fn map_exit_point(
    map: &Map,
    origin_x: f32,
    origin_y: f32,
    direction_x: f32,
    direction_y: f32,
) -> Option<(f32, f32)> {
    let max_x = map.width() as f32 * TILE_SIZE;
    let max_y = map.height() as f32 * TILE_SIZE;
    let mut exit_distance = f32::INFINITY;

    if direction_x > 0.0 {
        let distance = (max_x - origin_x) / direction_x;
        if distance >= 0.0 {
            let y = origin_y + direction_y * distance;
            if y >= 0.0 && y <= max_y {
                exit_distance = exit_distance.min(distance);
            }
        }
    } else if direction_x < 0.0 {
        let distance = -origin_x / direction_x;
        if distance >= 0.0 {
            let y = origin_y + direction_y * distance;
            if y >= 0.0 && y <= max_y {
                exit_distance = exit_distance.min(distance);
            }
        }
    }

    if direction_y > 0.0 {
        let distance = (max_y - origin_y) / direction_y;
        if distance >= 0.0 {
            let x = origin_x + direction_x * distance;
            if x >= 0.0 && x <= max_x {
                exit_distance = exit_distance.min(distance);
            }
        }
    } else if direction_y < 0.0 {
        let distance = -origin_y / direction_y;
        if distance >= 0.0 {
            let x = origin_x + direction_x * distance;
            if x >= 0.0 && x <= max_x {
                exit_distance = exit_distance.min(distance);
            }
        }
    }

    if exit_distance.is_finite() {
        Some((
            origin_x + direction_x * exit_distance,
            origin_y + direction_y * exit_distance,
        ))
    } else {
        None
    }
}

fn is_in_bounds(map: &Map, tile_x: i32, tile_y: i32) -> bool {
    tile_x >= 0
        && tile_y >= 0
        && (tile_x as usize) < map.width()
        && (tile_y as usize) < map.height()
}

#[cfg(test)]
mod tests {
    use super::{cast_ray, cast_view_rays};
    use crate::map::Map;
    use std::f32::consts::{FRAC_PI_2, FRAC_PI_3};

    const FLOAT_TOLERANCE: f32 = 1.0e-3;

    #[test]
    fn cast_ray_straight_to_nearby_wall_returns_expected_hit() {
        let map = Map::from_rows(&["....", ".1..", "...."]).expect("map should parse");
        let hit = cast_ray(&map, 24.0, 72.0, 0.0).expect("ray should hit wall");

        assert_eq!((hit.tile_x, hit.tile_y), (1, 1));
        assert!((hit.hit_x - 48.0).abs() < FLOAT_TOLERANCE);
        assert!((hit.hit_y - 72.0).abs() < FLOAT_TOLERANCE);
        assert!(hit.distance > 0.0);
    }

    #[test]
    fn cast_ray_reports_material_id_one() {
        let map = Map::from_rows(&["....", ".1..", "...."]).expect("map should parse");
        let hit = cast_ray(&map, 24.0, 72.0, 0.0).expect("ray should hit wall");

        assert_eq!(hit.wall.material_id, 1);
    }

    #[test]
    fn cast_ray_reports_material_id_two() {
        let map = Map::from_rows(&["....", ".2..", "...."]).expect("map should parse");
        let hit = cast_ray(&map, 24.0, 72.0, 0.0).expect("ray should hit wall");

        assert_eq!(hit.wall.material_id, 2);
    }

    #[test]
    fn cast_ray_returns_none_when_it_leaves_the_map() {
        let map = Map::from_rows(&["....", "....", "...."]).expect("map should parse");

        assert_eq!(cast_ray(&map, 24.0, 72.0, 0.0), None);
    }

    #[test]
    fn cast_view_rays_returns_requested_count_in_left_to_right_angle_order() {
        let map = Map::from_rows(&["....", "....", "....", "...."]).expect("map should parse");
        let rays = cast_view_rays(&map, 72.0, 72.0, FRAC_PI_2, 5, FRAC_PI_3);

        assert_eq!(rays.len(), 5);
        assert!(rays[0].angle < rays[1].angle);
        assert!((rays[2].angle - FRAC_PI_2).abs() < FLOAT_TOLERANCE);
        assert!(rays[3].angle < rays[4].angle);
    }

    #[test]
    fn cast_ray_handles_vertical_and_horizontal_angles() {
        let map = Map::from_rows(&["....", ".1..", "....", ".2.."]).expect("map should parse");

        let horizontal_hit = cast_ray(&map, 24.0, 72.0, 0.0).expect("horizontal ray should hit");
        let vertical_hit = cast_ray(&map, 72.0, 24.0, FRAC_PI_2).expect("vertical ray should hit");

        assert_eq!((horizontal_hit.tile_x, horizontal_hit.tile_y), (1, 1));
        assert_eq!(vertical_hit.wall.material_id, 1);
    }
}
