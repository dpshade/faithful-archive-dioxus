# Wallet Integration Guide

This guide explains how to use the comprehensive wallet integration system built for Faithful Archive. The system is designed to be highly composable and reusable across different Dioxus applications.

## Features

- 🔌 **Multiple Wallet Support**: Beacon Wallet (AO Sync SDK), Wander, WalletKit, and Web Wallets
- 🧩 **Highly Composable**: Mix and match components based on your needs
- 🔄 **Auto-Reconnection**: Maintains wallet connections across page reloads
- 💾 **State Persistence**: Remembers user preferences and connection state
- 🎯 **Strategy Pattern**: Easy to add new wallet types
- 🪝 **Rich Hook System**: Comprehensive set of hooks for different use cases
- ⚡ **Async-First**: Built with modern async patterns
- 🎨 **Customizable UI**: Flexible styling and theming options
- 🛡️ **Error Handling**: Robust error recovery and user feedback
- 🔒 **Type Safe**: Full TypeScript-like safety with Rust

## Quick Start

### 1. Basic Setup

Add the wallet provider to your app root:

```rust
use dioxus::prelude::*;
use faithful_archive::services::wallet::{WalletProvider, WalletStrategyType};
use faithful_archive::components::WalletConnectCompact;

#[component]
fn App() -> Element {
    rsx! {
        WalletProvider {
            auto_reconnect: true,
            initial_strategy: Some(WalletStrategyType::Beacon),
            
            div {
                class: "p-4",
                h1 { "My Dioxus App" }
                WalletConnectCompact {}
            }
        }
    }
}
```

### 2. Using Wallet Context

Access wallet functionality from any component:

```rust
use dioxus::prelude::*;
use faithful_archive::services::wallet::{use_wallet_context, use_wallet_connection};

#[component]
fn MyComponent() -> Element {
    let wallet = use_wallet_context();
    let (connected, address) = use_wallet_connection();
    
    let connect_handler = move |_| {
        spawn(async move {
            match wallet.connect.call(()).await {
                Ok(addr) => log::info!("Connected: {}", addr),
                Err(e) => log::error!("Connection failed: {}", e),
            }
        });
    };
    
    rsx! {
        div {
            if connected {
                p { "Connected: {address.unwrap_or_default()}" }
            } else {
                button {
                    onclick: connect_handler,
                    "Connect Wallet"
                }
            }
        }
    }
}
```

## Component Library

### WalletConnect Components

#### `WalletConnect` - Main Component
The primary wallet connection component with full customization:

```rust
use faithful_archive::components::{WalletConnect, WalletConnectVariant, WalletConnectSize};

rsx! {
    WalletConnect {
        class: "my-custom-class",
        show_strategy_selector: true,
        show_status: true,
        show_address: true,
        size: WalletConnectSize::Medium,
        variant: WalletConnectVariant::Primary,
        connect_text: "Connect",
        disconnect_text: "Disconnect",
        on_connection_change: |event| {
            println!("Connection: {}", event.connected);
        },
        on_strategy_change: |strategy| {
            println!("Strategy: {}", strategy.display_name());
        }
    }
}
```

#### `WalletConnectCompact` - Minimal Button
Perfect for space-constrained layouts:

```rust
rsx! {
    WalletConnectCompact {
        class: "top-nav-wallet",
        variant: WalletConnectVariant::Outline
    }
}
```

#### `WalletConnectWithAddress` - With Address Display
Shows connected address:

```rust
rsx! {
    WalletConnectWithAddress {
        class: "sidebar-wallet",
        variant: WalletConnectVariant::Secondary
    }
}
```

#### `WalletConnectFull` - All Features
Complete wallet interface with all options:

```rust
rsx! {
    WalletConnectFull {
        on_connection_change: |event| {
            // Handle connection changes
        },
        on_strategy_change: |strategy| {
            // Handle strategy changes
        }
    }
}
```

### Utility Components

#### `WalletGated` - Conditional Content
Show content only when wallet is connected:

```rust
use faithful_archive::services::wallet::{WalletGated, WalletStrategyType};

rsx! {
    WalletGated {
        // Basic wallet gating
        div { "This content requires any wallet connection" }
    }
    
    WalletGated {
        // Strategy-specific gating
        require_specific_strategy: true,
        required_strategy: Some(WalletStrategyType::Beacon),
        
        div { "This content requires Beacon Wallet specifically" }
    }
}
```

#### `WalletErrorBoundary` - Error Handling
Catch and display wallet errors gracefully:

```rust
use faithful_archive::services::wallet::WalletErrorBoundary;

rsx! {
    WalletErrorBoundary {
        fallback: rsx! {
            div { "Custom error UI" }
        },
        
        // Your wallet-dependent components
        MyWalletComponent {}
    }
}
```

