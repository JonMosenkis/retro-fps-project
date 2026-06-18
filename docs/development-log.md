# Development Log

This file is append-only. It records what changed by development stage so future AI agents can quickly understand how the current prototype was built.

## Stage 1 - Project scaffold and top-down map view
- Commit: `74c7725` (`Initial project scaffold`)
- Date: 2026-06-18
- User-visible result:
  - The project could open a `macroquad` window and draw a top-down debug map.
- What was added:
  - Rust project setup with `macroquad` in `Cargo.toml`
  - hard-coded level rows in `src/main.rs`
  - `Map` parsing and validation in `src/map.rs`
  - top-down tile rendering in `src/debug_view.rs`
  - initial unit tests for map dimensions, tile lookup, and invalid map input
- Architectural result:
  - Clear separation between app setup (`main.rs`), map data (`map.rs`), and debug rendering (`debug_view.rs`)
- Scope at the end of the stage:
  - The map was visible on screen, but there was no player, no input handling, and no movement yet.

## Stage 2 - Deterministic player movement and wall collision
- Commit: `da5f027` (`Add deterministic player movement and collision`)
- Date: 2026-06-18
- User-visible result:
  - A player marker appeared on the map and could move with `W/S` and rotate with `A/D`.
  - The player collided with walls and could slide along them.
- What was added:
  - new `src/player.rs` module for player state and movement simulation
  - fixed-step simulation loop in `src/main.rs`
  - frame-time clamp and input sampling in `src/main.rs`
  - world-space blocking queries in `src/map.rs`
  - player rendering in `src/debug_view.rs`
  - unit tests for movement, turning, collision, backward motion, and deterministic replay
- Architectural result:
  - The codebase moved from static rendering to a small playable simulation with explicit ownership split across map, player, rendering, and app loop modules.
- Scope at the end of the stage:
  - The prototype supported a controllable player in a top-down debug map, but still had no raycasting or first-person rendering.

## Current Summary
- Stage 1 established the world representation and visible debug map.
- Stage 2 added the first interactive gameplay behavior on top of that map.

## Stage 3 - Two visible wall materials in the debug map
- Date: 2026-06-18
- User-visible result:
  - The top-down map now shows two different wall colors instead of one generic wall color.
  - The player can move around both wall materials, and both still block movement the same way.
- What was added:
  - `Wall` data in `src/map.rs` with a `material_id` field
  - wall parsing from `1` and `2` map characters into material-carrying tiles
  - updated blocking logic so all wall materials count as solid
  - distinct wall colors in `src/debug_view.rs`
  - revised sample level in `src/main.rs` so both wall materials are visible during manual play
  - new unit-test coverage for parsing and blocking behavior across both wall materials
- Architectural result:
  - The map model now preserves wall material data directly on each wall tile, which prepares the codebase for future ray hits that need to know which wall material was struck.
- Scope at the end of the stage:
  - The prototype still uses only the top-down debug view, but it can now represent more than one wall material without adding textures or 3D rendering.

## Stage 4 - Debug ray fan cast through the grid
- Date: 2026-06-18
- User-visible result:
  - The top-down debug map now shows a fan of rays extending from the player.
  - Rays rotate with `A/D`, move with `W/S`, and stop at the first wall they hit.
- What was added:
  - new `src/raycast.rs` module with single-ray hit queries and multi-ray debug fan generation
  - DDA-style grid stepping that preserves hit position, tile coordinates, distance, and wall material
  - ray rendering in `src/debug_view.rs`
  - frame-by-frame ray fan wiring in `src/main.rs`
  - unit tests for nearby hits, material preservation, no-hit exits, fan ordering, and axis-aligned rays
- Architectural result:
  - Ray hit math is now isolated from rendering and player movement, which creates a clean base for the later first-person wall-column step.
- Scope at the end of the stage:
  - The prototype can visibly cast rays in the debug map, but it still does not render first-person wall columns, textures, or shading.

## Stage 5 - Split 3D panel projected from view-ray samples
- Date: 2026-06-18
- User-visible result:
  - The window now shows a simple 3D wall view on the left and a scaled top-down debug view on the right.
  - Moving and turning update both panels together, so the player can compare the 3D projection against the same rays in the debug map.
- What was added:
  - new `src/view_3d.rs` module for projecting ray hits into vertical spans and drawing the 3D panel
  - `ViewRaySample` data in `src/raycast.rs` so one ray fan can feed both the 3D panel and the debug panel
  - scaled split-panel debug rendering in `src/debug_view.rs`
  - split-screen layout and per-frame 3D projection wiring in `src/main.rs`
  - unit tests for 3D span projection behavior and color mapping
- Architectural result:
  - The code now separates world sampling (`raycast.rs`) from screen-space projection (`view_3d.rs`) and from debug visualization (`debug_view.rs`), which keeps the current wall-only step readable without locking the project into a wall-specific renderer API.
- Scope at the end of the stage:
  - The prototype can render flat-colored 3D wall spans, but it still has no textures, floor or ceiling rendering, sprites, doors, or broader scene content.

## Stage 6 - Ceiling, floor, and distance-shaded walls
- Date: 2026-06-18
- User-visible result:
  - The 3D panel now shows a distinct ceiling and floor instead of a single void background.
  - Wall columns get darker as they get farther away, which makes depth easier to read while moving.
- What was added:
  - ceiling and floor band drawing in `src/view_3d.rs`
  - simple clamped distance shading for wall colors in `src/view_3d.rs`
  - unit tests for shading brightness, clamp behavior, material distinction after shading, and midpoint horizon placement
- Architectural result:
  - Scene appearance remains owned entirely by `src/view_3d.rs`, while ray sampling and app wiring stay unchanged.
- Scope at the end of the stage:
  - The prototype has a more readable wall-only 3D scene, but it still has no textures, sprite rendering, doors, enemies, or other world objects.
- Follow-up note:
  - The current distance shading is a simple camera-distance cue, not world lighting.
  - A likely next visual step is side shading based on whether a ray hit a vertical or horizontal wall face, which should feel closer to classic raycasters without adding a full lighting system.
