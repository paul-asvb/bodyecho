# Bodyecho

Bodyecho is a starter [Bevy](https://bevyengine.org/) project configured to build to WebAssembly and deploy automatically to GitHub Pages.

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install)
- `wasm32-unknown-unknown` target: `rustup target add wasm32-unknown-unknown`
- [`trunk`](https://trunkrs.dev/): `cargo install trunk`

## Local Development

```bash
trunk serve --open
```

This command compiles the game to WebAssembly, serves it locally, and opens your browser. Trunk watches for changes and rebuilds automatically.

## Deployment

Pushing to the `main` branch triggers the `Deploy Web` workflow in `.github/workflows/deploy.yml`. The workflow:

- Builds the project with `trunk build --release`
- Uploads the build artifacts
- Publishes the `dist/` directory to GitHub Pages

The site is served from the `gh-pages` branch and hosted under `/bodyecho/`. Adjust the `public_url` value in `Trunk.toml` if you use a custom domain or deploy to a different path.

## Asset Structure

The game assets are organized in the `assets/` directory:

```
assets/
└── characters/
    └── 582b204c-ad0f-401d-b99a-97ecaf9a0abe/
        ├── metadata.json              # Character metadata (size, name, directions, etc.)
        ├── rotations/                 # Static character images for each direction
        │   ├── south.png
        │   ├── south-east.png
        │   ├── east.png
        │   ├── north-east.png
        │   ├── north.png
        │   ├── north-west.png
        │   ├── west.png
        │   └── south-west.png
        └── animations/                # Frame-by-frame animations
            ├── breathing-idle/        # 4 frames per direction
            │   ├── south/
            │   │   ├── frame_000.png
            │   │   ├── frame_001.png
            │   │   ├── frame_002.png
            │   │   └── frame_003.png
            │   ├── south-east/
            │   ├── east/
            │   ├── north-east/
            │   ├── north/
            │   ├── north-west/
            │   ├── west/
            │   └── south-west/
            └── walk/                  # 6 frames per direction
                ├── south/
                │   ├── frame_000.png
                │   ├── frame_001.png
                │   ├── frame_002.png
                │   ├── frame_003.png
                │   ├── frame_004.png
                │   └── frame_005.png
                ├── south-east/
                ├── east/
                ├── north-east/
                ├── north/
                ├── north-west/
                ├── west/
                └── south-west/
```

### Character Animations

The character supports **8 directional movement** (N, NE, E, SE, S, SW, W, NW) with two animation states:

- **Breathing Idle**: 4 frames per direction - subtle breathing animation when stationary
- **Walk**: 6 frames per direction - walking animation for movement

Each frame is 64x64 pixels. Frames are stored individually and can be assembled into sprite sheets for use in the game engine.
