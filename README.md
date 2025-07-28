# Faithful Archive

![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)
![WebAssembly](https://img.shields.io/badge/WebAssembly-654FF0?style=for-the-badge&logo=WebAssembly&logoColor=white)
[![Dioxus](https://img.shields.io/badge/Dioxus-0.6-blue?style=for-the-badge)](https://dioxuslabs.com/)
[![contributions welcome](https://img.shields.io/badge/contributions-welcome-brightgreen.svg?style=flat)](https://github.com/dpshade/faithful-archive-dioxus/issues)

Faithful Archive is a modern, high-performance web application for uploading and sharing Christ-honoring spiritual content on Arweave's permanent storage network. Built with Dioxus and compiled to WebAssembly, it ensures sermons, worship resources, and Bible studies remain permanently accessible while providing a fast, secure user experience.

## 🚀 Features

- **⚡ High Performance**: Built with Rust and compiled to WebAssembly for near-native speed
- **🔗 Blockchain Integration**: Permanent content storage on Arweave network
- **🛡️ Secure**: Memory-safe Rust code prevents common web vulnerabilities
- **📱 Progressive Web App**: Offline-capable with modern web standards
- **🎨 Modern UI**: Responsive design built with Dioxus components
- **🔐 Wallet Integration**: Seamless ArConnect wallet connectivity
- **📊 Content Moderation**: Built-in review system for quality control
- **🔍 Smart Search**: Efficient content discovery with spiritual metadata

## 🛠️ Technology Stack

- **Frontend**: [Dioxus](https://dioxuslabs.com/) (Rust-based React-like framework)
- **Build Target**: WebAssembly (WASM) for web deployment
- **State Management**: Dioxus Signals for reactive state
- **Storage**: IndexedDB for local caching and offline support
- **Blockchain**: [Arweave](https://arweave.org/) for permanent content storage
- **Wallet**: [ArConnect](https://arconnect.io/) for transaction signing

## 📋 Prerequisites

Before you begin, ensure you have the following installed:

- [Rust](https://rustup.rs/) (latest stable version)
- [Dioxus CLI](https://dioxuslabs.com/learn/0.6/getting_started) (`cargo install dioxus-cli`)
- A modern web browser with WASM support
- [ArConnect wallet extension](https://arconnect.io/) for blockchain interactions

## 🚀 Quick Start

### 1. Clone the repository

```bash
git clone https://github.com/dpshade/faithful-archive-dioxus.git
cd faithful-archive-dioxus/faithful-archive
```

### 2. Install dependencies

```bash
cargo fetch
```

### 3. Start development server

```bash
dx serve
```

The application will be available at `http://localhost:8080` with hot reload enabled.

### 4. Build for production

```bash
dx build --release
```

Production files will be generated in the `dist/` directory.

## 🏗️ Project Structure

```
faithful-archive/
├── 📁 src/
│   ├── 🦀 main.rs              # Application entry point
│   ├── 🦀 app.rs               # Main app component
│   ├── 📁 components/          # Reusable UI components
│   │   ├── 🦀 upload.rs        # File upload interface
│   │   ├── 🦀 file_browser.rs  # Content browsing
│   │   ├── 🦀 wallet.rs        # Wallet connection
│   │   └── 🦀 navigation.rs    # App navigation
│   ├── 📁 services/            # Business logic and external integrations
│   │   ├── 🦀 arweave.rs       # Arweave blockchain integration
│   │   ├── 🦀 wallet.rs        # Wallet authentication
│   │   ├── 🦀 upload.rs        # File upload management
│   │   └── 🦀 storage.rs       # Local storage/IndexedDB
│   ├── 📁 models/              # Data structures
│   │   ├── 🦀 file.rs          # File metadata models
│   │   ├── 🦀 transaction.rs   # Arweave transaction models
│   │   └── 🦀 metadata.rs      # Spiritual content metadata
│   └── 📁 utils/               # Utility functions
├── 📁 assets/                  # Static assets (CSS, images, icons)
├── 📁 public/                  # Public web assets
├── 📁 tests/                   # Test files
├── 🔧 Cargo.toml              # Rust dependencies and configuration
├── 🔧 Dioxus.toml             # Dioxus configuration
└── 📚 README.md               # This file
```

## 🔧 Development

### Available Commands

```bash
# Start development server with hot reload
dx serve

# Build for production
dx build --release

# Run tests
cargo test

# Format code
cargo fmt

# Lint code
cargo clippy

# Check for security vulnerabilities
cargo audit
```

### Environment Configuration

The application supports different build configurations:

- **Development**: Debug builds with hot reload (`dx serve`)
- **Production**: Optimized WASM builds (`dx build --release`)

### Code Style

This project follows Rust community standards:

- Use `rustfmt` for code formatting
- Follow `clippy` recommendations for best practices
- Write comprehensive tests for business logic
- Document public APIs with rustdoc comments

## 🧪 Testing

```bash
# Run all tests
cargo test

# Run tests with coverage
cargo test --coverage

# Run specific test module
cargo test services::arweave

# Run tests in watch mode (requires cargo-watch)
cargo watch -x test
```

## 🚀 Deployment

### Static Hosting (Recommended)

The application builds to static files that can be hosted on any CDN:

```bash
# Build for production
dx build --release

# Deploy to your preferred hosting service
# (Cloudflare Pages, Vercel, Netlify, GitHub Pages, etc.)
```

### Custom Gateway Configuration

Set a custom Arweave gateway by modifying browser localStorage:

```javascript
// Set custom gateway
localStorage.setItem('faithful-archive.arweaveGateway', '"https://my.custom.gateway"');

// Reset to default
localStorage.removeItem('faithful-archive.arweaveGateway');
```

## 🔐 Security

- **Memory Safety**: Rust prevents buffer overflows and memory leaks
- **Type Safety**: Compile-time error checking eliminates many runtime issues
- **Secure Wallet Integration**: Safe handling of cryptographic operations
- **Content Validation**: Input sanitization and validation for user uploads
- **No Server**: Client-side only application reduces attack surface

## 🤝 Contributing

We welcome contributions! Please see our [Contributing Guidelines](CONTRIBUTING.md) for details.

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Make your changes following our coding standards
4. Add tests for new functionality
5. Commit your changes (`git commit -m 'feat: add amazing feature'`)
6. Push to the branch (`git push origin feature/amazing-feature`)
7. Open a Pull Request

### Commit Convention

We use [Conventional Commits](https://conventionalcommits.org/):

- `feat:` new features
- `fix:` bug fixes
- `docs:` documentation changes
- `style:` formatting changes
- `refactor:` code refactoring
- `test:` adding tests
- `chore:` maintenance tasks

## 📖 Documentation

- [Project Requirements (PRD.md)](PRD.md) - Detailed project specifications
- [Development Guide (CLAUDE.md)](CLAUDE.md) - Development setup and guidelines
- [Roadmap (ROADMAP.md)](ROADMAP.md) - Project milestones and future plans
- [Dioxus Documentation](https://dioxuslabs.com/learn/0.6/) - Framework documentation
- [Arweave Documentation](https://docs.arweave.org/) - Blockchain integration guide

## 🗺️ Roadmap

See [ROADMAP.md](ROADMAP.md) for detailed development milestones and future features.

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- [Dioxus Labs](https://dioxuslabs.com/) for the excellent Rust web framework
- [Arweave](https://arweave.org/) for permanent, decentralized storage
- The Rust community for creating amazing tools and libraries
- Beta testers and contributors who help improve the platform

## 📞 Support

- 🐛 [Report Issues](https://github.com/dpshade/faithful-archive-dioxus/issues)
- 💬 [Discussions](https://github.com/dpshade/faithful-archive-dioxus/discussions)
- 📧 [Contact](mailto:contact@faithful-archive.org)

---

**Built with ❤️ and ⚡ Rust for the glory of God**