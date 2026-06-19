use crate::map::MaterialId;
use crate::raycast::{ViewRaySample, WallFaceOrientation};
use macroquad::prelude::*;

const WALL_HEIGHT_WORLD_UNITS: f32 = 48.0;
const NEAR_PLANE_DISTANCE: f32 = 0.0001;
const CEILING_COLOR: Color = Color {
    r: 44.0 / 255.0,
    g: 52.0 / 255.0,
    b: 68.0 / 255.0,
    a: 1.0,
};
const FLOOR_COLOR: Color = Color {
    r: 38.0 / 255.0,
    g: 28.0 / 255.0,
    b: 24.0 / 255.0,
    a: 1.0,
};
const MIN_SHADE_INTENSITY: f32 = 0.25;
const SHADE_FALLOFF_DISTANCE: f32 = 240.0;
const VERTICAL_FACE_SHADE: f32 = 1.0;
const HORIZONTAL_FACE_SHADE: f32 = 0.82;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct VerticalSpan {
    pub screen_x: f32,
    pub width: f32,
    pub top_y: f32,
    pub bottom_y: f32,
    pub material_id: MaterialId,
    pub face_orientation: WallFaceOrientation,
}

pub fn project_view_spans(
    samples: &[ViewRaySample],
    facing_angle: f32,
    viewport_width: f32,
    viewport_height: f32,
) -> Vec<VerticalSpan> {
    if samples.is_empty() {
        return Vec::new();
    }

    let column_width = viewport_width / samples.len() as f32;
    let projection_scale = viewport_height * 0.5;
    let horizon_y = viewport_height * 0.5;

    samples
        .iter()
        .filter_map(|sample| {
            let hit = sample.hit?;
            let corrected_distance =
                fisheye_corrected_distance(hit.distance, sample.angle, facing_angle);
            let span_height = projected_span_height(
                corrected_distance,
                projection_scale,
                WALL_HEIGHT_WORLD_UNITS,
            );
            let top_y = (horizon_y - span_height * 0.5).max(0.0);
            let bottom_y = (horizon_y + span_height * 0.5).min(viewport_height);

            Some(VerticalSpan {
                screen_x: sample.index as f32 * column_width,
                width: column_width.ceil().max(1.0),
                top_y,
                bottom_y,
                material_id: hit.wall.material_id,
                face_orientation: hit.face_orientation,
            })
        })
        .collect()
}

pub fn draw_view_3d(spans: &[VerticalSpan], viewport_rect: Rect) {
    let horizon_y = horizon_y(viewport_rect.h);

    draw_rectangle(
        viewport_rect.x,
        viewport_rect.y,
        viewport_rect.w,
        horizon_y,
        CEILING_COLOR,
    );
    draw_rectangle(
        viewport_rect.x,
        viewport_rect.y + horizon_y,
        viewport_rect.w,
        viewport_rect.h - horizon_y,
        FLOOR_COLOR,
    );

    if spans.is_empty() {
        return;
    }

    for span in spans {
        draw_rectangle(
            viewport_rect.x + span.screen_x,
            viewport_rect.y + span.top_y,
            span.width,
            (span.bottom_y - span.top_y).max(1.0),
            wall_color_for_distance(
                span.material_id,
                distance_from_span_height(
                    span.bottom_y - span.top_y,
                    viewport_rect.h,
                    WALL_HEIGHT_WORLD_UNITS,
                ),
                span.face_orientation,
            ),
        );
    }
}

fn fisheye_corrected_distance(distance: f32, ray_angle: f32, facing_angle: f32) -> f32 {
    let angle_offset = ray_angle - facing_angle;
    (distance * angle_offset.cos()).max(NEAR_PLANE_DISTANCE)
}

fn projected_span_height(
    corrected_distance: f32,
    projection_scale: f32,
    wall_height_world_units: f32,
) -> f32 {
    (wall_height_world_units / corrected_distance) * projection_scale
}

fn horizon_y(viewport_height: f32) -> f32 {
    viewport_height * 0.5
}

