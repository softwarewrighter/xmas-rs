# Christmas Lights - xmas-rs

Welcome to the **xmas-rs** project wiki! This project is a Rust implementation of an animated Christmas lights display using the Yew framework and WebAssembly.

## Overview

xmas-rs is a web application that renders an animated Christmas lights display with interactive controls. The application features 8 colorful lights that blink in sequence with customizable speed and on/off controls.

### Key Features

- **8 Animated Lights**: Red, Yellow, Blue, and Green lights arranged in a repeating pattern
- **Interactive Controls**: On/Off buttons to start/stop the animation
- **Variable Speed**: Adjustable animation speed from 1 (slowest) to 5 (fastest)
- **Pure Rust**: Built entirely in Rust using the Yew framework
- **WebAssembly**: Compiled to WASM for high-performance browser execution
- **CSS Animations**: Smooth blinking effects with glowing box shadows

## Project Origin

This project is a Rust implementation of [this CodePen](https://codepen.io/irfanezani_/pen/mdeLpKo) by Irfan Ezani, demonstrating how traditional JavaScript web applications can be reimagined using modern Rust web frameworks.

## Quick Start

```bash
# Install prerequisites
cargo install --locked trunk
rustup target add wasm32-unknown-unknown

# Run development server
trunk serve

# Open browser to http://127.0.0.1:8080
```

## Documentation Structure

This wiki is organized into the following sections:

- **[[Architecture]]** - System architecture and design diagrams
- **[[Component-Details]]** - Detailed breakdown of each component
- **[[Build-and-Deployment]]** - Build process, deployment, and workflows
- **[README](../README.md)** - Main project README with setup instructions

## Technology Stack

- **Language**: Rust (Edition 2021)
- **Web Framework**: Yew 0.21 (with CSR features)
- **Build Tool**: Trunk
- **Target**: WebAssembly (wasm32-unknown-unknown)
- **Styling**: CSS3 with animations
- **Dependencies**:
  - `yew` - Reactive web framework
  - `wasm-bindgen` - JavaScript interop
  - `web-sys` - Web APIs bindings
  - `gloo-timers` - Timer utilities

## Project Statistics

- **Lines of Code**: ~130 lines of Rust
- **Components**: 1 main component
- **Dependencies**: 4 core dependencies
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
