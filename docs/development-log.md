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