fn distance_from_span_height(
    span_height: f32,
    viewport_height: f32,
    wall_height_world_units: f32,
) -> f32 {
    let projection_scale = viewport_height * 0.5;
    let clamped_span_height = span_height.max(1.0);
    (wall_height_world_units * projection_scale / clamped_span_height).max(NEAR_PLANE_DISTANCE)
}

fn shade_for_distance(distance: f32) -> f32 {
    (1.0 - distance / SHADE_FALLOFF_DISTANCE).clamp(MIN_SHADE_INTENSITY, 1.0)
}

fn wall_color_for_distance(
    material_id: MaterialId,
    distance: f32,
    face_orientation: WallFaceOrientation,
) -> Color {
    let shade = shade_for_distance(distance) * face_shade(face_orientation);
    let base_color = base_color_for_material(material_id);

    Color {
        r: (base_color.r * shade).clamp(0.0, 1.0),
        g: (base_color.g * shade).clamp(0.0, 1.0),
        b: (base_color.b * shade).clamp(0.0, 1.0),
        a: base_color.a,
    }
}

fn face_shade(face_orientation: WallFaceOrientation) -> f32 {
    match face_orientation {
        WallFaceOrientation::Vertical => VERTICAL_FACE_SHADE,
        WallFaceOrientation::Horizontal => HORIZONTAL_FACE_SHADE,
    }
}

fn base_color_for_material(material_id: MaterialId) -> Color {
    match material_id {
        1 => Color::from_rgba(75, 145, 255, 255),
        2 => Color::from_rgba(240, 150, 55, 255),
        _ => MAGENTA,
    }
}

#[cfg(test)]
mod tests {
    use super::{
        base_color_for_material, horizon_y, project_view_spans, shade_for_distance,
        wall_color_for_distance,
    };
    use crate::{
        map::Wall,
        raycast::{RayHit, ViewRaySample, WallFaceOrientation},
    };
    use macroquad::prelude::Color;
    use std::f32::consts::FRAC_PI_4;

    #[test]
    fn nearer_hit_projects_taller_span() {
        let samples = [
            sample_with_hit(0, 0.0, 40.0, 1, WallFaceOrientation::Vertical),
            sample_with_hit(1, 0.0, 80.0, 1, WallFaceOrientation::Vertical),
        ];

        let spans = project_view_spans(&samples, 0.0, 200.0, 120.0);

        let near_height = spans[0].bottom_y - spans[0].top_y;
        let far_height = spans[1].bottom_y - spans[1].top_y;

        assert!(near_height > far_height);
    }

    #[test]
    fn symmetric_rays_keep_matching_heights_after_fisheye_correction() {
        let samples = [
            sample_with_hit(0, -FRAC_PI_4 * 0.5, 100.0, 1, WallFaceOrientation::Vertical),
            sample_with_hit(1, FRAC_PI_4 * 0.5, 100.0, 1, WallFaceOrientation::Vertical),
        ];

        let spans = project_view_spans(&samples, 0.0, 200.0, 120.0);
        let left_height = spans[0].bottom_y - spans[0].top_y;
        let right_height = spans[1].bottom_y - spans[1].top_y;

        assert!((left_height - right_height).abs() < f32::EPSILON);
    }

    #[test]
    fn no_hit_sample_produces_no_span() {
        let samples = [ViewRaySample {
            index: 0,
            angle: 0.0,
            hit: None,
            end_x: 0.0,
            end_y: 0.0,
        }];

        let spans = project_view_spans(&samples, 0.0, 200.0, 120.0);

        assert!(spans.is_empty());
    }

    #[test]
    fn projected_spans_preserve_left_to_right_order() {
        let samples = [
            sample_with_hit(0, -0.2, 50.0, 1, WallFaceOrientation::Vertical),
            sample_with_hit(1, 0.0, 60.0, 1, WallFaceOrientation::Vertical),
            sample_with_hit(2, 0.2, 70.0, 1, WallFaceOrientation::Vertical),
        ];

        let spans = project_view_spans(&samples, 0.0, 300.0, 120.0);

        assert!(spans[0].screen_x < spans[1].screen_x);
        assert!(spans[1].screen_x < spans[2].screen_x);
    }

