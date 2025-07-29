/*!
# Wallet Integration Examples

This file demonstrates various ways to use the wallet integration system
in your Dioxus applications. The wallet system is highly composable and
can be adapted to different use cases.

## Basic Usage

The simplest way to add wallet connectivity:

```rust
use dioxus::prelude::*;
use faithful_archive::components::WalletConnectCompact;

#[component]
fn SimpleApp() -> Element {
    rsx! {
        div {
            class: "p-4",
            h1 { "My App" }
            WalletConnectCompact {}
        }
    }
}
```

## Full Integration with Context

For applications that need comprehensive wallet functionality:

```rust
use dioxus::prelude::*;
use faithful_archive::services::wallet::{WalletProvider, use_wallet_context};
use faithful_archive::components::WalletConnectFull;

#[component]
fn App() -> Element {
    rsx! {
        WalletProvider {
            auto_reconnect: true,
            initial_strategy: Some(WalletStrategyType::Beacon),
            
            div {
                class: "min-h-screen bg-gray-50",
                WalletConnectFull {
                    class: "mb-8",
                    on_connection_change: |event| {
                        log::info!("Wallet connection: {}", event.connected);
                    },
                    on_strategy_change: |strategy| {
                        log::info!("Strategy changed: {}", strategy.display_name());
                    }
                }
                
                MainContent {}
            }
        }
    }
}
```
*/

use dioxus::prelude::*;
use std::collections::HashMap;
use crate::services::wallet::{
    WalletProvider, WalletGated, WalletErrorBoundary,
    use_wallet_context, use_wallet_connection, use_wallet_features,
    use_wallet_status, use_wallet_reconnect, use_wallet_persistence,
    use_wallet_signing, WalletStrategyType, get_strategy_icon
};
use crate::components::{
    WalletConnect, WalletConnectFull, WalletConnectCompact,
    WalletConnectSize, WalletConnectVariant, ConnectionChangeEvent
};

/// Complete wallet integration example
/// 
/// This component demonstrates a full-featured wallet integration with:
/// - Automatic reconnection
/// - State persistence
/// - Error handling
/// - Feature detection
/// - Transaction signing
#[component]
pub fn WalletIntegrationExample() -> Element {
    rsx! {
        WalletProvider {
            auto_reconnect: true,
            initial_strategy: Some(WalletStrategyType::Beacon),
            
            div {
                class: "max-w-4xl mx-auto p-6 space-y-8",
                
                h1 {
                    class: "text-3xl font-bold text-gray-900 dark:text-white mb-8",
                    "Wallet Integration Examples"
                }
                
                // Error boundary to catch wallet errors
                WalletErrorBoundary {
                    ExampleSections {}
                }
            }
        }
    }
}

#[component]
fn ExampleSections() -> Element {
    // Enable automatic reconnection and persistence
    use_wallet_reconnect();
    use_wallet_persistence();
    
    rsx! {
        div {
            class: "space-y-8",
            
            // Basic connection examples
            BasicConnectionExamples {}
            
            // Advanced features
            AdvancedFeaturesExample {}
            
            // Gated content example
            GatedContentExample {}
            
            // Transaction signing example
            TransactionSigningExample {}
            
            // Wallet status display
            WalletStatusExample {}
        }
    }
}

/// Basic wallet connection examples
#[component]
fn BasicConnectionExamples() -> Element {
    rsx! {
        section {
            class: "bg-white dark:bg-gray-800 rounded-lg p-6 shadow-sm",
            
            h2 {
                class: "text-xl font-semibold mb-4 text-gray-900 dark:text-white",
                "Basic Connection Examples"
            }
            
            div {
                class: "grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4",
                
                // Compact button
                div {
                    class: "p-4 border border-gray-200 dark:border-gray-700 rounded-lg",
                    h3 { class: "font-medium mb-2", "Compact Button" }
                    WalletConnectCompact {
                        variant: WalletConnectVariant::Primary
                    }
                }
                
                // Medium button with address
                div {
                    class: "p-4 border border-gray-200 dark:border-gray-700 rounded-lg",
                    h3 { class: "font-medium mb-2", "With Address Display" }
                    WalletConnect {
                        show_strategy_selector: false,
                        show_address: true,
                        size: WalletConnectSize::Medium,
                        variant: WalletConnectVariant::Outline
                    }
                }
                
                // Full featured
                div {
                    class: "p-4 border border-gray-200 dark:border-gray-700 rounded-lg col-span-full",
                    h3 { class: "font-medium mb-2", "Full Featured" }
                    WalletConnectFull {
                        on_connection_change: |event: ConnectionChangeEvent| {
                            if event.connected {
                                log::info!("Connected to: {:?}", event.address);
                            }
                        },
                        on_strategy_change: |strategy: WalletStrategyType| {
                            log::info!("Strategy changed to: {}", strategy.display_name());
                        }
                    }
                }
            }
        }
    }
}