## Hook System

### Basic Hooks

#### `use_wallet_connection()`
Get basic connection state:

```rust
let (connected, address) = use_wallet_connection();
```

#### `use_wallet_context()`
Access full wallet context:

```rust
let wallet = use_wallet_context();
let state = wallet.state();
```

#### `use_wallet_operations()`
Get operation callbacks only:

```rust
let ops = use_wallet_operations();
spawn(async move {
    let _ = ops.connect.call(()).await;
});
```

### Advanced Hooks

#### `use_wallet_features()`
Check wallet capabilities:

```rust
let features = use_wallet_features();

rsx! {
    if features.can_sign {
        button { "Sign Transaction" }
    }
    if features.can_encrypt {
        button { "Encrypt Data" }
    }
}
```

#### `use_wallet_status()`
Comprehensive status information:

```rust
let status = use_wallet_status();

rsx! {
    div {
        "Strategy: {status.strategy_name}"
        "Address: {status.formatted_address.unwrap_or_default()}"
        "Connected: {status.connected}"
    }
}
```

#### `use_wallet_signing()`
Transaction signing with loading states:

```rust
let (sign_function, is_loading, last_error) = use_wallet_signing();

let sign_transaction = move |_| {
    let transaction_data = create_transaction_data();
    sign_function.call(transaction_data);
};

rsx! {
    button {
        disabled: *is_loading.read(),
        onclick: sign_transaction,
        if *is_loading.read() { "Signing..." } else { "Sign Transaction" }
    }
    
    if let Some(error) = last_error.read().as_ref() {
        div { "Error: {error}" }
    }
}
```

### Lifecycle Hooks

#### `use_wallet_reconnect()`
Automatic reconnection on mount:

```rust
use faithful_archive::services::wallet::use_wallet_reconnect;

#[component]
fn App() -> Element {
    use_wallet_reconnect(); // Automatically tries to reconnect
    
    rsx! {
        // Your app
    }
}
```

#### `use_wallet_persistence()`
Persist connection state:

```rust
use faithful_archive::services::wallet::use_wallet_persistence;

#[component]
fn App() -> Element {
    use_wallet_persistence(); // Saves/loads connection state
    
    rsx! {
        // Your app
    }
}
```

#### `use_wallet_events()`
Monitor wallet events:

```rust
use_wallet_events(
    Some(|address| log::info!("Connected: {}", address)),
    Some(|_| log::info!("Disconnected")),
    Some(|strategy| log::info!("Strategy: {}", strategy.display_name())),
    Some(|error| log::error!("Error: {}", error))
);
```

## Wallet Strategies

### Beacon Wallet (AO Sync SDK)
iOS-focused wallet with AO ecosystem integration:

```rust
WalletStrategyType::Beacon
```

**Features:**
- Mobile-first design
- AO-optimized operations
- QR code connection
- Auto-signing capabilities

### Wander Wallet
Desktop/web wallet for Arweave:

```rust
WalletStrategyType::Wander
```

**Features:**
- Browser extension
- Full Arweave support
- Multi-address support

### WalletKit
Development-focused wallet framework:

```rust
WalletStrategyType::WalletKit
```

### WebWallet
Generic web-based wallet interface:

```rust
WalletStrategyType::WebWallet
```

## Styling and Theming

### Component Variants

```rust
// Button styles
WalletConnectVariant::Primary   // Solid color
WalletConnectVariant::Secondary // Alternative solid color
WalletConnectVariant::Outline   // Border only
WalletConnectVariant::Ghost     // Minimal styling

// Sizes
WalletConnectSize::Small   // Compact
WalletConnectSize::Medium  // Standard
WalletConnectSize::Large   // Prominent
```

### Custom CSS Classes

All components accept custom CSS classes:

```rust
rsx! {
    WalletConnect {
        class: "my-custom-wallet-button bg-purple-600 hover:bg-purple-700",
        // ...
    }
}
```

### Strategy-Specific Styling

Get colors and icons for different strategies:

```rust
use faithful_archive::services::wallet::{get_strategy_colors, get_strategy_icon};

let colors = get_strategy_colors(WalletStrategyType::Beacon);
let icon = get_strategy_icon(WalletStrategyType::Beacon);

rsx! {
    div {
        style: "background-color: {colors.background}; color: {colors.text}",
        "{icon} {strategy.display_name()}"
    }
}
```

## Advanced Usage

### Custom Strategy Implementation

Add new wallet types by implementing the `WalletStrategy` trait:

