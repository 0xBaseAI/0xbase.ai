# 0xbase.ai

A crypto-punk driven decentralized organization focused on building AI tools on Basechain. We have extreme requirements and security obsessions, requiring all code to be written in Rust and fully open source.

## 🎨 About This Project

This is a minimalist web application featuring Yves Klein's "Blue Monochrome" artwork. The site showcases our organization's projects and philosophy with a clean, modern design.

### Features

- **Responsive Design**: Optimized for all screen sizes with golden ratio proportions
- **Interactive Navigation**: Click the artwork or scroll down to access project information
- **Smooth Animations**: Subtle rainbow background animation and smooth scrolling
- **Modern UI**: Clean typography and minimalist layout
- **Project Showcase**: Display of our open-source projects with detailed descriptions

## 🚀 Quick Start

### Prerequisites

- Rust (nightly toolchain)
- Trunk (Rust WASM build tool)

### Installation

1. Clone the repository:
```bash
git clone https://github.com/0xBaseAI/0xbase.ai.git
cd 0xbase.ai
```

2. Install Trunk:
```bash
cargo install trunk
```

3. Serve the development server:
```bash
make serve-dev
```

4. Open your browser and navigate to `http://127.0.0.1:8081`

## 🛠️ Development

### Available Commands

- `make serve-dev` - Start development server on localhost:8081
- `make serve` - Start production server on localhost:8080
- `make build` - Build the project for production
- `make check` - Run cargo check
- `make clean` - Clean build artifacts

### Project Structure

```
0xbase.ai/
├── src/
│   └── main.rs          # Main Yew application
├── imgs/                # Static assets
│   ├── blue.jpg         # Yves Klein artwork
│   ├── logo.png         # 0xbase.ai logo
│   ├── favicon.png      # Site favicon
│   ├── rings.png        # Rings project logo
│   └── castorix.png     # Castorix project logo
├── index.html           # HTML entry point
├── Cargo.toml          # Rust dependencies
├── Trunk.toml          # Trunk configuration
├── rust-toolchain      # Rust toolchain specification
├── Makefile            # Build automation
└── README.md          # This file
```

## 🎯 Our Projects

### Rings
**P2P network with WebRTC & WASM**

A decentralized peer-to-peer networking library built with Rust, featuring WebRTC for real-time communication and WebAssembly for cross-platform compatibility. Designed for high-performance, low-latency applications.

[GitHub Repository](https://github.com/0xBaseAI/rings)

### Castorix
**Farcaster protocol library**

A comprehensive Rust implementation of the Farcaster protocol, providing secure and efficient tools for building decentralized social applications. Features include cryptographic signatures, message validation, and network synchronization.

[GitHub Repository](https://github.com/0xBaseAI/castorix)

## 🎨 Design Philosophy

This website embodies our core principles:

- **Minimalism**: Clean, uncluttered design focusing on essential elements
- **Aesthetic Excellence**: Inspired by MoMA's design language and Yves Klein's artistic vision
- **Technical Precision**: Built with Rust for performance and security
- **Open Source**: All code is publicly available and auditable

## 🔧 Technology Stack

- **Frontend**: Yew (Rust WebAssembly framework)
- **Build Tool**: Trunk
- **Styling**: CSS3 with modern features (Flexbox, Grid, CSS Variables)
- **Deployment**: Static site generation

## 📝 License

This project is licensed under a proprietary license that prohibits any form of usage. All rights reserved.

## 🤝 Contributing

We welcome contributions from the community. Please ensure all code follows our standards:

- Written in Rust
- Fully documented
- Security-focused
- Performance-optimized

## 📞 Contact

- **Organization**: 0xbase.ai
- **Focus**: Base, LLM, and Rust
- **Philosophy**: Crypto-punk driven, decentralized, security-obsessed

---

*Built with ❤️ and Rust by the 0xbase.ai team*