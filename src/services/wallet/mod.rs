// Wallet service module for Faithful Archive
pub mod strategy;
pub mod wander;
pub mod beacon;
pub mod wallet_kit;
pub mod web_wallet;
pub mod context;
pub mod hooks;

// Re-export main types
pub use strategy::{
    WalletStrategy, WalletStrategyType, WalletCapabilities, 
    ExtendedWalletState, WalletStrategyManager
};
pub use context::{
    WalletContext, WalletProvider, WalletErrorBoundary, WalletGated,
    use_wallet_context, use_wallet_connection, use_wallet_operations,
    use_wallet_capabilities, use_wallet_strategies, WalletOperations
};
pub use hooks::{
    use_wallet_reconnect, use_wallet_persistence, use_wallet_signing,
    use_wallet_events, use_wallet_features, use_wallet_status,
    use_auto_wallet_strategy, use_wallet_error_recovery, use_wallet_connect_with_timeout,
    WalletFeatures, WalletStatus, StrategyColors,
    is_valid_arweave_address, get_strategy_icon, get_strategy_colors
};

// Original wallet types and errors
use serde::{Deserialize, Serialize};
use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WalletState {
    pub connected: bool,
    pub address: Option<String>,
    pub permissions: Vec<String>,
    pub error: Option<String>,
    pub connecting: bool,
    pub available: bool,
}

impl Default for WalletState {
    fn default() -> Self {
        Self {
            connected: false,
            address: None,
            permissions: vec![],
            error: None,
            connecting: false,
            available: false,
        }
    }
}

#[derive(Debug, Clone)]
pub enum WalletError {
    NotInstalled,
    UserDenied,
    NetworkError(String),
    InvalidPermissions,
    TransactionFailed(String),
    ConnectionFailed(String),
    SigningFailed(String),
}

impl std::fmt::Display for WalletError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WalletError::NotInstalled => write!(f, "No wallet is installed or available"),
            WalletError::UserDenied => write!(f, "User denied wallet connection"),
            WalletError::NetworkError(msg) => write!(f, "Network error: {}", msg),
            WalletError::InvalidPermissions => write!(f, "Invalid permissions requested"),
            WalletError::TransactionFailed(msg) => write!(f, "Transaction failed: {}", msg),
            WalletError::ConnectionFailed(msg) => write!(f, "Connection failed: {}", msg),
            WalletError::SigningFailed(msg) => write!(f, "Transaction signing failed: {}", msg),
        }
    }
}

impl From<wasm_bindgen::JsValue> for WalletError {
    fn from(js_error: wasm_bindgen::JsValue) -> Self {
        let error_msg = js_error.as_string().unwrap_or_else(|| {
            format!("{:?}", js_error)
        });
        
        if error_msg.to_lowercase().contains("not installed") || 
           error_msg.to_lowercase().contains("undefined") {
            WalletError::NotInstalled
        } else if error_msg.to_lowercase().contains("denied") || 
                  error_msg.to_lowercase().contains("rejected") {
            WalletError::UserDenied
        } else if error_msg.to_lowercase().contains("network") {
            WalletError::NetworkError(error_msg)
        } else if error_msg.to_lowercase().contains("permission") {
            WalletError::InvalidPermissions
        } else if error_msg.to_lowercase().contains("sign") {
            WalletError::SigningFailed(error_msg)
        } else {
            WalletError::ConnectionFailed(error_msg)
        }
    }
}

// Global extended wallet state using Dioxus signals
fn use_extended_wallet_state() -> &'static GlobalSignal<ExtendedWalletState> {
    static EXTENDED_WALLET_STATE: GlobalSignal<ExtendedWalletState> = GlobalSignal::new(ExtendedWalletState::default);
    &EXTENDED_WALLET_STATE
}

// Legacy compatibility - returns just the base wallet state  
pub fn use_wallet_state() -> Signal<WalletState> {
    let extended_state = use_extended_wallet_state();
    let mut wallet_state = use_signal(|| WalletState::default());
    
    use_effect(move || {
        wallet_state.set(extended_state().base_state.clone());
    });
    
    wallet_state
}

