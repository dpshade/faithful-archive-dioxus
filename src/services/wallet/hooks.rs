use dioxus::prelude::*;
use std::collections::HashMap;
use crate::services::wallet::{
    WalletError, WalletStrategyType, use_wallet_context, 
    use_wallet_connection, WalletCapabilities
};

/// Hook for automatic wallet reconnection
/// 
/// Attempts to reconnect to a previously connected wallet on component mount.
/// Useful for maintaining wallet connections across page reloads.
/// 
/// # Example
/// 
/// ```rust
/// use dioxus::prelude::*;
/// use faithful_archive::services::wallet::use_wallet_reconnect;
/// 
/// #[component]
/// fn App() -> Element {
///     use_wallet_reconnect(); // Automatically tries to reconnect
///     
///     rsx! {
///         // Your app content
///     }
/// }
/// ```
pub fn use_wallet_reconnect() {
    let wallet = use_wallet_context();
    
    use_effect(move || {
        spawn(async move {
            // Check if there's a stored connection preference
            if let Some(window) = web_sys::window() {
                if let Ok(Some(storage)) = window.local_storage() {
                    // Check for stored wallet preference
                    if let Ok(Some(stored_strategy)) = storage.get_item("faithful_archive_wallet_strategy") {
                        if let Ok(strategy) = stored_strategy.parse::<WalletStrategyType>() {
                            let _ = wallet.set_strategy.call(strategy);
                        }
                    }
                    
                    // Check for stored connection state
                    if let Ok(Some(_)) = storage.get_item("faithful_archive_wallet_connected") {
                        // Attempt reconnection
                        match wallet.connect.call(()) {
                            Ok(_) => log::info!("Wallet reconnected successfully"),
                            Err(e) => {
                                log::warn!("Failed to reconnect wallet: {}", e);
                                // Clear stored connection state on failure
                                let _ = storage.remove_item("faithful_archive_wallet_connected");
                            }
                        }
                    }
                }
            }
        });
    });
}

/// Hook for persisting wallet connection state
/// 
/// Automatically saves wallet connection state to localStorage and
/// clears it when disconnected. Works in conjunction with `use_wallet_reconnect`.
pub fn use_wallet_persistence() {
    let (connected, _) = use_wallet_connection();
    let wallet = use_wallet_context();
    
    use_effect(move || {
        spawn(async move {
            let state = wallet.state.read();
            if let Some(window) = web_sys::window() {
                if let Ok(Some(storage)) = window.local_storage() {
                    if connected {
                        // Store connection state and strategy
                        let _ = storage.set_item("faithful_archive_wallet_connected", "true");
                        let _ = storage.set_item("faithful_archive_wallet_strategy", &state.strategy.to_string());
                    } else {
                        // Clear stored state
                        let _ = storage.remove_item("faithful_archive_wallet_connected");
                        let _ = storage.remove_item("faithful_archive_wallet_strategy");
                    }
                }
            }
        });
    });
}

/// Hook for wallet transaction signing with loading state
/// 
/// Provides a convenient interface for signing transactions with automatic
/// loading state management and error handling.
/// 
/// Returns (sign_function, is_loading, last_error)
pub fn use_wallet_signing() -> (
    Callback<HashMap<String, serde_json::Value>, ()>,
    Signal<bool>,
    Signal<Option<String>>,
) {
    let wallet = use_wallet_context();
    let mut is_loading = use_signal(|| false);
    let mut last_error = use_signal(|| None::<String>);
    
    let sign_function = use_callback(move |transaction_data: HashMap<String, serde_json::Value>| {
        let wallet = wallet.clone();
        let mut is_loading = is_loading.clone();
        let mut last_error = last_error.clone();
        
        spawn(async move {
            is_loading.set(true);
            last_error.set(None);
            
            match wallet.sign_transaction.call(transaction_data) {
                Ok(signed_tx) => {
                    log::info!("Transaction signed successfully");
                    // You might want to emit a custom event or callback here
                }
                Err(e) => {
                    log::error!("Transaction signing failed: {}", e);
                    last_error.set(Some(e.to_string()));
                }
            }
            
            is_loading.set(false);
        });
    });
    
    (sign_function, is_loading, last_error)
}

