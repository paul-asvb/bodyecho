# PixelLab Asset Generation Command

Generate game assets using the PixelLab AI API. This command helps create characters, terrain tiles, and map objects for the Bodyecho game.

## Usage

The user will specify what type of asset they want to generate:
- **Character**: Directional character sprites with animations
- **Terrain**: Wang tiles for seamless terrain, sidescroller tiles, or isometric tiles
- **Map Object**: Objects to place on the map

## Your Task

When this command is invoked, you should:

1. **Ask the user for details** using the AskUserQuestion tool:
   - What type of asset? (character, terrain-wang, terrain-sidescroller, terrain-isometric, map-object)
   - Description of the asset (e.g., "a nurse in blue scrubs", "hospital floor tiles", "medical equipment")
   - Size/dimensions (if applicable)
   - Additional parameters based on asset type

2. **Generate the asset** by calling the PixelLab API:
   - Use WebFetch to call the appropriate PixelLab API endpoint at https://api.pixellab.ai
   - The API returns a job ID immediately
   - Poll the status endpoint to check when generation is complete (typically 2-5 minutes)
   - Download the generated asset files

3. **Save to the appropriate location**:
   - Characters: `assets/characters/{uuid}/`
   - Tiles: `assets/tiles/`
   - Objects: `assets/objects/`
   - Use UUID v4 for new character directories
   - Save metadata.json with generation parameters

4. **Report results** to the user:
   - Confirm successful generation
   - Show file paths where assets were saved
   - Provide any relevant metadata

## PixelLab API Reference

### Character Generation
- **Endpoint**: `/create_character`
- **Parameters**:
  - `description`: Text description of the character
  - `size`: Pixel size (16-128px, default 64)
  - `n_directions`: Number of directions (4 or 8)
  - `proportions`: Style preset (default, chibi, cartoon, stylized, realistic, heroic)
  - `outline`: Outline style
  - `shading`: Shading style
  - `detail_level`: Level of detail

### Wang Tileset (Top-Down Terrain)
- **Endpoint**: `/create_wang_tileset`
- **Parameters**:
  - `lower_description`: Base terrain description
  - `upper_description`: Overlay terrain description
  - `transition_size`: Blend range (0-0.5)
  - `tile_size`: Size of each tile
  - `view`: Perspective (RTS/RPG)

### Sidescroller Tileset
- **Endpoint**: `/create_sidescroller_tileset`
- **Parameters**:
  - `lower_description`: Platform material
  - `transition_description`: Top decoration
  - `transition_size`: Blend range
  - `tile_size`: Size of each tile

### Isometric Tiles
- **Endpoint**: `/create_isometric_tile`
- **Parameters**:
  - `description`: Tile description
  - `size`: Pixel size (32px recommended)
  - `tile_shape`: Shape (thin/thick/block)
  - `outline`: Outline style
  - `shading`: Shading style
  - `detail_level`: Level of detail

### Map Objects
- **Endpoint**: `/create_map_object`
- **Parameters**:
  - `description`: Object description
  - `width`: Width in pixels
  - `height`: Height in pixels
  - `view`: View angle
  - `outline`: Outline style
  - `shading`: Shading style
  - `detail_level`: Level of detail

## Workflow

1. Call the creation endpoint → get job_id
2. Poll status endpoint with job_id every 10-30 seconds
3. When status is "completed", download the assets
4. Save to appropriate directory with metadata
5. Report success to user

## Example

User: `/pixellab`
Assistant: Asks what type of asset and parameters
Assistant: Calls PixelLab API, polls for completion
Assistant: Downloads and saves assets
Assistant: Reports: "Created character sprites in assets/characters/abc-123-def/"

## Notes

- API calls are asynchronous and take 2-5 minutes
- Always save metadata.json with generation parameters for reproducibility
- For characters, maintain the directory structure: animations/{animation_name}/{direction}/frame_XXX.png
- Permission for WebFetch(domain:api.pixellab.ai) is already configured
