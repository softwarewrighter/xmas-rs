# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

A Rust/WebAssembly Christmas light chain animation built with the Yew framework. Features two view modes: Settings (configure lights) and Demo (display a framed message with 32 animated lights).

## Build Commands

```bash
# Development server with hot reload (port 7030)
./scripts/dev.sh

# Build only (output to dist/)
trunk build

# Release build for GitHub Pages deployment
trunk build --release --public-url /xmas-rs/

# Run clippy (must target wasm32)
cargo clippy --target wasm32-unknown-unknown

# Format code
cargo fmt

# Run tests
cargo test
```

## GitHub Pages Deployment

**IMPORTANT:** The site deploys from the `docs/` folder. You MUST rebuild and update `docs/` before committing any code changes, or the live demo will not reflect your changes.

```bash
# Full deployment workflow (run before every commit with code changes)
trunk build --release --public-url /xmas-rs/
rm -rf docs/* && cp -r dist/* docs/ && touch docs/.nojekyll
git add docs/
```

The `--public-url /xmas-rs/` flag is **required** for correct asset paths on GitHub Pages. Without it, assets will 404.

## Architecture

**Single-file Yew application** (`src/main.rs`):
- All UI components and state in one file (~1000 lines)
- Uses Yew's functional components with `use_state` hooks
- SVGs embedded at compile time via `include_str!()`

**Key enums define configuration:**
- `ViewMode`: Settings | Demo
- `LightColor`: 8 colors with RGB values and CSS class names
- `ImageShape`: 13 Christmas shapes with SVG content
- `SelectionMode`: How images repeat (Repeat1-4, Manual8, Random8)
- `ColorMode`: How colors apply (Rainbow, AllSame, Alternating, etc.)
- `AnimationMode`: Solid, Blink, Fade, Sequence, Wave, Chase

**Components:**
- `ChainLight`: Renders individual light with shape, color, animation, and rotation
- `App`: Main component managing all state and UI

**CSS animations** (`styles.css`):
- Animation keyframes for each mode (blink, fade, sequence, wave, chase)
- CSS variables `--anim-delay` and `--anim-duration` control timing
- Color-specific glow effects via `filter: drop-shadow()`

**Build-time injection** (`build.rs`):
- Injects `BUILD_GIT_SHA`, `BUILD_TIMESTAMP`, `BUILD_HOST` for footer display
