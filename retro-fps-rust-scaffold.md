# Retro FPS Learning Project Scaffold

## Project Direction

Build a 90s-style first-person shooter inspired by Wolfenstein 3D and Doom, with the primary goal being learning rather than shipping quickly.

The recommended first target is a Wolfenstein-style raycaster:

- Grid-based levels
- 2D map data rendered as a fake 3D view
- Textured wall slices
- Billboard sprites for enemies, pickups, and decorations
- Simple collision, doors, shooting, health, ammo, and an exit condition

This is more approachable than starting with a Doom-style sector engine, while still teaching the important ideas behind early FPS engines.

## Language Choice
&
Use Rust.

Rust is a good fit if the goal is to learn modern systems programming while building a retro FPS. Compared with C, Rust adds safety, stronger tooling, Cargo package management, better enums, modules, tests, and long-term maintainability.

The tradeoff is that Rust makes ownership and shared mutable state explicit. That can add friction around entities, asset caches, game state, and rendering data, but it is useful learning if the project is approached incrementally.

## Recommended Starting Stack

- Language: Rust
- Framework: macroquad
- Package/build tool: Cargo
- Editor: VS Code or preferred editor
- Version control: Git
- Sprite/pixel art: Aseprite
- Sound editing: Audacity
- Retro sound effects: Bfxr or jsfxr

macroquad is recommended for the first prototype because it provides windowing, input, drawing, timing, and audio without requiring a full game engine or low-level graphics setup.

## Why Not Start With Godot?

Godot is a strong option for eventually shipping a game, but it hides many of the systems this project is meant to teach.

For learning how a retro FPS works internally, start closer to the rendering and simulation code. After a working prototype exists, moving to Godot or another engine can be reconsidered.

## First Vertical Slice

The first milestone should be small and playable:

1. Open a window.
2. Represent a level as a 2D grid.
3. Add player position, direction, and movement.
4. Cast rays into the grid.
5. Draw vertical wall columns.
6. Add basic wall colors or textures.
7. Add collision against walls.
8. Add one enemy sprite.
9. Add one weapon.
10. Add shooting, enemy damage, player damage, health, ammo, and an exit tile.

The goal is one tiny complete level, not a general engine.

## Later Expansion

After the first playable slice:

- Textured walls, floors, and ceilings
- Doors and keys
- Pickups and secrets
- Multiple enemies
- Weapon switching
- Sprite animation
- Sound and music
- Simple map editor or importer
- Better level format
- Title screen and menus

Only consider Doom-style features after the Wolfenstein-style prototype works:

- Sector-based maps
- Variable floor and ceiling heights
- Stairs, lifts, and moving platforms
- More complex visibility and collision

## AI Usage

AI can help with:

- Explaining rendering/math concepts
- Generating Rust scaffolding
- Debugging borrow checker and architecture issues
- Designing level formats
- Creating placeholder texture and sprite prompts
- Building small tools
- Iterating on enemy, weapon, and level ideas

AI will not replace:

- Scope control
- Game feel
- Level design judgment
- Asset consistency
- Finishing decisions

Use AI as a collaborator, but keep the project small and playable early.

## Next Conversation Starting Point

Start by creating a Rust + macroquad project for a Wolfenstein-style raycaster prototype.

Initial implementation target:

- `cargo new retro_fps`
- Add `macroquad`
- Create a fixed-size grid map
- Render a top-down debug view
- Add player movement and rotation
- Begin raycasting walls
