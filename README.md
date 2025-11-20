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

## Building and Running

### Development Server

To run the development server with hot-reloading:
```bash
trunk serve
```

Then open your browser to `http://127.0.0.1:8080`

### Production Build

To create an optimized production build:
```bash
trunk build --release
```

The output will be in the `dist/` directory.

## Usage

- Click **On** to start the lights animation
- Click **Off** to stop the animation
- Enter a speed value (1-5) in the input field and click **RUN** to adjust animation speed
  - 1 = slowest
  - 5 = fastest

## Project Structure

- `src/main.rs` - Main Yew component and application logic
- `index.html` - HTML template
- `styles.css` - CSS styles and animations
- `Cargo.toml` - Rust dependencies
- `Trunk.toml` - Build configuration

## Original Implementation

This is a Rust/WebAssembly implementation of the original JavaScript version created by Irfan Ezani.
