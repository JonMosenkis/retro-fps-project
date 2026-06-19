# Retro FPS Rust Prototype

Learning-focused retro FPS prototype in Rust.

The current goal is a small Wolfenstein-style raycaster built step by step with `macroquad`, keeping each stage visible, testable, and easy to understand.

## Current State

- A `macroquad` window opens with a split view.
- The left side shows a simple 3D wall view with ceiling, floor, and distance shading.
- The right side shows a top-down debug map with the player marker and ray fan.
- `W` and `S` move forward and backward.
- `A` and `D` rotate left and right.
- Walls block movement.

For a more detailed current-state summary, see [docs/ai-project-snapshot.md](docs/ai-project-snapshot.md).

## Run The Project

```bash
cargo run
```

## Development Setup

The game itself only needs Rust and Cargo.

For development checks, install these Rust components once:

```bash
rustup component add rustfmt clippy
```

## Quality Checks

Before declaring a coding task complete, run:

```bash
./scripts/check.sh
```

That script runs:

- `cargo fmt --check`
- `cargo clippy --all-targets --all-features -- -D warnings`
- `cargo test`

If `clippy` is missing, the script fails with a short setup message.

## Project Docs

- Agent workflow: [AGENTS.md](AGENTS.md)
- Task planning template: [docs/task-template.md](docs/task-template.md)
- Current project snapshot: [docs/ai-project-snapshot.md](docs/ai-project-snapshot.md)
- Development history: [docs/development-log.md](docs/development-log.md)