```rust
use async_trait::async_trait;
use faithful_archive::services::wallet::{WalletStrategy, WalletStrategyType, WalletError};

pub struct MyCustomStrategy;

#[async_trait(?Send)]
impl WalletStrategy for MyCustomStrategy {
    fn strategy_type(&self) -> WalletStrategyType {
        WalletStrategyType::Custom("MyWallet".to_string())
    }
    
    async fn is_available(&self) -> Result<bool, WalletError> {
        // Check if your wallet is available
        Ok(true)
    }
    
    async fn connect(&mut self, permissions: Vec<&str>) -> Result<String, WalletError> {
        // Implement connection logic
        Ok("connected-address".to_string())
    }
    
    // Implement other required methods...
}
```

### Transaction Signing with Custom Data

```rust
use std::collections::HashMap;

let (sign_function, is_loading, last_error) = use_wallet_signing();

let sign_custom_transaction = move |_| {
    let mut transaction_data = HashMap::new();
    transaction_data.insert("to".to_string(), serde_json::Value::String(recipient));
    transaction_data.insert("quantity".to_string(), serde_json::Value::String(amount));
    transaction_data.insert("data".to_string(), serde_json::Value::String(data));
    
    // Add custom tags
    let mut tags = Vec::new();
    tags.push(("App-Name", "Faithful Archive"));
    tags.push(("Content-Type", "application/json"));
    transaction_data.insert("tags", serde_json::to_value(tags).unwrap());
    
    sign_function.call(transaction_data);
};
```

### Error Recovery

```rust
let (current_error, recover, is_recovering) = use_wallet_error_recovery();

rsx! {
    if let Some(error) = current_error.read().as_ref() {
        div {
            class: "error-banner",
            "Error: {error}"
            
            button {
                disabled: *is_recovering.read(),
                onclick: move |_| recover.call(()),
                if *is_recovering.read() { "Recovering..." } else { "Retry" }
            }
        }
    }
}
```

## Best Practices

### 1. Use the Provider at App Root

```rust
#[component]
fn App() -> Element {
    rsx! {
        WalletProvider {
            auto_reconnect: true,
            
            Router::<Route> {}
        }
    }
}
```

### 2. Enable Persistence and Reconnection

```rust
#[component]
fn AppContent() -> Element {
    use_wallet_reconnect();
    use_wallet_persistence();
    
    rsx! {
        // Your app content
    }
}
```

### 3. Use Specific Hooks for Specific Needs

```rust
// For simple connection status
let (connected, address) = use_wallet_connection();

// For wallet capabilities
let features = use_wallet_features();

// For comprehensive status
let status = use_wallet_status();
```

### 4. Handle Errors Gracefully

```rust
rsx! {
    WalletErrorBoundary {
        WalletGated {
            // Protected content
        }
    }
}
```

### 5. Provide Fallback UI

```rust
rsx! {
    WalletGated {
        fallback: rsx! {
            div {
                class: "wallet-prompt",
                "Please connect your wallet to continue"
                WalletConnectCompact {}
            }
        },
        
        // Gated content
    }
}
```

## Troubleshooting

### Common Issues

1. **"WalletClient not found"**
   - Ensure `beacon-wallet-loader.js` is loaded before your WASM
   - Check browser console for loading errors

2. **Connection timeouts**
   - Use `use_wallet_connect_with_timeout()` for custom timeout handling
   - Check network connectivity

3. **Strategy not available**
   - Verify wallet extensions are installed
   - Check wallet strategy implementation

4. **State not persisting**
   - Ensure `use_wallet_persistence()` is called
   - Check localStorage permissions

### Debug Mode

Enable detailed logging:

```rust
console_log::init_with_level(log::Level::Debug).unwrap();
```

### Testing

The system includes mock implementations for testing:

```rust
// Mock strategies are used when real wallets aren't available
// Check the console for mock wallet logs during development
```

## Migration Guide

### From Legacy Wallet Components

Replace old wallet buttons:

```rust
// Old
WalletButton {}

// New
WalletConnectCompact {}
```

### Adding to Existing Apps

1. Wrap your app with `WalletProvider`
2. Replace existing wallet components with new ones
3. Use hooks instead of direct service calls
4. Add error boundaries around wallet-dependent content

## Contributing

To add new wallet strategies:

1. Implement the `WalletStrategy` trait
2. Add the new strategy to `WalletStrategyType`
3. Register it in `WalletService::new()`
4. Add tests and documentation

For UI components:

1. Follow the existing component patterns
2. Use the hook system for state management
3. Ensure accessibility compliance
4. Add comprehensive documentation

## Examples

See `src/components/wallet_example.rs` for comprehensive examples of all features in action.

The example component can be used directly in your app:

```rust
use faithful_archive::components::WalletIntegrationExample;

rsx! {
    WalletIntegrationExample {}
}
```