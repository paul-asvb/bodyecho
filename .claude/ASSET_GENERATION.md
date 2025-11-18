# Asset Generation Tool for Bodyecho

This guide explains how to use the Claude Code asset generation tool to create game assets using Imagen-4 via Replicate.

## Setup

### 1. Install MCP Replicate Server

The MCP server is configured in `.claude/mcp_servers.json` and will be automatically installed when Claude Code starts.

### 2. Set Your Replicate API Token

You need a Replicate API token to use this tool. Get one from https://replicate.com/account/api-tokens

Set the environment variable:

```bash
export REPLICATE_API_TOKEN="your-token-here"
```

Or add it to your shell profile (`~/.bashrc`, `~/.zshrc`, etc.):

```bash
echo 'export REPLICATE_API_TOKEN="your-token-here"' >> ~/.bashrc
source ~/.bashrc
```

### 3. Restart Claude Code

After setting the environment variable, restart Claude Code so it can pick up the MCP server configuration.

## Usage

### Generate a Character Asset

Use the `/generate-asset` slash command in Claude Code:

```
/generate-asset
```

Then provide:
- **Asset Type**: character
- **Description**: e.g., "A brave knight in silver armor with a red cape"
- **Asset Name**: e.g., "knight"

The tool will automatically:
1. Generate 8 directional views (N, NE, E, SE, S, SW, W, NW)
2. Create the proper directory structure under `assets/characters/`
3. Generate a unique UUID for the character
4. Create metadata.json with character information
5. Save all 8 directional sprite images as 64x64 pixel art

### Generate Other Assets

You can also generate:
- Items
- Tiles
- Environment objects
- UI elements

Just specify the asset type and description when prompted.

## Asset Specifications

### Characters
- **Size**: 64x64 pixels
- **Style**: Pixel art, clean and simple
- **Background**: Transparent
- **Perspective**: Top-down
- **Directions**: 8 (N, NE, E, SE, S, SW, W, NW)

### General Assets
- **Size**: 64x64 pixels (or custom)
- **Style**: Pixel art matching game aesthetic
- **Background**: Transparent

## Example Prompts

The tool will automatically generate prompts like:

> "64x64 pixel art sprite of a brave knight in silver armor with a red cape, viewed from the south, top-down perspective, transparent background, clean pixel art style, suitable for a 2D game"

Each direction will have a customized prompt to ensure proper orientation.

## Troubleshooting

### MCP Server Not Found
- Ensure you've restarted Claude Code after adding the configuration
- Check that `npx` is available in your PATH
- Verify `.claude/mcp_servers.json` exists

### API Token Error
- Verify `REPLICATE_API_TOKEN` is set: `echo $REPLICATE_API_TOKEN`
- Ensure the token is valid at https://replicate.com/account/api-tokens
- Restart Claude Code after setting the token

### Generation Quality Issues
- Refine your asset description with more detail
- Specify art style preferences
- Mention specific colors, features, or characteristics
- Review and regenerate individual directions if needed

## Cost Considerations

Replicate charges per generation. Each character requires 8 images (one per direction). Check current Imagen-4 pricing at https://replicate.com/google-deepmind/imagen-4

## Advanced Usage

### Batch Generation
You can ask Claude Code to generate multiple assets in sequence by providing a list of descriptions.

### Custom Metadata
After generation, you can manually edit the `metadata.json` file to add custom properties like:
- Animation frame counts
- Special abilities
- Tags or categories
- Custom properties for your game logic

### Post-Processing
Generated assets can be manually edited using your preferred pixel art tool to refine details or adjust the style.
