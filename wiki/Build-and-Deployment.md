# Build and Deployment

This page covers the complete build and deployment process for xmas-rs, including toolchain setup, build workflows, and deployment strategies.

## Table of Contents

- [Prerequisites](#prerequisites)
- [Build System](#build-system)
- [Build Process](#build-process)
- [Development Workflow](#development-workflow)
- [Production Build](#production-build)
- [Deployment](#deployment)

---

## Prerequisites

### Required Tools

```mermaid
graph TD
    DEV[Developer Machine] --> RUST[Rust Toolchain]
    DEV --> TRUNK[Trunk Build Tool]

    RUST --> RUSTC[rustc compiler]
    RUST --> CARGO[cargo package manager]
    RUST --> RUSTUP[rustup toolchain manager]

    TRUNK --> WASM_BINDGEN[wasm-bindgen-cli]
    TRUNK --> BUNDLER[Asset bundler]
```

### Installation Steps

#### 1. Rust Toolchain

```bash
# Install Rust via rustup (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Verify installation
rustc --version
cargo --version
```

#### 2. WebAssembly Target

```bash
# Add wasm32-unknown-unknown target
rustup target add wasm32-unknown-unknown

# Verify target is installed
rustup target list | grep wasm32
```

#### 3. Trunk Build Tool

```bash
# Install trunk
cargo install --locked trunk

# Verify installation
trunk --version
```

### Installation Sequence

```mermaid
sequenceDiagram
    participant Dev as Developer
    participant Rustup as rustup
    participant Cargo as cargo
    participant System as System

    Dev->>Rustup: curl rustup.sh
    Rustup->>System: Install Rust toolchain
    Dev->>Rustup: rustup target add wasm32
    Rustup->>System: Install WASM target
    Dev->>Cargo: cargo install trunk
    Cargo->>System: Compile and install trunk
    System->>Dev: Ready for development
```

---

## Build System

### Build Tools Overview

```mermaid
graph TB
    subgraph Tools[Build Tools]
        CARGO[Cargo]
        TRUNK[Trunk]
        RUSTC[rustc]
        WASM_BIND[wasm-bindgen]
    end

    subgraph Config[Configuration Files]
        CARGO_TOML[Cargo.toml]
        TRUNK_TOML[Trunk.toml]
    end

    subgraph Source[Source Files]
        RUST_SRC[src/main.rs]
        HTML_SRC[index.html]
        CSS_SRC[styles.css]
    end

    CARGO_TOML --> CARGO
    TRUNK_TOML --> TRUNK
    CARGO --> RUSTC
    RUSTC --> WASM_BIND
    TRUNK --> CARGO
    RUST_SRC --> CARGO
    HTML_SRC --> TRUNK
    CSS_SRC --> TRUNK
```

### Cargo Configuration

**File**: `Cargo.toml`

```toml
[package]
name = "xmas-rs"
version = "0.1.0"
edition = "2021"

[dependencies]
yew = { version = "0.21", features = ["csr"] }
wasm-bindgen = "0.2"
web-sys = { version = "0.3", features = ["HtmlInputElement"] }
gloo-timers = "0.3"
```

**Key Points**:
- **Edition 2021**: Uses latest Rust language features
- **Yew with CSR**: Client-side rendering feature enabled
- **web-sys features**: Only includes `HtmlInputElement` (minimal bloat)

### Trunk Configuration

**File**: `Trunk.toml`

```toml
[[hooks]]
stage = "build"
command = "sh"
command_arguments = ["-c", "echo 'Building Yew Christmas Lights app...'"]

[build]
target = "index.html"

[watch]
ignore = ["dist"]

[serve]
port = 8080
address = "127.0.0.1"
```

**Configuration Details**:
- **Build target**: `index.html` (entry point)
- **Watch ignore**: Prevents infinite rebuild loops
- **Serve settings**: Local development server on port 8080

---

## Build Process

### Compilation Pipeline

```mermaid
flowchart TD
    START[Start Build] --> CARGO_CHECK[Cargo: Check Dependencies]
    CARGO_CHECK --> RESOLVE[Resolve and Download Crates]
    RESOLVE --> RUSTC[rustc: Compile Rust to WASM]
    RUSTC --> OPT[WASM Optimization]
    OPT --> WASM_BINDGEN[wasm-bindgen: Generate JS Glue]
    WASM_BINDGEN --> TRUNK_BUNDLE[Trunk: Bundle Assets]
    TRUNK_BUNDLE --> COPY_HTML[Copy and Process HTML]
    TRUNK_BUNDLE --> COPY_CSS[Copy CSS]
    COPY_HTML --> DIST[Create dist/ Directory]
    COPY_CSS --> DIST
    WASM_BINDGEN --> DIST
    DIST --> END[Build Complete]
```

### Detailed Build Sequence

```mermaid
sequenceDiagram
    participant Dev as Developer
    participant Trunk as Trunk
    participant Cargo as Cargo
    participant Rustc as rustc
    participant WASM as wasm-bindgen
    participant FS as File System

    Dev->>Trunk: trunk serve/build
    Trunk->>Cargo: cargo build --target wasm32
    Cargo->>Rustc: Compile main.rs
    Rustc->>Rustc: Type checking
    Rustc->>Rustc: Borrow checking
    Rustc->>Rustc: Code generation
    Rustc->>Cargo: xmas_rs.wasm
    Cargo->>WASM: Process WASM binary
    WASM->>WASM: Generate JavaScript bindings
    WASM->>WASM: Generate TypeScript definitions
    WASM->>Trunk: Return processed assets
    Trunk->>Trunk: Bundle HTML
    Trunk->>Trunk: Bundle CSS
    Trunk->>Trunk: Inject script tags
    Trunk->>FS: Write to dist/
    FS->>Dev: Build artifacts ready
```

### Build Artifacts

```mermaid
graph TB
    subgraph Output[dist/ Directory]
        HTML[index.html]
        WASM_FILE[xmas_rs_bg.wasm]
        JS_FILE[xmas_rs.js]
        CSS_FILE[styles.css]
        SNIPPETS[snippets/ directory]
    end

    subgraph Assets[Asset Types]
        ENTRY[HTML Entry Point]
        BINARY[WASM Binary]
        GLUE[JavaScript Glue Code]
        STYLES[CSS Styles]
        MODULES[WASM Snippets]
    end

    HTML --> ENTRY
    WASM_FILE --> BINARY
    JS_FILE --> GLUE
    CSS_FILE --> STYLES
    SNIPPETS --> MODULES
```

---

## Development Workflow

### Development Server

```bash
trunk serve
```

**Process**:

```mermaid
sequenceDiagram
    participant Dev as Developer
    participant Trunk as Trunk Server
    participant Browser as Browser
    participant WS as WebSocket

    Dev->>Trunk: trunk serve
    Trunk->>Trunk: Initial build
    Trunk->>WS: Start WebSocket server
    Trunk->>Trunk: Start HTTP server
    Trunk->>Dev: Server ready at :8080
    Dev->>Browser: Open localhost:8080
    Browser->>Trunk: Request index.html
    Trunk->>Browser: Serve index.html + assets
    Browser->>WS: Connect WebSocket
    loop File Watching
        Trunk->>Trunk: Detect file change
        Trunk->>Trunk: Rebuild project
        Trunk->>WS: Send reload signal
        WS->>Browser: Trigger page reload
        Browser->>Trunk: Request updated assets
        Trunk->>Browser: Serve new assets
    end
```

### Hot Reload Mechanism

```mermaid
flowchart LR
    FILE_CHANGE[File Changed] --> DETECT[Trunk Detects Change]
    DETECT --> BUILD[Incremental Build]
    BUILD --> NOTIFY[WebSocket Notification]
    NOTIFY --> RELOAD[Browser Auto-reload]
    RELOAD --> UPDATED[Updated App Displayed]
```

### Development Cycle

```mermaid
stateDiagram-v2
    [*] --> Editing
    Editing --> FileSaved: Save file
    FileSaved --> Building: Trunk detects change
    Building --> Success: Build succeeds
    Building --> Failed: Build fails
    Success --> Reloading: Send WebSocket signal
    Reloading --> Running: Browser reloads
    Failed --> ShowError: Display error in terminal
    ShowError --> Editing: Fix error
    Running --> Editing: Continue development
```

---

## Production Build

### Release Build Command

```bash
trunk build --release
```

### Release Build Optimizations

```mermaid
graph TD
    SOURCE[Source Code] --> RUSTC_OPT[rustc with optimizations]
    RUSTC_OPT --> OPT_LEVEL[Optimization level: 's' or 'z']
    OPT_LEVEL --> LTO[Link-Time Optimization]
    LTO --> CODEGEN[Code Generation]
    CODEGEN --> WASM_OPT[wasm-opt]
    WASM_OPT --> SIZE_OPT[Size optimization]
    SIZE_OPT --> STRIP[Strip debug info]
    STRIP --> FINAL[Optimized WASM]
```

### Release Build Sequence

```mermaid
sequenceDiagram
    participant Dev as Developer
    participant Trunk as Trunk
    participant Cargo as Cargo
    participant Optimizer as Optimizer
    participant Dist as dist/

    Dev->>Trunk: trunk build --release
    Trunk->>Cargo: cargo build --release
    Cargo->>Cargo: Set opt-level='z'
    Cargo->>Cargo: Enable LTO
    Cargo->>Cargo: Compile with optimizations
    Cargo->>Optimizer: Pass WASM to optimizer
    Optimizer->>Optimizer: Run wasm-opt
    Optimizer->>Optimizer: Strip debug symbols
    Optimizer->>Optimizer: Minimize size
    Optimizer->>Trunk: Return optimized WASM
    Trunk->>Trunk: Bundle assets
    Trunk->>Trunk: Minify HTML/CSS
    Trunk->>Dist: Write production build
    Dist->>Dev: Build complete
```

### Build Size Comparison

```mermaid
graph LR
    subgraph Debug[Debug Build]
        DEBUG_WASM[~500KB WASM]
        DEBUG_SYMBOLS[+ Debug symbols]
        DEBUG_TOTAL[Total: ~2MB]
    end

    subgraph Release[Release Build]
        RELEASE_WASM[~150KB WASM]
        RELEASE_OPT[+ Optimizations]
        RELEASE_TOTAL[Total: ~200KB]
    end

    DEBUG_TOTAL -.->|Optimization| RELEASE_TOTAL
```

---

## Deployment

### Deployment Options

```mermaid
graph TB
    DIST[dist/ Directory] --> STATIC[Static Hosting]
    DIST --> CDN[CDN Distribution]
    DIST --> CONTAINER[Container Deployment]

    STATIC --> NETLIFY[Netlify]
    STATIC --> VERCEL[Vercel]
    STATIC --> PAGES[GitHub Pages]

    CDN --> CLOUDFLARE[Cloudflare]
    CDN --> AWS_CF[AWS CloudFront]

    CONTAINER --> DOCKER[Docker]
    CONTAINER --> K8S[Kubernetes]
```

### GitHub Pages Deployment

```mermaid
sequenceDiagram
    participant Dev as Developer
    participant Git as Git
    participant GHA as GitHub Actions
    participant Pages as GitHub Pages

    Dev->>Git: Commit changes
    Dev->>Git: Push to main
    Git->>GHA: Trigger workflow
    GHA->>GHA: Checkout code
    GHA->>GHA: Install Rust + Trunk
    GHA->>GHA: trunk build --release
    GHA->>GHA: Upload artifacts
    GHA->>Pages: Deploy to gh-pages branch
    Pages->>Pages: Serve static files
    Pages->>Dev: Site live at username.github.io
```

### Static File Server Deployment

```mermaid
flowchart TD
    BUILD[Run: trunk build --release] --> VERIFY[Verify dist/ contents]
    VERIFY --> UPLOAD[Upload dist/ to server]
    UPLOAD --> NGINX[Configure web server]
    NGINX --> HEADERS[Set proper headers]
    HEADERS --> MIME[Configure MIME types]
    MIME --> LIVE[Site is live]

    subgraph Headers
        WASM_MIME[application/wasm for .wasm]
        CACHE[Cache-Control headers]
        CORS[CORS if needed]
    end
```

### Docker Deployment

**Dockerfile** example:

```dockerfile
FROM nginx:alpine
COPY dist/ /usr/share/nginx/html/
EXPOSE 80
```

**Deployment sequence**:

```mermaid
sequenceDiagram
    participant Dev as Developer
    participant Docker as Docker
    participant Registry as Container Registry
    participant Server as Production Server

    Dev->>Docker: docker build -t xmas-rs .
    Docker->>Docker: Create image with dist/
    Dev->>Docker: docker tag xmas-rs
    Docker->>Registry: docker push
    Registry->>Server: Pull image
    Server->>Server: docker run -p 80:80
    Server->>Dev: Application live
```

---

## Continuous Integration

### CI Pipeline

```mermaid
flowchart TD
    PUSH[Git Push] --> CI[CI Trigger]
    CI --> CHECKOUT[Checkout Code]
    CHECKOUT --> SETUP[Setup Rust + WASM]
    SETUP --> CACHE[Restore Cache]
    CACHE --> CHECK[cargo check]
    CHECK --> TEST[cargo test]
    TEST --> LINT[cargo clippy]
    LINT --> FMT[cargo fmt --check]
    FMT --> BUILD[trunk build --release]
    BUILD --> ARTIFACT[Store Artifacts]
    ARTIFACT --> DEPLOY{Deploy?}
    DEPLOY -->|main branch| PROD[Deploy to Production]
    DEPLOY -->|other| END[End]
    PROD --> END
```

### CI/CD Workflow

```mermaid
sequenceDiagram
    participant Dev as Developer
    participant Git as Git
    participant CI as CI System
    participant Tests as Test Suite
    participant Deploy as Deployment

    Dev->>Git: git push
    Git->>CI: Webhook trigger
    CI->>CI: Setup environment
    CI->>CI: Install dependencies
    CI->>CI: Build project
    CI->>Tests: Run tests
    Tests->>CI: Tests pass
    CI->>CI: Run linters
    CI->>Deploy: Trigger deployment
    Deploy->>Deploy: Deploy to staging
    Deploy->>Deploy: Run smoke tests
    Deploy->>Deploy: Deploy to production
    Deploy->>Dev: Deployment complete
```

---

## Troubleshooting

### Common Build Issues

```mermaid
graph TD
    ERROR[Build Error] --> TYPE{Error Type}

    TYPE -->|Dependency| DEP_ERR[Dependency Resolution Error]
    TYPE -->|Compilation| COMPILE_ERR[Compilation Error]
    TYPE -->|WASM| WASM_ERR[WASM Generation Error]
    TYPE -->|Trunk| TRUNK_ERR[Trunk Error]

    DEP_ERR --> CLEAN1[cargo clean]
    COMPILE_ERR --> FIX_CODE[Fix Rust code]
    WASM_ERR --> CHECK_TARGET[Verify WASM target installed]
    TRUNK_ERR --> CHECK_TRUNK[Verify Trunk version]

    CLEAN1 --> REBUILD[Rebuild]
    FIX_CODE --> REBUILD
    CHECK_TARGET --> REBUILD
    CHECK_TRUNK --> REBUILD
```

### Debug vs Release Differences

| Aspect | Debug Build | Release Build |
|--------|-------------|---------------|
| Optimization | None (fast compile) | Maximum (slow compile) |
| Size | ~2MB | ~200KB |
| Debug info | Included | Stripped |
| Source maps | Generated | Optional |
| Performance | Slower runtime | Faster runtime |
| Build time | ~10s | ~60s |

---

## Performance Metrics

### Build Time Breakdown

```mermaid
gantt
    title Build Process Timeline
    dateFormat X
    axisFormat %S

    section Cargo
    Dependency resolution: 0, 5s
    Compilation: 5s, 20s

    section WASM
    Code generation: 20s, 25s
    wasm-bindgen: 25s, 30s

    section Trunk
    Asset bundling: 30s, 32s
    File copying: 32s, 33s
```

### Resource Usage

```mermaid
graph TB
    subgraph BuildResources[Build Resources]
        CPU[CPU: 4 cores, 100% utilization]
        RAM[RAM: ~2GB peak]
        DISK[Disk: ~500MB cache]
        TIME[Time: ~30-60s release build]
    end

    subgraph RuntimeResources[Runtime Resources]
        WASM_SIZE[WASM: ~150KB]
        JS_SIZE[JS: ~50KB]
        CSS_SIZE[CSS: ~2KB]
        TOTAL[Total: ~200KB]
    end
```

---

**Related Pages**:
- [[Architecture]] - System architecture overview
- [[Component-Details]] - Component implementation details
- [[Home]] - Return to wiki home
