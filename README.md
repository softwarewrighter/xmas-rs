# Christmas Lights - Rust/Yew Implementation

A Rust implementation of a Christmas lights blinking animation using the Yew framework. This project is based on [this CodePen](https://codepen.io/irfanezani_/pen/mdeLpKo).

## Features

- 8 animated Christmas lights in red, yellow, blue, and green
- On/Off controls
- Adjustable animation speed (1-5)
- Built with Yew (Rust frontend framework)
- Compiles to WebAssembly

## Prerequisites

- Rust (latest stable version)
- Trunk (for building and serving the app)

Install Trunk:
```bash
cargo install --locked trunk
```

Add the WebAssembly target:
```bash
rustup target add wasm32-unknown-unknown
```

## Project Configuration

- **Port:** 7030 (configured for parallel development)
- **URL:** http://localhost:7030

## Building and Running

All development scripts are in the `./scripts` directory.

### Development Server

To run the development server with hot-reloading:
```bash
./scripts/dev.sh
```

Then open your browser to `http://localhost:7030`

### Build Only

To create a clean build:
```bash
./scripts/build.sh
```

### Production Server

To build and serve:
```bash
./scripts/serve.sh
```

The output will be in the `dist/` directory.

## Usage

- Click **On** to start the lights animation
- Click **Off** to stop the animation
- Adjust the **Speed** input (1-5) to change animation speed in real-time
  - 1 = slowest (2 seconds per blink cycle)
  - 5 = fastest (0.5 seconds per blink cycle)

## Color Scheme

The lights use a festive Christmas color palette:
- **Red** - Primary color (#ff0000)
- **Green** - Primary color (#00cc00)
- **Yellow** - Accent color (#ffdd00)
- **White** - Accent color (#ffffff)

## Project Structure

- `src/main.rs` - Main Yew component and application logic
- `index.html` - HTML template
- `styles.css` - CSS styles and animations
- `svg/` - SVG templates for Christmas light shapes (editable)
  - `bulb.svg` - Traditional Christmas light bulb
  - `star.svg` - 5-pointed star
  - `candy-cane.svg` - Candy cane with stripes
  - `bell.svg` - Christmas bell
- `scripts/` - Build and development scripts
- `Cargo.toml` - Rust dependencies
- `Trunk.toml` - Build configuration

## Customizing Light Shapes

You can customize the Christmas light shapes by editing the SVG files in the `./svg` directory. Each SVG uses `COLOR` as a placeholder which gets replaced at runtime with the actual light color. See `svg/README.md` for details on editing and adding new shapes.

## Original Implementation

This is a Rust/WebAssembly implementation of the original JavaScript version created by Irfan Ezani.
