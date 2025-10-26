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

The site is served from the `gh-pages` branch. Adjust the `public_url` value in `Trunk.toml` if you use a custom domain or deploy to a different path.
