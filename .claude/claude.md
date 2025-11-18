# Bodyecho Project Context

## Overview
Bodyecho is a 2D game built with Bevy 0.14 that compiles to WebAssembly and deploys to GitHub Pages. It features an animated character with 8-directional movement and a loading screen.

## Tech Stack
- **Engine**: Bevy 0.14 (Rust game engine)
- **Build Tool**: Trunk
- **Target**: WebAssembly (wasm32-unknown-unknown)
- **Deployment**: GitHub Pages via GitHub Actions

## Project Structure
- `src/main.rs` - Main game code with player movement, animation system, and loading screen
- `assets/` - Game assets including character sprites, backgrounds, and furniture
- `index.html` - HTML entry point for WASM
- `Cargo.toml` - Rust dependencies and build configuration
- `.github/workflows/` - CI/CD for automatic deployment

## Key Features

### Character System
- 8-directional movement (N, NE, E, SE, S, SW, W, NW)
- Two animation states:
  - **Breathing Idle**: 4 frames per direction
  - **Walk**: 6 frames per direction
- WASD controls for movement
- Player speed: 80.0 units/second

### Loading Screen
- Blue background (Color::srgb(0.1, 0.3, 0.8))
- Progress bar showing loading percentage
- Title and loading text
- Simulated 2-second loading time
- Automatically removes when loading completes

### Asset Structure
Character assets are organized by UUID under `assets/characters/582b204c-ad0f-401d-b99a-97ecaf9a0abe/`:
- `animations/breathing-idle/{direction}/frame_XXX.png` - Idle animation frames
- `animations/walk/{direction}/frame_XXX.png` - Walk animation frames
- `metadata.json` - Character metadata

## Development Commands
```bash
# Serve locally with hot reload
trunk serve --open

# Build for production
trunk build --release
```

## Important Implementation Details
- Animation frame index resets to 0 when state or direction changes
- Movement is normalized for diagonal movement to maintain consistent speed
- Loading progress is simulated (timer-based) - replace with actual asset loading as needed
- Window resolution: 960x540 with canvas fitting to parent
