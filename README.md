# Christmas Light Chain - Rust/Yew Implementation

A Rust implementation of a customizable Christmas light chain animation using the Yew framework. Build your own festive light display with 20 unique SVG shapes, multiple color modes, and various animation effects.

**[🎄 Live Demo](https://softwarewrighter.github.io/xmas-rs/?ts=1766708169314)**

## Screenshot

![Christmas Lights Demo](./images/screenshot.png?ts=1766708169314)

## Features

### View Modes
- **Settings Mode** - Configure all light options and preview your chain
- **Demo Mode** - Display a custom Christmas message framed by 40 animated lights (15 top/bottom, 5 on each side)
- **Details Mode** - View all available shapes and their names in a grid layout

### Light Chain Builder
- **20 unique Christmas shapes**: Stocking, Christmas Tree, Snowman, Star, Reindeer, Snowflake, Light Bulb, Icicle, Candy Cane, Ornament, Snowman 2, Santa's Sleigh, Wreath, Bells, Elf, Gift, Gift 2, Gnome, Mistletoe, and Ice Skate
- **Chain selection modes**:
  - 1 Image (Repeat) - Single image repeated 8 times
  - 2 Images (Alternate) - Two images alternating
  - 3 Images (Cycle) - Three images cycling
  - 4 Images (Cycle) - Four images cycling
  - 8 Images (Manual) - Select each position manually
  - 8 Images (Random) - Random selection with Randomize button

### Color Options
- **8 festive colors**: Red, Green, White, Yellow, Blue, Purple, Orange, Pink
- **5 color modes**:
  - Rainbow - Cycles through all colors
  - Per Image - Each position gets its own color
  - All Same - Override all to single color
  - Alternating - Even/odd positions alternate between two colors
  - Match Repeats - Same shapes get the same color

### Animation Modes
- **Solid On** - Always on, no animation
- **Blink** - Classic on/off blinking
- **Fade** - Smooth fade in/out
- **Sequence** - Lights turn on/off one by one
- **Wave** - Brightness wave travels through chain
- **Chase** - Chasing light pattern

### Technical Features
- On/Off controls
- Auto-applying speed control (1-5)
- Colored glow effects per light
- Built with Yew (Rust frontend framework)
- Compiles to WebAssembly
- Rust 2024 edition

## Prerequisites

- Rust (latest stable version with 2024 edition support)
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

1. **Select View Mode** - Use the dropdown in the header to switch between Settings and Demo modes
2. **Enter Demo Message** - In Settings mode, type your custom Christmas message
3. **Select Chain Mode** - Choose how many images to use and how they repeat
4. **Select Images** - Pick your favorite Christmas shapes for each position
5. **Choose Color Mode** - Select how colors are applied to the lights
6. **Pick Animation** - Choose the animation effect you prefer
7. **Adjust Speed** - Use the speed control (1-5) to change animation speed
8. **Toggle On/Off** - Start or stop the animation
9. **Switch to Demo** - View your message framed by the configured lights

## Color Scheme

The lights use an expanded festive color palette:
- **Red** (#ff0000) - Classic Christmas
- **Green** (#00cc00) - Traditional holiday
- **White** (#ffffff) - Snow/Winter
- **Yellow** (#ffdd00) - Star/Warm glow
- **Blue** (#0088ff) - Winter/Ice
- **Purple** (#aa00ff) - Royal accent
- **Orange** (#ff8800) - Warm accent
- **Pink** (#ff66cc) - Festive accent

## Project Structure

- `src/main.rs` - Main Yew component and application logic
- `index.html` - HTML template
- `styles.css` - CSS styles and animations
- `images/` - SVG Christmas shapes (20 images)
- `svg/` - Legacy SVG templates
- `scripts/` - Build and development scripts
- `Cargo.toml` - Rust dependencies
- `Trunk.toml` - Build configuration

## Original Implementation

This project started as a Rust/WebAssembly implementation of [this CodePen](https://codepen.io/irfanezani_/pen/mdeLpKo) by Irfan Ezani and has been significantly expanded with the light chain builder feature.
