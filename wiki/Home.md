# Christmas Light Chain - xmas-rs

Welcome to the **xmas-rs** project wiki! This project is a Rust implementation of a customizable Christmas light chain builder using the Yew framework and WebAssembly.

## Overview

xmas-rs is a web application that lets you build and animate your own Christmas light chains with 13 unique SVG shapes, 8 colors, multiple animation modes, and various customization options.

### Key Features

- **13 Unique Shapes**: Stocking, Christmas Tree, Snowman, Star, Reindeer, Snowflake, Light Bulb, Icicle, Candy Cane, Ornament, Snowman 2, Santa's Sleigh, and Wreath
- **Chain Selection Modes**:
  - 1-4 images repeated to fill 8 positions
  - 8 images selected manually
  - 8 random images with Randomize button
- **8 Festive Colors**: Red, Green, White, Yellow, Blue, Purple, Orange, Pink
- **5 Color Modes**: Rainbow, Per Image, All Same, Alternating, Match Repeats
- **6 Animation Modes**: Solid On, Blink, Fade, Sequence, Wave, Chase
- **Interactive Controls**: On/Off buttons, speed control (1-5)
- **Pure Rust**: Built entirely in Rust using the Yew framework with Rust 2024 edition features
- **WebAssembly**: Compiled to WASM for high-performance browser execution
- **SVG Graphics**: Dynamic SVG coloring with glow effects

## Project Origin

This project started as a Rust implementation of [this CodePen](https://codepen.io/irfanezani_/pen/mdeLpKo) by Irfan Ezani and has been significantly expanded with the light chain builder feature.

## Quick Start

```bash
# Install prerequisites
cargo install --locked trunk
rustup target add wasm32-unknown-unknown

# Clone and navigate to project
cd xmas-rs

# Run development server using script
./scripts/dev.sh

# Open browser to http://localhost:7030
```

## Documentation Structure

This wiki is organized into the following sections:

- **[[Architecture]]** - System architecture and design diagrams
- **[[Component-Details]]** - Detailed breakdown of each component
- **[[Build-and-Deployment]]** - Build process, deployment, and workflows
- **[README](../README.md)** - Main project README with setup instructions

## Technology Stack

- **Language**: Rust (Edition 2024)
- **Web Framework**: Yew 0.21 (with CSR features)
- **Build Tool**: Trunk 0.21.14
- **Target**: WebAssembly (wasm32-unknown-unknown)
- **Port**: 7030 (configured for parallel development)
- **Styling**: CSS3 with animations, drop-shadow filters, and flexbox
- **Graphics**: Embedded SVG with dynamic coloring via `include_str!`
- **Dependencies**:
  - `yew` - Reactive web framework
  - `wasm-bindgen` - JavaScript interop
  - `web-sys` - Web APIs bindings (HtmlInputElement, HtmlSelectElement)
  - `gloo-timers` - Timer utilities

## Project Statistics

- **Lines of Code**: ~790 lines of Rust
- **Components**: 2 components (App, ChainLight)
- **Shapes**: 13 unique SVG Christmas shapes
- **Selection Modes**: 6 chain configuration modes
- **Color Modes**: 5 color application modes
- **Animation Modes**: 6 animation effects
- **Colors**: 8 festive colors
- **Build Scripts**: 3 shell scripts (build.sh, serve.sh, dev.sh)
- **Build Target**: WebAssembly
- **Browser Compatibility**: All modern browsers with WASM support

## Contributing

To contribute to this project:

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Submit a pull request

## License

MIT License - see the main repository for details.

---

**Next Steps**: Explore the [[Architecture]] page to understand the system design and component interactions.
