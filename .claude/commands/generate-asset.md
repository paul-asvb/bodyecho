---
description: Generate game assets using Imagen-4 via Replicate API
---

Generate game assets for the Bodyecho game using Imagen-4 hosted on Replicate.

## Task Details

The user wants to generate a game asset. Please gather the following information if not already provided:

1. **Asset Type**: What type of asset to generate (character, item, tile, etc.)
2. **Description**: A detailed description of what the asset should look like
3. **Asset Name**: A name for this asset (will be used for file organization)

## Asset Requirements

For **character** assets, the game requires:
- 64x64 pixel sprite images
- 8 directional views: south, south-east, east, north-east, north, north-west, west, south-west
- Pixel art style with clean, simple design
- Transparent background
- Top-down perspective

For **other** asset types:
- 64x64 pixel size (or specify custom size)
- Pixel art style matching the game aesthetic
- Transparent background

## Generation Process

1. Ask the user for any missing information using the AskUserQuestion tool
2. Generate appropriate prompts for Imagen-4 that include:
   - The asset description
   - Technical requirements (64x64 pixels, pixel art style, transparent background, etc.)
   - Direction/view information (for characters)
   - Game aesthetic context

3. Use the MCP Replicate tool to generate images with Imagen-4:
   - Model: `google-deepmind/imagen-4`
   - For characters: Generate each directional view separately
   - Save images to the appropriate directory structure

4. For characters, create the directory structure:
   ```
   assets/characters/[UUID]/
   ├── metadata.json
   └── rotations/
       ├── south.png
       ├── south-east.png
       ├── east.png
       ├── north-east.png
       ├── north.png
       ├── north-west.png
       ├── west.png
       └── south-west.png
   ```

5. Create metadata.json with:
   ```json
   {
     "name": "[Asset Name]",
     "size": {"width": 64, "height": 64},
     "directions": ["south", "south-east", "east", "north-east", "north", "north-west", "west", "south-west"],
     "animations": {},
     "description": "[User's description]"
   }
   ```

## Important Notes

- Generate UUID for new characters using a standard UUID v4 format
- Ensure all prompts maintain consistent style across directional views
- Verify the Replicate API token is set in environment: REPLICATE_API_TOKEN
- For characters, mention in prompts: "viewed from [direction], top-down perspective"
- Keep pixel art style consistent with existing game assets

## MCP Tool Usage

Use the MCP Replicate tools to interact with the Imagen-4 model. The typical flow:
1. Call the replicate tool with the appropriate model ID and parameters
2. Wait for generation to complete
3. Download and save the generated images

Begin by gathering the necessary information from the user and then proceed with generation.
