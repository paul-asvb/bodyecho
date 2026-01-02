# Agent Guide for Bodyecho

This repository contains a 2D game built with **Bevy 0.14** (Rust) that compiles to WebAssembly using **Trunk**.

## Quick Start
- **Build & Run**: `trunk serve --open`
- **Build Release**: `trunk build --release`
- **Language**: Rust
- **Engine**: Bevy 0.14

## Code Structure
- **Entry Point**: `src/main.rs` contains the main game logic, including:
    - `setup`: Spawns camera, tilemap, and player.
    - `move_player`: Handles WASD input and 8-directional movement.
    - `animate_sprite`: Manages animation frames based on state/direction.
    - `loading_screen_system`: Simulates a loading screen.
- **Resources**:
    - `CharacterAnimations`: Stores handles to idle/walk frames.
    - `LoadingProgress`: Tracks simulated loading state.
- **Components**:
    - `Player`: Stores speed, direction, and animation state.
    - `AnimationTimer` / `AnimationFrameIndex`: Controls sprite animation.

## Asset Convention
Assets are strictly organized in the `assets/` directory.

### Characters
Located in `assets/characters/{UUID}/`.
- **UUID**: Must be a valid v4 UUID.
- **Directory Structure**:
    ```
    assets/characters/{UUID}/
    ├── metadata.json
    └── animations/
        ├── breathing-idle/
        │   └── {direction}/frame_000.png ... (4 frames)
        └── walk/
            └── {direction}/frame_000.png ... (6 frames)
    ```
- **Directions**: `north`, `north-east`, `east`, `south-east`, `south`, `south-west`, `west`, `north-west`.

### Tiles
- Located in `assets/tiles/`.
- Managed via `bevy_ecs_tilemap`.

## PixelLab Asset Generation
This project uses a custom workflow for generating assets via AI.
When asked to generic/create assets (characters, tiles, objects), follow the **PixelLab Protocol**:

1.  **Gather Requirements**: Ask for description, type (character/terrain/object), and size.
2.  **Execute**: Use the `agentic_fetch` or `fetch` tool to interact with `https://api.pixellab.ai` (see `.claude/commands/pixellab.md` for specific endpoints and params).
    *   *Note*: If you cannot directly invoke the API due to tool limitations, explain the missing permission or instruct the user.
3.  **Poll & Download**: The API is async. Poll until completion, then download using the `download` tool.
4.  **Organize**: Save files to the correct `assets/` subdirectory using the conventions above.

## Development Patterns
- **Movement**: 8-way movement is normalized (`movement.normalize()`) to prevent faster diagonal speed.
- **Animation**: Frames reset to index 0 when state (Idle vs Walk) or direction changes.
- **Loading**: The loading screen is currently a simulation (timer-based), not actual asset loading measurement.
- **Window**: Fixed resolution logical 960x540, scaled to fit parent canvas.

## Deployment
- Deploys to GitHub Pages via `.github/workflows/deploy.yml`.
- Build output goes to `dist/`.
