use crate::map::MaterialId;
use crate::raycast::ViewRaySample;
use macroquad::prelude::*;

const WALL_HEIGHT_WORLD_UNITS: f32 = 48.0;
const NEAR_PLANE_DISTANCE: f32 = 0.0001;
const VOID_COLOR: Color = Color {
    r: 22.0 / 255.0,
    g: 18.0 / 255.0,
    b: 16.0 / 255.0,
    a: 1.0,
};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct VerticalSpan {
    pub screen_x: f32,
    pub width: f32,
    pub top_y: f32,
    pub bottom_y: f32,
    pub material_id: MaterialId,
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
            })
        })
        .collect()
}

pub fn draw_view_3d(spans: &[VerticalSpan], viewport_rect: Rect) {
    draw_rectangle(
        viewport_rect.x,
        viewport_rect.y,
        viewport_rect.w,
        viewport_rect.h,
        VOID_COLOR,
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
            color_for_material(span.material_id),
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

fn color_for_material(material_id: MaterialId) -> Color {
    match material_id {
        1 => Color::from_rgba(75, 145, 255, 255),
        2 => Color::from_rgba(240, 150, 55, 255),
        _ => MAGENTA,
    }
}

#[cfg(test)]
mod tests {
    use super::{color_for_material, project_view_spans};
    use crate::{
        map::Wall,
        raycast::{RayHit, ViewRaySample},
    };
    use macroquad::prelude::Color;
    use std::f32::consts::FRAC_PI_4;

    #[test]
    fn nearer_hit_projects_taller_span() {
        let samples = [
            sample_with_hit(0, 0.0, 40.0, 1),
            sample_with_hit(1, 0.0, 80.0, 1),
        ];

        let spans = project_view_spans(&samples, 0.0, 200.0, 120.0);

        let near_height = spans[0].bottom_y - spans[0].top_y;
        let far_height = spans[1].bottom_y - spans[1].top_y;

        assert!(near_height > far_height);
    }

    #[test]
    fn symmetric_rays_keep_matching_heights_after_fisheye_correction() {
        let samples = [
            sample_with_hit(0, -FRAC_PI_4 * 0.5, 100.0, 1),
            sample_with_hit(1, FRAC_PI_4 * 0.5, 100.0, 1),
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
            sample_with_hit(0, -0.2, 50.0, 1),
            sample_with_hit(1, 0.0, 60.0, 1),
            sample_with_hit(2, 0.2, 70.0, 1),
        ];

        let spans = project_view_spans(&samples, 0.0, 300.0, 120.0);

        assert!(spans[0].screen_x < spans[1].screen_x);
        assert!(spans[1].screen_x < spans[2].screen_x);
    }

    #[test]
    fn material_one_maps_to_expected_color() {
        assert_eq!(color_for_material(1), Color::from_rgba(75, 145, 255, 255));
    }

    #[test]
    fn material_two_maps_to_expected_color() {
        assert_eq!(color_for_material(2), Color::from_rgba(240, 150, 55, 255));
    }

    fn sample_with_hit(index: usize, angle: f32, distance: f32, material_id: u8) -> ViewRaySample {
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
            }),
            end_x: 0.0,
            end_y: 0.0,
        }
    }
}
