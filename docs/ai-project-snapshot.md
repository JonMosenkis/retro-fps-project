# Project Snapshot

## Purpose
- Learning-focused retro FPS prototype in Rust.
- Current prototype goal: build a Wolfenstein-style raycaster step by step, starting with a top-down debug map and player movement.

## Current Vertical Slice
- Running the project opens a `macroquad` window titled `Retro FPS Debug Map`.
- The left side shows a simple 3D scene with a flat ceiling band, a flat floor band, and shaded wall spans.
- The right side shows a scaled top-down debug map with the player marker and ray fan.
- `W` moves forward, `S` moves backward, `A` turns left, and `D` turns right.
- Both wall tile types block movement, so the player cannot move through either one.
- Moving and turning updates both views from the same player state.
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
  - Rendering is top-down only and lives in the right-side split panel.
  - The debug map scales to fit its viewport instead of assuming a fixed tile size on screen.
  - Rays are drawn from already-computed world-space endpoints.
  - Material `1` and material `2` use different solid colors for easy inspection.
  - Unknown material ids render as `MAGENTA` so missing debug colors are obvious.
  - The module owns the whole debug panel, including its background.

### 3D View Projection
- Responsibility: project ray hits into vertical screen spans and draw the left-side 3D panel.
- Files: `src/view_3d.rs`
- Important rules and invariants:
  - The 3D panel uses one vertical span per ray hit.
  - The viewport horizon stays fixed at the vertical midpoint.
  - Projection uses fisheye-corrected distance so edge rays do not look obviously stretched.
  - Only hit rays create spans; rays that leave the map create no 3D wall slice.
  - The renderer fills the top half with a ceiling color and the bottom half with a floor color before drawing walls.
  - Wall colors come from material-specific base colors plus simple clamped distance darkening.
  - Unknown material ids still render as `MAGENTA` so missing wall colors stay obvious.

### Raycasting
- Responsibility: cast rays through the tile grid and report first wall hits in world space.
- Files: `src/raycast.rs`
- Important rules and invariants:
  - A single ray returns `Option<RayHit>` so no-hit cases stay simple.
  - `cast_view_rays` returns left-to-right ray samples for the active field of view.
  - Ray stepping checks one grid boundary at a time until a wall is hit or the ray exits the map.
  - Hit results preserve the struck wall material and tile coordinates.
  - The debug ray fan is fixed at `31` rays across a `60` degree field of view.
  - Rays that leave the map without hitting a wall still end at the map edge in the debug view.

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
6. Rendering casts the current view ray fan from the player state.
7. Rendering projects ray hits into 3D vertical spans for the left panel.
8. Rendering draws ceiling, floor, and shaded wall spans in the 3D panel.
9. Rendering draws the scaled debug panel from the same ray samples.

## File Ownership
- `src/main.rs`: app setup, hard-coded level data, keyboard input, fixed-step loop
- `src/map.rs`: map data model, parsing, tile lookup, world collision queries
- `src/player.rs`: player state and movement simulation
- `src/raycast.rs`: grid ray stepping and view-ray sample generation
- `src/view_3d.rs`: 3D projection math and 3D panel drawing
- `src/debug_view.rs`: scaled top-down debug panel rendering

## Tests That Exist
- `src/map.rs` covers map parsing, tile lookup, bounds behavior, and blocked-vs-empty world queries.
- `src/player.rs` covers forward/backward movement, turning, wall collision, and deterministic replay of the same input sequence.
- `src/raycast.rs` covers first-hit distance, material preservation, no-hit map exits, left-to-right sampling order, and axis-aligned rays.
- `src/view_3d.rs` covers distance-based span height, fisheye correction symmetry, no-hit omission, left-to-right span order, base material colors, distance shading clamps, shaded material distinction, and horizon placement.

## Developer Workflow
- `README.md` is the top-level human-facing entry point for project purpose, run instructions, and development setup.
- Run `./scripts/check.sh` before declaring a coding task complete.
- The check script runs `cargo fmt --check`, `cargo clippy --all-targets --all-features -- -D warnings`, and `cargo test`.
- `clippy` is a development-time requirement, not a runtime dependency.
- If `clippy` is missing locally, install it once with `rustup component add clippy`.
- `docs/subagents.md` defines optional focused reviewer roles for architecture, Rust clarity, correctness, and playability checks.

## Manual QA
- Run: `cargo run`
- Expect:
  - a window opens
  - the left side shows a ceiling band, a floor band, and shaded wall columns
  - the right side shows a scaled top-down tile map with the player marker and visible ray fan
  - `W/S` move forward/backward
  - `A/D` rotate the player
  - the 3D view and the debug rays rotate and move together
  - nearby walls appear taller and brighter in the 3D panel than distant walls
  - the player stops at both wall materials and slides along them when moving diagonally into edges over multiple frames

## Known Limits
- No textures, floor casting, or ceiling textures.
- No doors, enemies, weapons, pickups, damage, or exit condition.
- No external level loading; the level is hard-coded in `main.rs`.
- No separate game-state module yet.

## Update Rules
- Update this file whenever a new visible feature, module, or API boundary is added.
- Keep this document focused on current truth, not future plans.
- Prefer short bullets over prose.
- Prefer responsibilities and invariants over full function inventories.
- Explicitly remove or revise outdated statements when behavior changes.
