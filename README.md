# Yew WebAssembly Application

A simple Yew application displaying Yves Klein's Blue Monochrome artwork with MOMA-style presentation.

## Prerequisites

- Rust (nightly version)
- Trunk (installed via `cargo install trunk`)

## Quick Start

### Using Makefile (Recommended)

```bash
# Start development server
make serve

# Or start on a different port
make serve-dev

# Build for production
make build-prod

# See all available commands
make help
```

### Manual Commands

```bash
# Start development server
trunk serve --port 8080

# Build for production
trunk build --release

# Check code
cargo check
```

## Project Structure

```
├── src/
│   └── main.rs          # Main Yew application
├── index.html           # HTML template
├── blue.jpg             # Artwork image
├── Cargo.toml           # Rust dependencies
├── rust-toolchain      # Rust version override
├── Makefile            # Build automation
└── README.md           # This file
```

## Features

- **MOMA-style Design**: Clean, minimalist presentation
- **Responsive Layout**: Works on different screen sizes
- **WebAssembly**: Fast, native performance in the browser
- **Hot Reload**: Automatic rebuilds during development

## Development

The application displays:
- Yves Klein's Blue Monochrome artwork
- Artist name: "Yves Klein"
- Artwork title: "Blue Monochrome" (italicized)
- Year: "1961"

## Access

Once the server is running, open your browser to:
- `http://127.0.0.1:8080` (default)
- `http://127.0.0.1:8081` (alternative port)

**Security Note**: The server only listens on localhost (127.0.0.1) for security reasons.
