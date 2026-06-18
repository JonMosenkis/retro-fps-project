# Project Snapshot

## Purpose
- Learning-focused retro FPS prototype in Rust.
- Current prototype goal: build a Wolfenstein-style raycaster step by step, starting with a top-down debug map and player movement.

## Current Vertical Slice
- Running the project opens a `macroquad` window titled `Retro FPS Debug Map`.
- The screen shows a top-down grid map with two wall materials and empty floor tiles.
- A player marker appears as a circle with a short facing-direction line.
- `W` moves forward, `S` moves backward, `A` turns left, and `D` turns right.
- Both wall tile types block movement, so the player cannot move through either one.
- Simulation runs on a fixed 60 Hz update step.

## Implemented Systems

### Map
- Responsibility: own the fixed tile grid and answer tile/blocking queries.
- Files: `src/map.rs`
- Important rules and invariants:
  - `1` means a wall with material id `1`, `2` means a wall with material id `2`, and `.` means empty floor.
  - Tile queries preserve wall material data as `Tile::Wall(Wall { material_id })`.
  - All rows must have the same width.
  - Invalid map input is rejected during parsing.
  - Any wall material is blocking.
  - World positions outside the map count as blocked.
  - `TILE_SIZE` is `48.0` world units.

### Player Simulation
- Responsibility: own player position and facing direction, and apply movement input.
- Files: `src/player.rs`
- Important rules and invariants:
  - Movement uses the current facing angle.
  - Turning and movement are time-step based.
  - Collision is resolved separately on the X and Y axes.
  - Separate axis checks allow wall sliding.
  - `MOVE_SPEED` is `140.0`.
  - `TURN_SPEED` is `2.5`.

### Debug Rendering
- Responsibility: draw the current map and player state in a top-down debug view.
- Files: `src/debug_view.rs`
- Important rules and invariants:
  - Rendering is top-down only.
  - Material `1` and material `2` use different solid colors for easy inspection.
  - Unknown material ids render as `MAGENTA` so missing debug colors are obvious.
  - Map tiles are offset on screen rather than drawn at the window origin.

### App Loop and Input
- Responsibility: create the level, read keyboard input, advance simulation, and call render functions.
- Files: `src/main.rs`
- Important rules and invariants:
  - The current level is hard-coded in `LEVEL_ROWS`.
  - The player starts at `1.5` tiles from the left and top edges.
  - Frame time is clamped to avoid very large simulation steps.
  - Input sampling happens once per rendered frame.
  - Simulation may run multiple fixed updates per frame if needed.

## Runtime Flow
1. `main.rs` creates a `Map` from `LEVEL_ROWS`.
2. `main.rs` creates a `Player` with a fixed starting position and facing angle.
3. Each frame reads keyboard input into a `PlayerIntent`.
4. Frame time is accumulated and processed in fixed `1.0 / 60.0` second simulation steps.
5. Each simulation step calls `Player::step(intent, &map, step_seconds)`.
6. Rendering clears the screen, draws the map, then draws the player.

## File Ownership
- `src/main.rs`: app setup, hard-coded level data, keyboard input, fixed-step loop
- `src/map.rs`: map data model, parsing, tile lookup, world collision queries
- `src/player.rs`: player state and movement simulation
- `src/debug_view.rs`: top-down debug rendering

## Tests That Exist
- `src/map.rs` covers map parsing, tile lookup, bounds behavior, and blocked-vs-empty world queries.
- `src/player.rs` covers forward/backward movement, turning, wall collision, and deterministic replay of the same input sequence.

## Manual QA
- Run: `cargo run`
- Expect:
  - a window opens
  - a top-down tile map is visible with two different wall colors
  - a player marker and facing line are visible
  - `W/S` move forward/backward
  - `A/D` rotate the player
  - the player stops at both wall materials and slides along them when moving diagonally into edges over multiple frames

## Known Limits
- No raycasting yet.
- No first-person 3D wall rendering yet.
- No textures, floor casting, or ceiling rendering.
- No doors, enemies, weapons, pickups, damage, or exit condition.
- No external level loading; the level is hard-coded in `main.rs`.
- No separate game-state module yet.

## Update Rules
- Update this file whenever a new visible feature, module, or API boundary is added.
- Keep this document focused on current truth, not future plans.
- Prefer short bullets over prose.
- Prefer responsibilities and invariants over full function inventories.
- Explicitly remove or revise outdated statements when behavior changes.