/// Enhanced wallet service with strategy support
pub struct WalletService {
    strategy_manager: WalletStrategyManager,
}

impl WalletService {
    pub fn new() -> Self {
        let mut strategy_manager = WalletStrategyManager::new();
        
        // Register all available strategies
        strategy_manager.register_strategy(Box::new(wander::WanderStrategy::new()));
        strategy_manager.register_strategy(Box::new(beacon::BeaconStrategy::new()));
        // TODO: Register other strategies when implemented
        // strategy_manager.register_strategy(Box::new(wallet_kit::WalletKitStrategy::new()));
        // strategy_manager.register_strategy(Box::new(web_wallet::WebWalletStrategy::new()));
        
        Self { strategy_manager }
    }
    
    /// Initialize wallet service and discover available strategies
    pub async fn init() -> Self {
        let mut service = Self::new();
        let extended_state = use_extended_wallet_state();
        
        log::info!("🚀 Initializing WalletService with {} registered strategies", service.strategy_manager.strategy_count());
        
        // Discover available strategies
        let available_strategies = service.strategy_manager.get_available_strategies().await;
        log::info!("🔍 Found {} available strategies: {:?}", available_strategies.len(), available_strategies);
        extended_state.write().available_strategies = available_strategies.clone();
        
        // Auto-select best strategy if any available
        if !available_strategies.is_empty() {
            if let Ok(selected_strategy) = service.strategy_manager.auto_select_strategy().await {
                log::info!("✅ Auto-selected strategy: {:?}", selected_strategy);
                extended_state.write().strategy = selected_strategy;
                extended_state.write().base_state.available = true;
                
                // Set capabilities for selected strategy
                if let Some(strategy) = service.strategy_manager.get_current_strategy() {
                    extended_state.write().capabilities = strategy.get_capabilities();
                }
            }
        } else {
            log::warn!("❌ No wallet strategies available");
            extended_state.write().base_state.available = false;
            extended_state.write().base_state.error = Some("No wallet strategies available".to_string());
        }
        
        service
    }
    
    /// Get available wallet strategies
    pub async fn get_available_strategies(&self) -> Vec<WalletStrategyType> {
        self.strategy_manager.get_available_strategies().await
    }
    
    /// Set active wallet strategy
    pub async fn set_strategy(&mut self, strategy_type: WalletStrategyType) -> Result<(), WalletError> {
        self.strategy_manager.set_strategy(strategy_type)?;
        
        let extended_state = use_extended_wallet_state();
        extended_state.write().strategy = strategy_type;
        
        // Update capabilities
        if let Some(strategy) = self.strategy_manager.get_current_strategy() {
            extended_state.write().capabilities = strategy.get_capabilities();
        }
        
        Ok(())
    }
    
    /// Connect using current strategy
    pub async fn connect(&mut self) -> Result<String, WalletError> {
        let extended_state = use_extended_wallet_state();
        
        extended_state.write().base_state.connecting = true;
        extended_state.write().base_state.error = None;
        
        let permissions = vec!["ACCESS_ADDRESS", "SIGN_TRANSACTION", "ACCESS_PUBLIC_KEY"];
        let permissions_clone = permissions.clone();
        
        let result = self.strategy_manager.with_current_strategy_mut(|strategy| {
            Box::pin(async move {
                strategy.connect(permissions_clone).await
            })
        }).await;
        
        match result {
            Ok(address) => {
                extended_state.write().base_state.connected = true;
                extended_state.write().base_state.address = Some(address.clone());
                extended_state.write().base_state.permissions = permissions.into_iter().map(|s| s.to_string()).collect();
                extended_state.write().base_state.connecting = false;
                Ok(address)
            }
            Err(e) => {
                extended_state.write().base_state.connecting = false;
                extended_state.write().base_state.error = Some(e.to_string());
                Err(e)
            }
        }
    }
    
