# SVG Christmas Light Shapes

This directory contains the SVG templates for Christmas light shapes used in the application.

## Available Shapes

- **bulb.svg** - Traditional pointed oval Christmas light bulb
- **star.svg** - 5-pointed star
- **candy-cane.svg** - Candy cane with red/white stripes
- **bell.svg** - Christmas bell with gold clapper

## Editing SVGs

Each SVG file uses the placeholder text `COLOR` which gets replaced at runtime with the actual light color:
- Red: `#ff0000`
- Green: `#00cc00`
- White: `#ffffff`
- Yellow: `#ffdd00`

To customize shapes:
1. Edit the SVG files directly
2. Keep the `COLOR` placeholder where you want the light's color applied
3. Maintain the `viewBox="0 0 100 140"` for consistent sizing
4. Keep the `#glow` filter ID for the glowing effect

## Adding New Shapes

To add a new shape:
1. Create a new SVG file in this directory
2. Use `viewBox="0 0 100 140"` for consistency
3. Add a glow filter definition
4. Use `COLOR` placeholder for dynamic coloring
5. Update `src/main.rs` to add the new shape to the `LightShape` enum and implement its rendering
