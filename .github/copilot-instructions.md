# Copilot Instructions for icebook

## Project Overview

Icebook is a **theme-agnostic** component storybook framework for [Iced](https://iced.rs) applications. It provides reusable storybook infrastructure that any Iced-based component library can use to document and showcase components. Compiles to WASM for web deployment.

## Architecture

### Theme Plugin System
- `ThemeProvider` trait - Consumers implement this to supply their theme type
- `SidebarTheme` trait - Provides colors and fonts for icebook's sidebar UI chrome
- `SidebarFont` - Font configuration (font family, shaping) for sidebar text
- `Brightness` enum - Dark/Light mode toggle

### Font System
- **Fallback Font**: icebook includes **Fira Sans Regular** as a built-in fallback font
- `FALLBACK_FONT` constant - The font bytes (automatically loaded)
- `FALLBACK_FONT_NAME` constant - Font family name ("Fira Sans")
- Consumers can provide additional fonts via `Settings.fonts`
- The fallback font ensures text renders in WASM even without system fonts

### Story System
- `Story<Theme>` trait - Individual component documentation
- `StoryRegistry` trait - Container for all stories with routing
- `StoryMeta` - Metadata for sidebar navigation

### Key Files
- `crates/icebook/src/lib.rs` - Public API exports, fallback font
- `crates/icebook/src/app.rs` - Main Storybook application shell
- `crates/icebook/src/story.rs` - Story and StoryRegistry traits
- `crates/icebook/src/theme.rs` - Theme abstraction layer, SidebarTheme
- `crates/icebook/src/sidebar.rs` - Sidebar navigation component

## Build & Run

```bash
# Native build
cargo build

# WASM build (requires wasm32 target)
cargo build --target wasm32-unknown-unknown

# Run clippy
cargo clippy --all-targets --all-features

# Format check
cargo fmt --all -- --check
```

## CI/CD Pipeline

### GitHub Actions Workflows

1. **CI** (`.github/workflows/ci.yml`)
   - Triggers on: push/PR to main
   - Jobs: format check, clippy lint, build (native + WASM), tests

2. **Release** (`.github/workflows/release.yml`)
   - Triggers on: push to main
   - Uses semantic-release for automatic versioning
   - Updates CHANGELOG.md, Cargo.toml versions
   - Creates GitHub releases with tags

3. **Publish** (`.github/workflows/publish.yml`)
   - Triggers on: new version tags (v*)
   - Publishes to crates.io
   - Requires `CARGO_REGISTRY_TOKEN` secret

### Commit Convention (Conventional Commits)
- `feat:` - New feature (minor version bump)
- `fix:` - Bug fix (patch version bump)
- `feat!:` or `BREAKING CHANGE:` - Breaking change (major version bump)
- `chore:`, `docs:`, `style:`, `refactor:`, `test:` - No version bump

### Required Secrets
- `CARGO_REGISTRY_TOKEN` - crates.io API token for publishing

## Dependencies

- **iced 0.14** - GUI framework with WASM support
- **once_cell** - Lazy static initialization
- **tracing** - Logging infrastructure

## Conventions

- All public types exported via `lib.rs` and `prelude` module
- Theme-related code isolated in `theme.rs`
- Stories are generic over theme type `Theme: ?Sized`
- WASM builds use `webgl` feature for iced