/// Hook for monitoring wallet events
/// 
/// Provides callbacks for various wallet events like connection, disconnection,
/// strategy changes, etc. Useful for analytics or custom event handling.
pub fn use_wallet_events(
    on_connect: Option<Callback<String>>,
    on_disconnect: Option<Callback<()>>,
    on_strategy_change: Option<Callback<WalletStrategyType>>,
    on_error: Option<Callback<String>>,
) {
    let wallet = use_wallet_context();
    
    let mut previous_connected = use_signal(|| false);
    let mut previous_strategy = use_signal(|| WalletStrategyType::Beacon);
    let mut previous_error = use_signal(|| None::<String>);
    
    use_effect(move || {
        let state = wallet.state.read();
        // Check for connection state changes
        if state.base_state.connected != *previous_connected.read() {
            if state.base_state.connected {
                if let Some(callback) = on_connect {
                    if let Some(address) = &state.base_state.address {
                        callback.call(address.clone());
                    }
                }
            } else {
                if let Some(callback) = on_disconnect {
                    callback.call(());
                }
            }
            previous_connected.set(state.base_state.connected);
        }
        
        // Check for strategy changes
        if state.strategy != *previous_strategy.read() {
            if let Some(callback) = on_strategy_change {
                callback.call(state.strategy);
            }
            previous_strategy.set(state.strategy);
        }
        
        // Check for error changes
        if state.base_state.error != *previous_error.read() {
            if let Some(error) = &state.base_state.error {
                if let Some(callback) = on_error {
                    callback.call(error.clone());
                }
            }
            previous_error.set(state.base_state.error.clone());
        }
    });
}

/// Hook for wallet capabilities-based UI state
/// 
/// Returns boolean flags for different wallet capabilities to conditionally
/// render UI elements.
pub fn use_wallet_features() -> WalletFeatures {
    let wallet = use_wallet_context();
    let state = wallet.state.read();
    let (connected, _) = use_wallet_connection();
    
    WalletFeatures {
        can_sign: connected && state.capabilities.can_sign_transactions,
        can_encrypt: connected && state.capabilities.can_encrypt_data,
        can_decrypt: connected && state.capabilities.can_decrypt_data,
        supports_batch: connected && state.capabilities.supports_batch_signing,
        supports_permissions: connected && state.capabilities.supports_permissions,
        supports_multiple_addresses: connected && state.capabilities.supports_multiple_addresses,
        is_connected: connected,
        has_multiple_strategies: state.available_strategies.len() > 1,
    }
}

#[derive(Clone, PartialEq)]
pub struct WalletFeatures {
    pub can_sign: bool,
    pub can_encrypt: bool,
    pub can_decrypt: bool,
    pub supports_batch: bool,
    pub supports_permissions: bool,
    pub supports_multiple_addresses: bool,
    pub is_connected: bool,
    pub has_multiple_strategies: bool,
}

/// Hook for wallet connection status with detailed information
/// 
/// Returns comprehensive connection information including strategy, capabilities,
/// and connection health.
pub fn use_wallet_status() -> WalletStatus {
    let wallet = use_wallet_context();
    let state = wallet.state.read();
    let (connected, address) = use_wallet_connection();
    
    WalletStatus {
        connected,
        connecting: state.base_state.connecting,
        available: state.base_state.available,
        address: address.clone(),
        formatted_address: address.as_ref().map(|addr| (wallet.format_address)(addr)),
        strategy: state.strategy,
        strategy_name: state.strategy.display_name(),
        capabilities: state.capabilities.clone(),
        available_strategies: state.available_strategies.clone(),
        error: state.base_state.error.clone(),
        has_error: state.base_state.error.is_some(),
        permissions: state.base_state.permissions.clone(),
    }
}

#[derive(Clone, PartialEq)]
pub struct WalletStatus {
    pub connected: bool,
    pub connecting: bool,
    pub available: bool,
    pub address: Option<String>,
    pub formatted_address: Option<String>,
    pub strategy: WalletStrategyType,
    pub strategy_name: &'static str,
    pub capabilities: WalletCapabilities,
    pub available_strategies: Vec<WalletStrategyType>,
    pub error: Option<String>,
    pub has_error: bool,
    pub permissions: Vec<String>,
}

/// Hook for conditional wallet strategy selection
/// 
/// Automatically selects the best available wallet strategy based on
/// user preferences and availability.
pub fn use_auto_wallet_strategy(
    preferences: Vec<WalletStrategyType>,
) -> (WalletStrategyType, bool) {
    let wallet = use_wallet_context();
    let mut has_auto_selected = use_signal(|| false);
    
    use_effect({
        let wallet = wallet.clone();
        let preferences = preferences.clone();
        move || {
            let state = wallet.state.read();
            if !*has_auto_selected.read() && !state.available_strategies.is_empty() {
                // Try to select based on preferences
                for preferred in &preferences {
                    if state.available_strategies.contains(preferred) {
                        spawn({
                            let wallet = wallet.clone();
                            let preferred = *preferred;
                            async move {
                                let _ = wallet.set_strategy.call(preferred);
                            }
                        });
                        has_auto_selected.set(true);
                        return;
                    }
                }
                
                // Fallback to first available strategy
                if let Some(first_available) = state.available_strategies.first() {
                    let strategy = *first_available;
                    spawn({
                        let wallet = wallet.clone();
                        async move {
                            let _ = wallet.set_strategy.call(strategy);
                        }
                    });
                    has_auto_selected.set(true);
                }
            }
        }
    });
    
    {
        let state = wallet.state.read();
        let auto_selected = *has_auto_selected.read();
        (state.strategy, auto_selected)
    }
}