/// Advanced wallet features demonstration
#[component]
fn AdvancedFeaturesExample() -> Element {
    let features = use_wallet_features();
    let status = use_wallet_status();
    
    rsx! {
        section {
            class: "bg-white dark:bg-gray-800 rounded-lg p-6 shadow-sm",
            
            h2 {
                class: "text-xl font-semibold mb-4 text-gray-900 dark:text-white",
                "Advanced Features"
            }
            
            div {
                class: "grid grid-cols-1 md:grid-cols-2 gap-6",
                
                // Feature detection
                div {
                    h3 { class: "font-medium mb-3", "Wallet Capabilities" }
                    
                    div {
                        class: "space-y-2",
                        
                        FeatureBadge {
                            label: "Transaction Signing",
                            enabled: features.can_sign,
                            icon: "✍️"
                        }
                        
                        FeatureBadge {
                            label: "Data Encryption",
                            enabled: features.can_encrypt,
                            icon: "🔒"
                        }
                        
                        FeatureBadge {
                            label: "Batch Operations",
                            enabled: features.supports_batch,
                            icon: "📦"
                        }
                        
                        FeatureBadge {
                            label: "Multiple Addresses",
                            enabled: features.supports_multiple_addresses,
                            icon: "🏠"
                        }
                    }
                }
                
                // Strategy information
                div {
                    h3 { class: "font-medium mb-3", "Current Strategy" }
                    
                    if status.connected {
                        div {
                            class: "flex items-center space-x-3 p-3 bg-green-50 dark:bg-green-900/20 rounded-lg border border-green-200 dark:border-green-800",
                            
                            span {
                                class: "text-2xl",
                                "{get_strategy_icon(status.strategy)}"
                            }
                            
                            div {
                                div {
                                    class: "font-medium text-green-800 dark:text-green-200",
                                    "{status.strategy_name}"
                                }
                                
                                div {
                                    class: "text-sm text-green-600 dark:text-green-400",
                                    "Connected"
                                }
                            }
                        }
                    } else {
                        div {
                            class: "p-3 bg-gray-50 dark:bg-gray-700 rounded-lg border border-gray-200 dark:border-gray-600 text-center",
                            
                            div {
                                class: "text-gray-500 dark:text-gray-400",
                                "No wallet connected"
                            }
                        }
                    }
                    
                    // Available strategies
                    if !status.available_strategies.is_empty() {
                        div {
                            class: "mt-3",
                            
                            div {
                                class: "text-sm font-medium text-gray-700 dark:text-gray-300 mb-2",
                                "Available Strategies:"
                            }
                            
                            div {
                                class: "flex flex-wrap gap-2",
                                
                                for strategy in status.available_strategies {
                                    span {
                                        class: "inline-flex items-center px-2 py-1 rounded-full text-xs bg-blue-100 text-blue-800 dark:bg-blue-900 dark:text-blue-200",
                                        
                                        span {
                                            class: "mr-1",
                                            "{get_strategy_icon(strategy)}"
                                        }
                                        
                                        "{strategy.display_name()}"
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Feature badge component
#[component]
fn FeatureBadge(label: String, enabled: bool, icon: String) -> Element {
    let badge_class = if enabled {
        "inline-flex items-center px-3 py-1 rounded-full text-sm bg-green-100 text-green-800 dark:bg-green-900 dark:text-green-200"
    } else {
        "inline-flex items-center px-3 py-1 rounded-full text-sm bg-gray-100 text-gray-500 dark:bg-gray-700 dark:text-gray-400"
    };
    
    rsx! {
        div {
            class: "flex items-center justify-between",
            
            span {
                class: "text-sm text-gray-700 dark:text-gray-300",
                "{label}"
            }
            
            span {
                class: badge_class,
                
                span {
                    class: "mr-1",
                    "{icon}"
                }
                
                if enabled { "Available" } else { "Not Available" }
            }
        }
    }
}

/// Gated content example
#[component]
fn GatedContentExample() -> Element {
    rsx! {
        section {
            class: "bg-white dark:bg-gray-800 rounded-lg p-6 shadow-sm",
            
            h2 {
                class: "text-xl font-semibold mb-4 text-gray-900 dark:text-white",
                "Wallet-Gated Content"
            }
            
            div {
                class: "space-y-4",
                
                // Basic gated content
                WalletGated {
                    div {
                        class: "p-4 bg-green-50 dark:bg-green-900/20 border border-green-200 dark:border-green-800 rounded-lg",
                        
                        h3 {
                            class: "font-medium text-green-800 dark:text-green-200 mb-2",
                            "🎉 Premium Content Unlocked!"
                        }
                        
                        p {
                            class: "text-green-700 dark:text-green-300",
                            "This content is only visible when a wallet is connected. You can upload files, sign transactions, and access all premium features."
                        }
                    }
                }
                
                // Strategy-specific gated content
                WalletGated {
                    require_specific_strategy: true,
                    required_strategy: Some(WalletStrategyType::Beacon),
                    
                    div {
                        class: "p-4 bg-blue-50 dark:bg-blue-900/20 border border-blue-200 dark:border-blue-800 rounded-lg",
                        
                        h3 {
                            class: "font-medium text-blue-800 dark:text-blue-200 mb-2",
                            "📱 Beacon-Only Feature"
                        }
                        
                        p {
                            class: "text-blue-700 dark:text-blue-300",
                            "This feature is specifically designed for Beacon Wallet users. Advanced AO operations and mobile-optimized signing flows are available here."
                        }
                    }
                }
            }
        }
    }
}

/// Transaction signing example
#[component]
fn TransactionSigningExample() -> Element {
    let (sign_function, is_loading, last_error) = use_wallet_signing();
    let features = use_wallet_features();
    
    let sign_demo_transaction = move |_| {
        let mut transaction_data = HashMap::new();
        transaction_data.insert("to".to_string(), serde_json::Value::String("demo-address".to_string()));
        transaction_data.insert("quantity".to_string(), serde_json::Value::String("1000000000000".to_string()));
        transaction_data.insert("data".to_string(), serde_json::Value::String("Hello from Faithful Archive!".to_string()));
        
        sign_function.call(transaction_data);
    };
    
    rsx! {
        section {
            class: "bg-white dark:bg-gray-800 rounded-lg p-6 shadow-sm",
            
            h2 {
                class: "text-xl font-semibold mb-4 text-gray-900 dark:text-white",
                "Transaction Signing"
            }
            
            if features.can_sign {
                div {
                    class: "space-y-4",
                    
                    p {
                        class: "text-gray-600 dark:text-gray-400",
                        "Click the button below to sign a demo transaction. This will open your wallet for confirmation."
                    }
                    
                    button {
                        class: "inline-flex items-center px-4 py-2 border border-transparent text-sm font-medium rounded-md text-white bg-indigo-600 hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500 disabled:opacity-50 disabled:cursor-not-allowed",
                        disabled: *is_loading.read(),
                        onclick: sign_demo_transaction,
                        
                        if *is_loading.read() {
                            svg {
                                class: "animate-spin -ml-1 mr-2 h-4 w-4",
                                fill: "none",
                                view_box: "0 0 24 24",
                                
                                circle {
                                    class: "opacity-25",
                                    cx: "12",
                                    cy: "12",
                                    r: "10",
                                    stroke: "currentColor",
                                    stroke_width: "4"
                                }
                                
                                path {
                                    class: "opacity-75",
                                    fill: "currentColor",
                                    d: "M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"
                                }
                            }
                            "Signing..."
                        } else {
                            "Sign Demo Transaction"
                        }
                    }
                    
                    if let Some(error) = last_error.read().as_ref() {
                        div {
                            class: "p-3 bg-red-50 dark:bg-red-900/20 border border-red-200 dark:border-red-800 rounded-lg",
                            
                            div {
                                class: "flex items-start",
                                
                                svg {
                                    class: "w-5 h-5 text-red-400 mt-0.5 mr-3 flex-shrink-0",
                                    fill: "currentColor",
                                    view_box: "0 0 20 20",
                                    
                                    path {
                                        fill_rule: "evenodd",
                                        d: "M18 10a8 8 0 11-16 0 8 8 0 0116 0zm-7 4a1 1 0 11-2 0 1 1 0 012 0zm-1-9a1 1 0 00-1 1v4a1 1 0 102 0V6a1 1 0 00-1-1z",
                                        clip_rule: "evenodd"
                                    }
                                }
                                
                                div {
                                    h3 {
                                        class: "text-sm font-medium text-red-800 dark:text-red-200",
                                        "Signing Failed"
                                    }
                                    
                                    p {
                                        class: "mt-1 text-sm text-red-700 dark:text-red-300",
                                        "{error}"
                                    }
                                }
                            }
                        }
                    }
                }
            } else {
                div {
                    class: "p-4 bg-yellow-50 dark:bg-yellow-900/20 border border-yellow-200 dark:border-yellow-800 rounded-lg text-center",
                    
                    p {
                        class: "text-yellow-800 dark:text-yellow-200",
                        "⚠️ Transaction signing is not available with the current wallet strategy. Please connect a compatible wallet."
                    }
                }
            }
        }
    }
}

/// Wallet status display example
#[component]
fn WalletStatusExample() -> Element {
    let status = use_wallet_status();
    let (connected, address) = use_wallet_connection();
    
    rsx! {
        section {
            class: "bg-white dark:bg-gray-800 rounded-lg p-6 shadow-sm",
            
            h2 {
                class: "text-xl font-semibold mb-4 text-gray-900 dark:text-white",
                "Wallet Status"
            }
            
            div {
                class: "space-y-3",
                
                // Connection status
                div {
                    class: "flex items-center justify-between p-3 bg-gray-50 dark:bg-gray-700 rounded-lg",
                    
                    span {
                        class: "text-sm font-medium text-gray-700 dark:text-gray-300",
                        "Connection Status"
                    }
                    
                    span {
                        class: if connected {
                            "inline-flex items-center px-2 py-1 rounded-full text-xs bg-green-100 text-green-800 dark:bg-green-900 dark:text-green-200"
                        } else {
                            "inline-flex items-center px-2 py-1 rounded-full text-xs bg-red-100 text-red-800 dark:bg-red-900 dark:text-red-200"
                        },
                        
                        span {
                            class: if connected { "w-2 h-2 bg-green-500 rounded-full mr-1" } else { "w-2 h-2 bg-red-500 rounded-full mr-1" }
                        }
                        
                        if connected { "Connected" } else { "Disconnected" }
                    }
                }
                
                // Address display
                if let Some(addr) = address {
                    div {
                        class: "flex items-center justify-between p-3 bg-gray-50 dark:bg-gray-700 rounded-lg",
                        
                        span {
                            class: "text-sm font-medium text-gray-700 dark:text-gray-300",
                            "Address"
                        }
                        
                        code {
                            class: "text-xs bg-gray-200 dark:bg-gray-600 px-2 py-1 rounded font-mono",
                            "{status.formatted_address.as_deref().unwrap_or(&addr)}"
                        }
                    }
                }
                
                // Strategy
                div {
                    class: "flex items-center justify-between p-3 bg-gray-50 dark:bg-gray-700 rounded-lg",
                    
                    span {
                        class: "text-sm font-medium text-gray-700 dark:text-gray-300",
                        "Strategy"
                    }
                    
                    span {
                        class: "inline-flex items-center text-sm text-gray-600 dark:text-gray-400",
                        
                        span {
                            class: "mr-2",
                            "{get_strategy_icon(status.strategy)}"
                        }
                        
                        "{status.strategy_name}"
                    }
                }
                
                // Permissions
                if !status.permissions.is_empty() {
                    div {
                        class: "p-3 bg-gray-50 dark:bg-gray-700 rounded-lg",
                        
                        div {
                            class: "text-sm font-medium text-gray-700 dark:text-gray-300 mb-2",
                            "Permissions"
                        }
                        
                        div {
                            class: "flex flex-wrap gap-1",
                            
                            for permission in status.permissions {
                                span {
                                    class: "inline-block px-2 py-1 text-xs bg-blue-100 text-blue-800 dark:bg-blue-900 dark:text-blue-200 rounded",
                                    "{permission}"
                                }
                            }
                        }
                    }
                }
                
                // Error display
                if status.has_error {
                    if let Some(error) = status.error {
                        div {
                            class: "p-3 bg-red-50 dark:bg-red-900/20 border border-red-200 dark:border-red-800 rounded-lg",
                            
                            div {
                                class: "text-sm font-medium text-red-800 dark:text-red-200 mb-1",
                                "Error"
                            }
                            
                            div {
                                class: "text-sm text-red-600 dark:text-red-400",
                                "{error}"
                            }
                        }
                    }
                }
            }
        }
    }
}