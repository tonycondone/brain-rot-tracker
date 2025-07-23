# Contributing to Brain Rot Tracker

## Development Setup

### Prerequisites

- Rust 1.70+ with `cargo`
- Node.js 18+ with `npm`
- Platform-specific dependencies:
  - **Windows**: Visual Studio Build Tools
  - **macOS**: Xcode Command Line Tools
  - **Linux**: `build-essential`, `libgtk-3-dev`, `libwebkit2gtk-4.0-dev`

### One-Command Setup

```bash
npm run setup
```

This will:
1. Install Rust dependencies
2. Install Node.js dependencies
3. Set up the development database
4. Generate initial configuration files

### Development Workflow

1. **Start the daemon** (in one terminal):
   ```bash
   cd daemon
   cargo run
   ```

2. **Start the dashboard** (in another terminal):
   ```bash
   cd dashboard
   npm run dev
   ```

3. **Start the Tauri app** (in a third terminal):
   ```bash
   cd app
   npm run tauri dev
   ```

### Project Architecture

```
daemon/          # Rust workspace for system monitoring
├─ src/
│  ├─ main.rs           # Entry point
│  ├─ tracker/          # OS-specific usage tracking
│  ├─ database/         # SQLite operations
│  ├─ rules/            # YAML rule engine
│  └─ api/              # Local HTTP API server
│
app/             # Tauri desktop application
├─ src/                 # Rust backend for Tauri
├─ src-ui/              # React frontend
└─ tauri.conf.json      # Tauri configuration
│
dashboard/       # Standalone React dashboard
├─ src/
│  ├─ components/       # Reusable UI components
│  ├─ pages/            # Dashboard pages
│  ├─ hooks/            # Custom React hooks
│  └─ utils/            # Utility functions
```

### Code Style

- **Rust**: Use `rustfmt` and `clippy`
- **TypeScript**: Use Prettier and ESLint
- **Commits**: Follow conventional commits format

### Testing

```bash
# Run all tests
npm run test

# Rust tests
cd daemon && cargo test

# Frontend tests
cd dashboard && npm test
```

### Building

```bash
# Development build
npm run build:dev

# Production build
npm run build

# Platform-specific builds
npm run build:windows
npm run build:macos
npm run build:linux
```

### Debugging

- Enable debug logging: `RUST_LOG=debug`
- Use browser dev tools for dashboard debugging
- Tauri dev tools available in development mode

## Architecture Decisions

### Why Rust for the Daemon?

- **Performance**: Minimal system resource usage
- **Safety**: Memory safety without garbage collection
- **Cross-platform**: Single codebase for all desktop platforms
- **System integration**: Excellent OS API bindings

### Why Tauri over Electron?

- **Size**: ~10MB vs ~100MB+ for Electron apps
- **Performance**: Native performance with web UI flexibility
- **Security**: Sandboxed by default
- **Resource usage**: Lower memory and CPU footprint

### Why Local-First?

- **Privacy**: No data leaves the user's device
- **Performance**: No network latency
- **Reliability**: Works offline
- **Compliance**: GDPR-friendly by design

## Release Process

1. Update version numbers in `Cargo.toml` and `package.json`
2. Update `CHANGELOG.md`
3. Create release PR
4. Tag release: `git tag v1.0.0`
5. Push tag: `git push origin v1.0.0`
6. GitHub Actions will build and publish releases

## Getting Help

- Check existing [Issues](https://github.com/tonycondone/brain-rot-tracker/issues)
- Join discussions in [Discussions](https://github.com/tonycondone/brain-rot-tracker/discussions)
- Read the [documentation](./docs/)