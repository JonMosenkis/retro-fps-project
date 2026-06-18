pub const TILE_SIZE: f32 = 48.0;

pub type MaterialId = u8;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Wall {
    pub material_id: MaterialId,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tile {
    Wall(Wall),
    Empty,
}

#[derive(Debug, PartialEq, Eq)]
pub enum MapError {
    EmptyMap,
    EmptyRow {
        row: usize,
    },
    InconsistentRowWidth {
        expected: usize,
        actual: usize,
        row: usize,
    },
    UnsupportedTile {
        row: usize,
        column: usize,
        character: char,
    },
}

#[derive(Debug)]
pub struct Map {
    width: usize,
    height: usize,
    tiles: Vec<Tile>,
}

impl Map {
    pub fn from_rows(rows: &[&str]) -> Result<Self, MapError> {
        if rows.is_empty() {
            return Err(MapError::EmptyMap);
        }

        let width = rows[0].chars().count();

        if width == 0 {
            return Err(MapError::EmptyRow { row: 0 });
        }

        let mut tiles = Vec::with_capacity(width * rows.len());

        for (row_index, row) in rows.iter().enumerate() {
            let row_width = row.chars().count();

            if row_width == 0 {
                return Err(MapError::EmptyRow { row: row_index });
            }

            if row_width != width {
                return Err(MapError::InconsistentRowWidth {
                    expected: width,
                    actual: row_width,
                    row: row_index,
                });
            }

            for (column_index, character) in row.chars().enumerate() {
                let tile = match character {
                    '1' => Tile::Wall(Wall { material_id: 1 }),
                    '2' => Tile::Wall(Wall { material_id: 2 }),
                    '.' => Tile::Empty,
                    _ => {
                        return Err(MapError::UnsupportedTile {
                            row: row_index,
                            column: column_index,
                            character,
                        });
                    }
                };

                tiles.push(tile);
            }
        }

        Ok(Self {
            width,
            height: rows.len(),
            tiles,
        })
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn tile_at(&self, x: usize, y: usize) -> Option<Tile> {
        if x >= self.width || y >= self.height {
            return None;
        }

        let index = y * self.width + x;
        self.tiles.get(index).copied()
    }

    pub fn is_blocked_at_world(&self, world_x: f32, world_y: f32) -> bool {
        let Some((tile_x, tile_y)) = self.world_to_tile(world_x, world_y) else {
            return true;
        };

        matches!(self.tile_at(tile_x, tile_y), Some(Tile::Wall(_)) | None)
    }

    fn world_to_tile(&self, world_x: f32, world_y: f32) -> Option<(usize, usize)> {
        if world_x < 0.0 || world_y < 0.0 {
            return None;
        }

        let tile_x = (world_x / TILE_SIZE).floor() as usize;
        let tile_y = (world_y / TILE_SIZE).floor() as usize;

        if tile_x >= self.width || tile_y >= self.height {
            return None;
        }

        Some((tile_x, tile_y))
    }
}

#[cfg(test)]
mod tests {
    use super::{Map, MapError, Tile, Wall, TILE_SIZE};

    #[test]
    fn from_rows_sets_expected_dimensions() {
        let map = Map::from_rows(&["111", "1.2"]).expect("map should parse");

        assert_eq!(map.width(), 3);
        assert_eq!(map.height(), 2);
    }

    #[test]
    fn tile_at_returns_expected_wall_materials_and_floor() {
        let map = Map::from_rows(&["1.", ".2"]).expect("map should parse");

        assert_eq!(
            map.tile_at(0, 0),
            Some(Tile::Wall(Wall { material_id: 1 }))
        );
        assert_eq!(map.tile_at(1, 0), Some(Tile::Empty));
        assert_eq!(map.tile_at(0, 1), Some(Tile::Empty));
        assert_eq!(
            map.tile_at(1, 1),
            Some(Tile::Wall(Wall { material_id: 2 }))
        );
    }

    #[test]
    fn from_rows_rejects_inconsistent_row_widths() {
        let error = Map::from_rows(&["111", "11"]).expect_err("map should fail");

        assert_eq!(
            error,
            MapError::InconsistentRowWidth {
                expected: 3,
                actual: 2,
                row: 1,
            }
        );
    }

    #[test]
    fn from_rows_rejects_unsupported_characters() {
        let error = Map::from_rows(&["1x"]).expect_err("map should fail");

        assert_eq!(
            error,
            MapError::UnsupportedTile {
                row: 0,
                column: 1,
                character: 'x',
            }
        );
    }

    #[test]
    fn tile_at_returns_none_when_out_of_bounds() {
        let map = Map::from_rows(&["11", "22"]).expect("map should parse");

        assert_eq!(map.tile_at(2, 0), None);
        assert_eq!(map.tile_at(0, 2), None);
    }

    #[test]
    fn is_blocked_at_world_returns_true_for_wall_type_a() {
        let map = Map::from_rows(&["1.", ".."]).expect("map should parse");

        assert!(map.is_blocked_at_world(TILE_SIZE * 0.5, TILE_SIZE * 0.5));
    }

    #[test]
    fn is_blocked_at_world_returns_true_for_wall_type_b() {
        let map = Map::from_rows(&["2.", ".."]).expect("map should parse");

        assert!(map.is_blocked_at_world(TILE_SIZE * 0.5, TILE_SIZE * 0.5));
    }

    #[test]
    fn is_blocked_at_world_returns_false_for_empty_tiles() {
        let map = Map::from_rows(&["1.", ".."]).expect("map should parse");

        assert!(!map.is_blocked_at_world(TILE_SIZE * 1.5, TILE_SIZE * 0.5));
    }

    #[test]
    fn is_blocked_at_world_returns_true_out_of_bounds() {
        let map = Map::from_rows(&["11", "22"]).expect("map should parse");

        assert!(map.is_blocked_at_world(-1.0, TILE_SIZE));
        assert!(map.is_blocked_at_world(TILE_SIZE * 2.0, TILE_SIZE));
        assert!(map.is_blocked_at_world(TILE_SIZE, TILE_SIZE * 2.0));
    }
}
