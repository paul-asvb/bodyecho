---
description: Generate game assets using Imagen-4 via Replicate API
---

Generate game assets for the Bodyecho game using Imagen-4 hosted on Replicate via direct API calls.

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

3. Use the Replicate API via curl to generate images with Imagen-4:
   - Model: `google/imagen-4`
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

   For tiles and other assets, save to:
   ```
   assets/tiles/[asset_name].png
   assets/tiles/[asset_name].json
   ```

5. For characters, create metadata.json with:
   ```json
   {
     "name": "[Asset Name]",
     "size": {"width": 64, "height": 64},
     "directions": ["south", "south-east", "east", "north-east", "north", "north-west", "west", "south-west"],
     "animations": {},
     "description": "[User's description]"
   }
   ```

   For tiles, create a JSON metadata file with:
   ```json
   {
     "name": "[Asset Name]",
     "type": "tile",
     "size": {"width": 1024, "height": 1024},
     "tileable": true,
     "description": "[Description]",
     "style": "pixel_art",
     "tags": ["tag1", "tag2"],
     "generated": {
       "model": "google/imagen-4",
       "date": "YYYY-MM-DD",
       "prompt": "[Generation prompt]"
     }
   }
   ```

## Important Notes

- Generate UUID for new characters using a standard UUID v4 format
- Ensure all prompts maintain consistent style across directional views
- Verify the Replicate API token is set in environment: REPLICATE_API_TOKEN
- For characters, mention in prompts: "viewed from [direction], top-down perspective"
- Keep pixel art style consistent with existing game assets

## Replicate API Usage

Use curl with the Bash tool to interact with the Imagen-4 model via the Replicate API. The typical flow:

1. Generate the image using curl with the `Prefer: wait` header to wait for completion:
   ```bash
   curl --silent --show-error https://api.replicate.com/v1/models/google/imagen-4/predictions \
       --request POST \
       --header "Authorization: Bearer $REPLICATE_API_TOKEN" \
       --header "Content-Type: application/json" \
       --header "Prefer: wait" \
       --data @- <<'EOM'
   {
       "input": {
         "prompt": "[Your detailed prompt here]",
         "aspect_ratio": "1:1",
         "safety_filter_level": "block_medium_and_above"
       }
   }
   EOM
   ```

2. The API will return JSON with an `output` field containing the image URL
3. Download the image using curl: `curl -o [output_file] [image_url]`
4. Convert to PNG if needed: `ffmpeg -i [input.jpg] [output.png] -y`
5. Save to the appropriate directory structure

Begin by gathering the necessary information from the user and then proceed with generation.