    /// Disconnect using current strategy
    pub async fn disconnect(&mut self) -> Result<(), WalletError> {
        let extended_state = use_extended_wallet_state();
        
        let result = self.strategy_manager.with_current_strategy_mut(|strategy| {
            Box::pin(async move {
                strategy.disconnect().await
            })
        }).await;
        
        match result {
            Ok(()) => {
                extended_state.write().base_state.connected = false;
                extended_state.write().base_state.address = None;
                extended_state.write().base_state.permissions.clear();
                extended_state.write().base_state.error = None;
                Ok(())
            }
            Err(e) => {
                extended_state.write().base_state.error = Some(e.to_string());
                Err(e)
            }
        }
    }
    
    /// Get active address using current strategy
    pub async fn get_active_address(&self) -> Result<String, WalletError> {
        if let Some(strategy) = self.strategy_manager.get_current_strategy() {
            strategy.get_active_address().await
        } else {
            Err(WalletError::NotInstalled)
        }
    }
    
    /// Sign transaction using current strategy
    pub async fn sign_transaction(&self, transaction_data: std::collections::HashMap<String, serde_json::Value>) -> Result<std::collections::HashMap<String, serde_json::Value>, WalletError> {
        if let Some(strategy) = self.strategy_manager.get_current_strategy() {
            strategy.sign_transaction(transaction_data).await
        } else {
            Err(WalletError::NotInstalled)
        }
    }
    
    /// Check connection status using current strategy
    pub async fn check_connection(&self) -> Result<bool, WalletError> {
        if let Some(strategy) = self.strategy_manager.get_current_strategy() {
            strategy.check_connection().await
        } else {
            Ok(false)
        }
    }
    
    /// Format address for display
    pub fn format_address(address: &str) -> String {
        if address.len() <= 10 {
            address.to_string()
        } else {
            format!("{}...{}", &address[..6], &address[address.len()-4..])
        }
    }
    
    /// Get current extended wallet state
    pub fn get_extended_state() -> Signal<ExtendedWalletState> {
        use_extended_wallet_state().signal()
    }
}

impl Default for WalletService {
    fn default() -> Self {
        Self::new()
    }
}

// Legacy wallet button component - maintains compatibility
#[component]
pub fn WalletButton() -> Element {
    let wallet_state = use_wallet_state();
    
    // TODO: Replace with WalletStrategySelector for multi-strategy support
    let connect_handler = {
        let wallet_state = wallet_state.clone();
        move |_| {
            let wallet_state = wallet_state.clone();
            spawn(async move {
                let mut service = WalletService::new();
                if wallet_state.read().connected {
                    let _ = service.disconnect().await;
                } else {
                    let _ = service.connect().await;
                }
            });
        }
    };
    
    let button_text = if wallet_state.read().connecting {
        "Connecting..."
    } else if wallet_state.read().connected {
        "Disconnect Wallet"
    } else if !wallet_state.read().available {
        "No Wallet Available"
    } else {
        "Connect Wallet"
    };
    
    let button_class = if wallet_state.read().connected {
        "bg-red-600 hover:bg-red-700 text-white px-4 py-2 rounded-lg text-sm font-medium transition-colors"
    } else if !wallet_state.read().available {
        "bg-gray-400 cursor-not-allowed text-white px-4 py-2 rounded-lg text-sm font-medium"
    } else {
        "bg-green-600 hover:bg-green-700 text-white px-4 py-2 rounded-lg text-sm font-medium transition-colors"
    };
    
    rsx! {
        div {
            class: "wallet-button-container",
            
            button {
                class: button_class,
                disabled: wallet_state.read().connecting || !wallet_state.read().available,
                onclick: connect_handler,
                "{button_text}"
            }
            
            if wallet_state.read().connected {
                div {
                    class: "mt-2 text-xs text-gray-600",
                    "Connected: {WalletService::format_address(wallet_state.read().address.as_ref().unwrap_or(&\"Unknown\".to_string()))}"
                }
            }
            
            if let Some(error) = &wallet_state.read().error {
                div {
                    class: "mt-2 text-xs text-red-600",
                    "{error}"
                }
            }
        }
    }
}

// Initialize wallet service (legacy compatibility)
pub fn init_wallet_service() {
    // Initialize in async context
    spawn(async {
        let _service = WalletService::init().await;
    });
}