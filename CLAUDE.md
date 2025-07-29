# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Context: Faithful Archive (Dioxus)

This is **Faithful Archive**, a Dioxus-based web application for uploading and sharing Christ-honoring spiritual content on Arweave. The project aims to ensure sermons, worship resources, and Bible studies remain permanently accessible through decentralized storage.

### Key Project Goals
- **Decentralized spiritual archive** built on Arweave's permanent storage
- **Content moderation system** ensuring only Christ-honoring content is indexed
- **Lean, purpose-built platform** for pastors, worship leaders, and Bible teachers
- **Global accessibility** with censorship-resistant features

See PRD.md for full project details and roadmap.

## Essential Commands

### Setup & Development
```bash
# Install Dioxus CLI (if not already installed)
cargo install dioxus-cli

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
```

### Project Structure
```
faithful-archive/
├── Cargo.toml
├── src/
│   ├── main.rs              # Application entry point
│   ├── app.rs               # Main app component
│   ├── components/          # Reusable UI components
│   │   ├── mod.rs
│   │   ├── upload.rs        # File upload interface
│   │   ├── file_browser.rs  # Content browsing
│   │   ├── wallet.rs        # Wallet connection
│   │   └── navigation.rs    # App navigation
│   ├── services/            # Business logic and external integrations
│   │   ├── mod.rs
│   │   ├── arweave.rs       # Arweave blockchain integration
│   │   ├── wallet.rs        # Wallet authentication
│   │   ├── upload.rs        # File upload management
│   │   └── storage.rs       # Local storage/IndexedDB
│   ├── models/              # Data structures
│   │   ├── mod.rs
│   │   ├── file.rs          # File metadata models
│   │   ├── transaction.rs   # Arweave transaction models
│   │   └── metadata.rs      # Spiritual content metadata
│   └── utils/               # Utility functions
│       ├── mod.rs
│       ├── crypto.rs        # Encryption utilities
│       └── constants.rs     # App constants
├── assets/                  # Static assets
│   ├── css/
│   ├── images/
│   └── icons/
├── public/                  # Public web assets
│   ├── index.html
│   └── manifest.json
└── tests/                   # Test files
    ├── integration/
    └── unit/
```

## Architecture Overview

### Core Technology Stack
- **Frontend Framework**: Dioxus (Rust-based React-like framework)
- **Build Target**: WebAssembly (WASM) for web deployment
- **State Management**: Dioxus Signals for reactive state
- **Storage**: IndexedDB for local caching via rexie
- **Blockchain**: Arweave for permanent content storage
- **Wallet Integration**: ArConnect via WASM-JS bridge

### Key Architectural Components

1. **Dioxus Components** (`src/components/`): UI components using RSX syntax
   - Each component manages its own local state
   - Global state shared via Dioxus signals
   - Reactive updates on state changes

2. **Services** (`src/services/`): Business logic and external integrations
   - `ArweaveService`: Blockchain interactions and transaction management
   - `WalletService`: ArConnect integration via JS interop
   - `UploadService`: File upload orchestration with progress tracking
   - `StorageService`: Local storage and caching with IndexedDB

3. **Models** (`src/models/`): Data structures and type definitions
   - Spiritual content metadata schema
   - Arweave transaction types
   - User profile and authentication models

4. **WASM-JS Bridge**: Integration with browser APIs
   - ArConnect wallet extension interaction
   - File API for upload handling
   - IndexedDB for persistent storage

### Development Patterns

1. **Component Structure**:
   - Use functional components with hooks
   - Implement proper error boundaries
   - Follow RSX syntax for template rendering

2. **State Management**:
   - Local state with `use_signal()` for component-specific data
   - Global state with `GlobalSignal` for app-wide data
   - Reactive updates automatically trigger re-renders

3. **Async Operations**:
   - Use `use_future()` for async data fetching
   - Implement proper loading and error states
   - Handle WASM-JS interop with `wasm_bindgen`

4. **Error Handling**:
   - Use `Result<T, E>` types throughout
   - Implement user-friendly error messages
   - Log errors for debugging

### Git Commit Conventions
Prefix commits with:
- `feat:` new features
- `fix:` bug fixes
- `refactor:` code refactoring
- `style:` formatting changes
- `docs:` documentation changes
- `test:` adding tests
- `chore:` maintenance tasks
- `perf:` performance improvements

Use lowercase for commit messages.

### Testing Strategy
- Unit tests for business logic (services, models)
- Component tests for UI behavior
- Integration tests for critical user flows
- WASM-specific tests for browser integration
- Tests must pass before merging to main

### Deployment Flow
- `main` branch → production deployment
- Feature branches → PR preview builds
- Continuous integration with GitHub Actions
- WASM optimization for production builds

### Security Considerations
- Secure wallet integration with proper permission handling
- Content validation before Arweave submission
- Encrypted storage for sensitive data
- Input sanitization for user-generated content

### Performance Optimization
- WASM compilation for near-native performance
- Code splitting for optimal bundle sizes
- Efficient re-rendering with Dioxus signals
- Local caching strategies for content browsing

