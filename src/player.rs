use crate::map::Map;

pub const MOVE_SPEED: f32 = 140.0;
pub const TURN_SPEED: f32 = 2.5;

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PlayerIntent {
    pub forward: f32,
    pub turn: f32,
}

#[derive(Debug)]
pub struct Player {
    x: f32,
    y: f32,
    facing_angle: f32,
}

impl Player {
    pub fn new(x: f32, y: f32, facing_angle: f32) -> Self {
        Self { x, y, facing_angle }
    }

    pub fn x(&self) -> f32 {
        self.x
    }

    pub fn y(&self) -> f32 {
        self.y
    }

    pub fn facing_angle(&self) -> f32 {
        self.facing_angle
    }

    pub fn step(&mut self, intent: PlayerIntent, map: &Map, step_seconds: f32) {
        self.facing_angle += intent.turn * TURN_SPEED * step_seconds;

        let move_distance = intent.forward * MOVE_SPEED * step_seconds;
        if move_distance == 0.0 {
            return;
        }

        let next_x = self.x + self.facing_angle.cos() * move_distance;
        let next_y = self.y + self.facing_angle.sin() * move_distance;

        if !map.is_blocked_at_world(next_x, self.y) {
            self.x = next_x;
        }

        if !map.is_blocked_at_world(self.x, next_y) {
            self.y = next_y;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Player, PlayerIntent, MOVE_SPEED, TURN_SPEED};
    use crate::map::{Map, TILE_SIZE};
    use std::f32::consts::FRAC_PI_2;

    const STEP_SECONDS: f32 = 1.0 / 60.0;
    const FLOAT_TOLERANCE: f32 = 1.0e-4;

    #[test]
    fn moving_forward_for_known_ticks_reaches_expected_position() {
        let map = Map::from_rows(&["....", "....", "....", "...."]).expect("map should parse");
        let mut player = Player::new(TILE_SIZE * 1.5, TILE_SIZE * 1.5, 0.0);

        for _ in 0..30 {
            player.step(
                PlayerIntent {
                    forward: 1.0,
                    turn: 0.0,
                },
                &map,
                STEP_SECONDS,
            );
        }

        let expected_x = TILE_SIZE * 1.5 + MOVE_SPEED * STEP_SECONDS * 30.0;
        assert!((player.x() - expected_x).abs() < FLOAT_TOLERANCE);
        assert!((player.y() - TILE_SIZE * 1.5).abs() < FLOAT_TOLERANCE);
    }

    #[test]
    fn turning_for_known_ticks_reaches_expected_angle() {
        let map = Map::from_rows(&["....", "...."]).expect("map should parse");
        let mut player = Player::new(TILE_SIZE * 1.5, TILE_SIZE * 0.5, 0.0);

        for _ in 0..24 {
            player.step(
                PlayerIntent {
                    forward: 0.0,
                    turn: 1.0,
                },
                &map,
                STEP_SECONDS,
            );
        }

        let expected_angle = TURN_SPEED * STEP_SECONDS * 24.0;
        assert!((player.facing_angle() - expected_angle).abs() < FLOAT_TOLERANCE);
        assert!((player.x() - TILE_SIZE * 1.5).abs() < FLOAT_TOLERANCE);
        assert!((player.y() - TILE_SIZE * 0.5).abs() < FLOAT_TOLERANCE);
    }

    #[test]
    fn moving_into_blocked_tile_keeps_player_in_place() {
        let map = Map::from_rows(&[".2", ".."]).expect("map should parse");
        let mut player = Player::new(TILE_SIZE - 1.0, TILE_SIZE * 0.5, 0.0);
        let starting_x = player.x();
        let starting_y = player.y();

        player.step(
            PlayerIntent {
                forward: 1.0,
                turn: 0.0,
            },
            &map,
            STEP_SECONDS,
        );

        assert!((player.x() - starting_x).abs() < FLOAT_TOLERANCE);
        assert!((player.y() - starting_y).abs() < FLOAT_TOLERANCE);
    }

    #[test]
    fn moving_backward_uses_current_facing_direction() {
        let map = Map::from_rows(&["....", "....", "...."]).expect("map should parse");
        let mut player = Player::new(TILE_SIZE * 1.5, TILE_SIZE * 1.5, FRAC_PI_2);

        for _ in 0..15 {
            player.step(
                PlayerIntent {
                    forward: -1.0,
                    turn: 0.0,
                },
                &map,
                STEP_SECONDS,
            );
        }

        let expected_y = TILE_SIZE * 1.5 - MOVE_SPEED * STEP_SECONDS * 15.0;
        assert!((player.x() - TILE_SIZE * 1.5).abs() < FLOAT_TOLERANCE);
        assert!((player.y() - expected_y).abs() < FLOAT_TOLERANCE);
        assert!((player.facing_angle() - FRAC_PI_2).abs() < FLOAT_TOLERANCE);
    }

    #[test]
    fn replaying_the_same_inputs_produces_the_same_result() {
        let map = Map::from_rows(&["....", "....", "....", "...."]).expect("map should parse");
        let input_sequence = [
            PlayerIntent {
                forward: 1.0,
                turn: 0.0,
            },
            PlayerIntent {
                forward: 1.0,
                turn: 0.0,
            },
            PlayerIntent {
                forward: 0.0,
                turn: 1.0,
            },
            PlayerIntent {
                forward: 1.0,
                turn: 1.0,
            },
            PlayerIntent {
                forward: -1.0,
                turn: 0.0,
            },
        ];
        let mut first_run = Player::new(TILE_SIZE * 1.5, TILE_SIZE * 1.5, 0.0);
        let mut second_run = Player::new(TILE_SIZE * 1.5, TILE_SIZE * 1.5, 0.0);

        for intent in input_sequence {
            first_run.step(intent, &map, STEP_SECONDS);
        }

        for intent in input_sequence {
            second_run.step(intent, &map, STEP_SECONDS);
        }

        assert!((first_run.x() - second_run.x()).abs() < FLOAT_TOLERANCE);
        assert!((first_run.y() - second_run.y()).abs() < FLOAT_TOLERANCE);
        assert!((first_run.facing_angle() - second_run.facing_angle()).abs() < FLOAT_TOLERANCE);
    }
}