    #[test]
    fn more_samples_produce_narrower_columns() {
        let sparse_samples = [
            sample_with_hit(0, -0.2, 60.0, 1, WallFaceOrientation::Vertical),
            sample_with_hit(1, 0.0, 60.0, 1, WallFaceOrientation::Vertical),
            sample_with_hit(2, 0.2, 60.0, 1, WallFaceOrientation::Vertical),
        ];
        let dense_samples = [
            sample_with_hit(0, -0.2, 60.0, 1, WallFaceOrientation::Vertical),
            sample_with_hit(1, -0.1, 60.0, 1, WallFaceOrientation::Vertical),
            sample_with_hit(2, 0.0, 60.0, 1, WallFaceOrientation::Vertical),
            sample_with_hit(3, 0.1, 60.0, 1, WallFaceOrientation::Vertical),
            sample_with_hit(4, 0.2, 60.0, 1, WallFaceOrientation::Vertical),
        ];

        let sparse_spans = project_view_spans(&sparse_samples, 0.0, 300.0, 120.0);
        let dense_spans = project_view_spans(&dense_samples, 0.0, 300.0, 120.0);

        assert!(dense_spans[0].width < sparse_spans[0].width);
    }

    #[test]
    fn material_one_maps_to_expected_color() {
        assert_eq!(
            base_color_for_material(1),
            Color::from_rgba(75, 145, 255, 255)
        );
    }

    #[test]
    fn material_two_maps_to_expected_color() {
        assert_eq!(
            base_color_for_material(2),
            Color::from_rgba(240, 150, 55, 255)
        );
    }

    #[test]
    fn nearer_distance_produces_brighter_wall_color() {
        let near_color = wall_color_for_distance(1, 40.0, WallFaceOrientation::Vertical);
        let far_color = wall_color_for_distance(1, 160.0, WallFaceOrientation::Vertical);

        assert!(near_color.r > far_color.r);
        assert!(near_color.g > far_color.g);
        assert!(near_color.b > far_color.b);
    }

    #[test]
    fn distance_shading_stays_clamped_for_extreme_values() {
        let near_shade = shade_for_distance(0.0);
        let far_shade = shade_for_distance(10_000.0);

        assert_eq!(near_shade, 1.0);
        assert_eq!(far_shade, 0.25);
    }

    #[test]
    fn different_materials_stay_distinct_after_shading() {
        let material_one = wall_color_for_distance(1, 120.0, WallFaceOrientation::Vertical);
        let material_two = wall_color_for_distance(2, 120.0, WallFaceOrientation::Vertical);

        assert_ne!(material_one, material_two);
    }

    #[test]
    fn horizon_splits_viewport_in_half() {
        assert_eq!(horizon_y(120.0), 60.0);
    }

    #[test]
    fn face_orientation_changes_final_wall_color_at_same_distance() {
        let vertical = wall_color_for_distance(1, 120.0, WallFaceOrientation::Vertical);
        let horizontal = wall_color_for_distance(1, 120.0, WallFaceOrientation::Horizontal);

        assert_ne!(vertical, horizontal);
    }

    #[test]
    fn horizontal_face_stays_darker_after_distance_shading() {
        let vertical = wall_color_for_distance(1, 180.0, WallFaceOrientation::Vertical);
        let horizontal = wall_color_for_distance(1, 180.0, WallFaceOrientation::Horizontal);

        assert!(horizontal.r < vertical.r);
        assert!(horizontal.g < vertical.g);
        assert!(horizontal.b < vertical.b);
    }

    fn sample_with_hit(
        index: usize,
        angle: f32,
        distance: f32,
        material_id: u8,
        face_orientation: WallFaceOrientation,
    ) -> ViewRaySample {
        ViewRaySample {
            index,
            angle,
            hit: Some(RayHit {
                hit_x: 0.0,
                hit_y: 0.0,
                distance,
                tile_x: 0,
                tile_y: 0,
                wall: Wall { material_id },
                face_orientation,
            }),
            end_x: 0.0,
            end_y: 0.0,
        }
    }
}