## Arweave Integration

### Initial Integration - bundles-rs

<p align="center">
  <a href="https://load.network">
    <img src="https://gateway.load.rs/bundle/0x83cf4417880af0d2df56ce04ecfc108ea4ee940e8fb81400e31ab81571e28d21/0">
  </a>
</p>

A Rust SDK for creating, signing, managing and posting [ANS-104 dataitems](https://github.com/ArweaveTeam/arweave-standards/blob/master/ans/ANS-104.md).

> Warning: this repository is actively under development and could have breaking changes until reaching full API compatibility in v1.0.0.

**Installation:**

Add to your `Cargo.toml`:

```toml
[dependencies]
# main library
bundles_rs = { git = "https://github.com/loadnetwork/bundles-rs", branch = "main" }

# use individual crates
ans104 = { git = "https://github.com/loadnetwork/bundles-rs", version = "0.1.0" } 
crypto = { git = "https://github.com/loadnetwork/bundles-rs", version = "0.1.0" }
```

**Quick start example:**

```rust
use bundles_rs::{
    ans104::{data_item::DataItem, tags::Tag},
    crypto::ethereum::EthereumSigner,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // create a signer
    let signer = EthereumSigner::random()?;
    
    // create tags (metadata)
    let tags = vec![
        Tag::new("Content-Type", "text/plain"),
        Tag::new("App-Name", "Faithful-Archive"),
    ];
    
    // create and sign a dataitem
    let data = b"Hello World Arweave!".to_vec();
    let item = DataItem::build_and_sign(&signer, None, None, tags, data)?;
    
    // get the dataitem id
    let id = item.arweave_id();
    println!("dataitem id: {}", id);
    
    // serialize for upload
    let bytes = item.to_bytes()?;
    println!("Ready to upload {} bytes", bytes.len());
    
    Ok(())
}
```

Full documentation: https://github.com/loadnetwork/bundles-rs

### Key Concepts
- **Permanent Storage**: All content uploaded to Arweave is immutable
- **Transaction Tags**: Metadata for content discovery and filtering
- **Gateways**: Access points for reading Arweave data
- **Wallet Management**: ArConnect for transaction signing

### Content Moderation Strategy
- **Pre-upload Review**: Content enters moderation queue before Arweave submission
- **Tagging System**: Only approved content gets indexed with spiritual content tags
- **Community Guidelines**: Clear policies for Christ-honoring content
- **Reviewer Dashboard**: Admin interface for content approval/rejection

### Spiritual Content Schema
```rust
#[derive(Serialize, Deserialize)]
pub struct SpiritualContent {
    pub title: String,
    pub description: String,
    pub content_type: ContentType, // Sermon, Worship, Study, etc.
    pub scripture_references: Vec<String>,
    pub language: String,
    pub church_or_ministry: Option<String>,
    pub speaker_or_author: Option<String>,
    pub series_name: Option<String>,
    pub date_created: DateTime<Utc>,
    pub tags: Vec<String>,
}
```

### Upload Process
1. User selects file and fills metadata form
2. Content enters local review queue
3. Moderator reviews and approves/rejects
4. Approved content uploaded to Arweave with proper tags
5. Transaction ID stored locally for quick access
6. Content becomes discoverable through search

This project leverages Rust's performance and safety features while providing a modern web interface for spiritual content archiving on the Arweave blockchain.

## Project Management Guidelines

### Project Synchronization
- Always keep @ROADMAP.md and GH issues in step/sync with one another

## Common Issues & Solutions

### dx serve Problems
- **`dx serve` fails with wasm-bindgen file not found**: Simplify `Dioxus.toml` to minimal config, clean build with `cargo clean && rm -rf target`
- **No CSS styling in browser**: Use `document::Stylesheet { href: asset!("/assets/tailwind.css") }` with proper `assets/tailwind.css` file
- **WASM compilation errors in browser**: Set `opt-level = 0` and `debug = true` in `[profile.wasm-dev]`
- **Build path mismatches**: Remove complex `[web.resource]` and `[bundle]` sections from `Dioxus.toml`

### Tailwind CSS Integration
- **External CDN CSS not loading**: Create local `assets/tailwind.css` with required utility classes
- **Classes not applying**: Use `asset!` macro instead of direct href paths
- **Build fails with CSS**: Follow Dioxus docs pattern with `input.css` and generated output

## Repository Management Guidelines

### Workflow Memory
- **Keep '/Users/dylanshade/Developer/faithful-archive-dioxus/faithful-archive/NEXT_UP.md' updated with the next issues/points on the roadmap to work on next. It should not list anything that cannot be worked on RIGHT NOW. Only list a single item if everything else depends on that single item, or if multiple items can be done in parallel right now, do so**

## Development Workflow Memories

### Tailwind CSS Setup
- We must run tailwind server to use it alongside dx serve: `npx @tailwindcss/cli -i ./input.css -o ./assets/tailwind.css --watch`