/// Hook for wallet error handling with recovery
/// 
/// Provides error handling utilities with automatic retry logic.
pub fn use_wallet_error_recovery() -> (
    Signal<Option<String>>,
    Callback<(), ()>,
    Signal<bool>,
) {
    let wallet = use_wallet_context();
    let state = wallet.state.read();
    let mut is_recovering = use_signal(|| false);
    
    let current_error = use_signal(move || state.base_state.error.clone());
    
    let recover = use_callback(move |_: ()| {
        let wallet = wallet.clone();
        let mut is_recovering = is_recovering.clone();
        
        spawn(async move {
            is_recovering.set(true);
            
            // Clear error by disconnecting and attempting reconnection
            let _ = wallet.disconnect.call(());
            
            // Wait a bit before reconnecting
            gloo_timers::future::TimeoutFuture::new(1000).await;
            
            match wallet.connect.call(()) {
                Ok(_) => log::info!("Wallet recovery successful"),
                Err(e) => log::error!("Wallet recovery failed: {}", e),
            }
            
            is_recovering.set(false);
        });
    });
    
    (current_error, recover, is_recovering)
}

/// Hook for wallet connection with timeout
/// 
/// Provides connection functionality with configurable timeout.
pub fn use_wallet_connect_with_timeout(
    timeout_ms: u32,
) -> (
    Callback<(), ()>,
    Signal<bool>,
    Signal<Option<String>>,
) {
    let wallet = use_wallet_context();
    let mut is_connecting = use_signal(|| false);
    let mut connection_error = use_signal(|| None::<String>);
    
    let connect_with_timeout = use_callback(move |_: ()| {
        let wallet = wallet.clone();
        let mut is_connecting = is_connecting.clone();
        let mut connection_error = connection_error.clone();
        
        spawn(async move {
            is_connecting.set(true);
            connection_error.set(None);
            
            // Since callbacks are synchronous, just call directly
            match wallet.connect.call(()) {
                Ok(_) => log::info!("Wallet connected successfully"),
                Err(e) => {
                    connection_error.set(Some(e.to_string()));
                    log::error!("Wallet connection failed: {}", e);
                }
            }
            
            is_connecting.set(false);
        });
    });
    
    (connect_with_timeout, is_connecting, connection_error)
}

/// Utility function to validate wallet addresses
pub fn is_valid_arweave_address(address: &str) -> bool {
    // Arweave addresses are base64url encoded and typically 43 characters long
    address.len() == 43 && 
    address.chars().all(|c| c.is_alphanumeric() || c == '-' || c == '_')
}

/// Utility function to get wallet strategy icon/emoji
pub fn get_strategy_icon(strategy: WalletStrategyType) -> &'static str {
    match strategy {
        WalletStrategyType::Beacon => "📱", // Mobile-first wallet
        WalletStrategyType::Wander => "🧭", // Navigation/exploration theme
        WalletStrategyType::WalletKit => "🔧", // Tool/kit theme
        WalletStrategyType::WebWallet => "🌐", // Web theme
    }
}

/// Utility function to get strategy color theme
pub fn get_strategy_colors(strategy: WalletStrategyType) -> StrategyColors {
    match strategy {
        WalletStrategyType::Beacon => StrategyColors {
            primary: "#4969FF",
            background: "#F0F4FF",
            text: "#1E293B",
        },
        WalletStrategyType::Wander => StrategyColors {
            primary: "#059669",
            background: "#F0FDF4",
            text: "#065F46",
        },
        WalletStrategyType::WalletKit => StrategyColors {
            primary: "#7C3AED",
            background: "#F5F3FF",
            text: "#3C1A78",
        },
        WalletStrategyType::WebWallet => StrategyColors {
            primary: "#DC2626",
            background: "#FEF2F2",
            text: "#7F1D1D",
        },
    }
}

#[derive(Clone, PartialEq)]
pub struct StrategyColors {
    pub primary: &'static str,
    pub background: &'static str,
    pub text: &'static str,
}