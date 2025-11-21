# Christmas Lights - xmas-rs

Welcome to the **xmas-rs** project wiki! This project is a Rust implementation of an animated Christmas lights display using the Yew framework and WebAssembly.

## Overview

xmas-rs is a web application that renders an animated Christmas lights display with interactive controls. The application features 8 colorful lights in 7 unique shapes that blink in sequence with customizable speed, mode selection, and on/off controls.

### Key Features

- **7 Unique Shapes**: Circle, Star, Candy Cane, Bell, Angel, Reindeer, and Tree
- **Mode Selector**: Choose between Mixed mode (one of each shape) or individual shape modes
- **Festive Colors**: Primary Red and Green with Yellow and White accents
- **Interactive Controls**: On/Off buttons to start/stop the animation
- **Auto-Applying Speed**: Adjustable animation speed from 1 (slowest) to 5 (fastest)
- **Staggered Animation**: Lights blink with delays for a cascading effect
- **Pure Rust**: Built entirely in Rust using the Yew framework with Rust 2024 edition features
- **WebAssembly**: Compiled to WASM for high-performance browser execution
- **SVG Graphics**: Inline SVG rendering with dynamic coloring and glow effects

## Project Origin

This project is a Rust implementation of [this CodePen](https://codepen.io/irfanezani_/pen/mdeLpKo) by Irfan Ezani, demonstrating how traditional JavaScript web applications can be reimagined using modern Rust web frameworks.

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
- **Graphics**: Inline SVG with dynamic coloring
- **Dependencies**:
  - `yew` - Reactive web framework
  - `wasm-bindgen` - JavaScript interop
  - `web-sys` - Web APIs bindings (HtmlInputElement, HtmlSelectElement)
  - `gloo-timers` - Timer utilities

## Project Statistics

- **Lines of Code**: ~350 lines of Rust
- **Components**: 2 components (App, Light)
- **Shapes**: 7 unique SVG light shapes
- **Modes**: 8 display modes (1 mixed + 7 individual)
- **Colors**: 4 festive colors
- **Dependencies**: 4 core dependencies
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

Please refer to the main repository for license information.

---

**Next Steps**: Explore the [[Architecture]] page to understand the system design and component interactions.
