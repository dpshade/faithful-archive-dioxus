use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::js_sys;
use anyhow::{Result, anyhow};
use std::collections::HashMap;

// WASM bindings for Wander wallet (formerly ArConnect)
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "arweaveWallet"], catch)]
    async fn connect(permissions: JsValue) -> Result<JsValue, JsValue>;
    
    #[wasm_bindgen(js_namespace = ["window", "arweaveWallet"], catch)]
    async fn disconnect() -> Result<JsValue, JsValue>;
    
    #[wasm_bindgen(js_namespace = ["window", "arweaveWallet"], catch)]
    async fn getActiveAddress() -> Result<JsValue, JsValue>;
    
    #[wasm_bindgen(js_namespace = ["window", "arweaveWallet"], catch)]
    async fn getPermissions() -> Result<JsValue, JsValue>;
    
    #[wasm_bindgen(js_namespace = ["window", "arweaveWallet"], catch)]
    async fn sign(transaction: JsValue) -> Result<JsValue, JsValue>;
    
    #[wasm_bindgen(js_namespace = ["window", "arweaveWallet"], catch)]
    async fn getWalletNames() -> Result<JsValue, JsValue>;
    
    #[wasm_bindgen(js_namespace = ["window", "arweaveWallet"], catch)]
    async fn getAllAddresses() -> Result<JsValue, JsValue>;
    
    // Check if wallet extension is available
    #[wasm_bindgen(js_namespace = ["window"], js_name = "arweaveWallet")]
    static ARWEAVE_WALLET: JsValue;
}

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
            WalletError::NotInstalled => write!(f, "Wander wallet extension is not installed"),
            WalletError::UserDenied => write!(f, "User denied wallet connection"),
            WalletError::NetworkError(msg) => write!(f, "Network error: {}", msg),
            WalletError::InvalidPermissions => write!(f, "Invalid permissions requested"),
            WalletError::TransactionFailed(msg) => write!(f, "Transaction failed: {}", msg),
            WalletError::ConnectionFailed(msg) => write!(f, "Connection failed: {}", msg),
            WalletError::SigningFailed(msg) => write!(f, "Transaction signing failed: {}", msg),
        }
    }
}

impl From<JsValue> for WalletError {
    fn from(js_error: JsValue) -> Self {
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

// Global wallet state using Dioxus signals
pub fn use_wallet_state() -> &'static GlobalSignal<WalletState> {
    static WALLET_STATE: GlobalSignal<WalletState> = GlobalSignal::new(WalletState::default);
    &WALLET_STATE
}

pub struct WalletService;

impl WalletService {
    /// Check if Wander wallet extension is available
    pub fn is_wallet_available() -> bool {
        !ARWEAVE_WALLET.is_undefined() && !ARWEAVE_WALLET.is_null()
    }
    
    /// Initialize wallet service and check availability
    pub fn init() {
        let wallet_state = use_wallet_state();
        wallet_state.write().available = Self::is_wallet_available();
        
        if !wallet_state.read().available {
            wallet_state.write().error = Some("Wander wallet extension not found. Please install it from https://wander.app".to_string());
        }
    }
    
    /// Connect to Wander wallet with required permissions
    pub async fn connect() -> Result<String, WalletError> {
        let wallet_state = use_wallet_state();
        
        if !Self::is_wallet_available() {
            wallet_state.write().error = Some("Wallet not available".to_string());
            return Err(WalletError::NotInstalled);
        }
        
        wallet_state.write().connecting = true;
        wallet_state.write().error = None;
        
        // Request required permissions for Faithful Archive
        let permissions = js_sys::Array::new();
        permissions.push(&JsValue::from_str("ACCESS_ADDRESS"));
        permissions.push(&JsValue::from_str("SIGN_TRANSACTION"));
        permissions.push(&JsValue::from_str("ACCESS_PUBLIC_KEY"));
        
        match connect(permissions.into()).await {
            Ok(_) => {
                // Get the active address after successful connection
                match Self::get_active_address().await {
                    Ok(address) => {
                        wallet_state.write().connected = true;
                        wallet_state.write().address = Some(address.clone());
                        wallet_state.write().permissions = vec![
                            "ACCESS_ADDRESS".to_string(),
                            "SIGN_TRANSACTION".to_string(),
                            "ACCESS_PUBLIC_KEY".to_string(),
                        ];
                        wallet_state.write().connecting = false;
                        log::info!("Successfully connected to wallet: {}", address);
                        Ok(address)
                    }
                    Err(e) => {
                        wallet_state.write().connecting = false;
                        wallet_state.write().error = Some(e.to_string());
                        Err(e)
                    }
                }
            }
            Err(js_error) => {
                let error = WalletError::from(js_error);
                wallet_state.write().connecting = false;
                wallet_state.write().error = Some(error.to_string());
                log::error!("Wallet connection failed: {}", error);
                Err(error)
            }
        }
    }
    
    /// Disconnect from Wander wallet
    pub async fn disconnect() -> Result<(), WalletError> {
        let wallet_state = use_wallet_state();
        
        match disconnect().await {
            Ok(_) => {
                wallet_state.write().connected = false;
                wallet_state.write().address = None;
                wallet_state.write().permissions.clear();
                wallet_state.write().error = None;
                log::info!("Successfully disconnected from wallet");
                Ok(())
            }
            Err(js_error) => {
                let error = WalletError::from(js_error);
                wallet_state.write().error = Some(error.to_string());
                log::error!("Wallet disconnection failed: {}", error);
                Err(error)
            }
        }
    }
    
    /// Get the currently active wallet address
    pub async fn get_active_address() -> Result<String, WalletError> {
        match getActiveAddress().await {
            Ok(js_address) => {
                let address = js_address.as_string().unwrap_or_default();
                if address.is_empty() {
                    Err(WalletError::ConnectionFailed("No active address found".to_string()))
                } else {
                    Ok(address)
                }
            }
            Err(js_error) => Err(WalletError::from(js_error))
        }
    }
    
    /// Get current wallet permissions
    pub async fn get_permissions() -> Result<Vec<String>, WalletError> {
        match getPermissions().await {
            Ok(js_permissions) => {
                let permissions: Vec<String> = serde_wasm_bindgen::from_value(js_permissions)
                    .unwrap_or_else(|_| vec![]);
                Ok(permissions)
            }
            Err(js_error) => Err(WalletError::from(js_error))
        }
    }
    
    /// Sign a transaction with the connected wallet
    pub async fn sign_transaction(transaction_data: HashMap<String, serde_json::Value>) -> Result<HashMap<String, serde_json::Value>, WalletError> {
        let wallet_state = use_wallet_state();
        
        if !wallet_state.read().connected {
            return Err(WalletError::ConnectionFailed("Wallet not connected".to_string()));
        }
        
        // Convert transaction data to JS value
        let tx_js = serde_wasm_bindgen::to_value(&transaction_data)
            .map_err(|e| WalletError::SigningFailed(format!("Serialization error: {}", e)))?;
        
        match sign(tx_js).await {
            Ok(signed_tx_js) => {
                let signed_tx: HashMap<String, serde_json::Value> = serde_wasm_bindgen::from_value(signed_tx_js)
                    .map_err(|e| WalletError::SigningFailed(format!("Deserialization error: {}", e)))?;
                
                log::info!("Transaction signed successfully");
                Ok(signed_tx)
            }
            Err(js_error) => {
                let error = WalletError::from(js_error);
                wallet_state.write().error = Some(error.to_string());
                log::error!("Transaction signing failed: {}", error);
                Err(error)
            }
        }
    }
    
    /// Check wallet connection status and update state
    pub async fn check_connection() -> Result<bool, WalletError> {
        let wallet_state = use_wallet_state();
        
        if !Self::is_wallet_available() {
            wallet_state.write().available = false;
            wallet_state.write().connected = false;
            return Ok(false);
        }
        
        wallet_state.write().available = true;
        
        // Try to get active address to check if already connected
        match Self::get_active_address().await {
            Ok(address) => {
                wallet_state.write().connected = true;
                wallet_state.write().address = Some(address);
                
                // Also get current permissions
                if let Ok(permissions) = Self::get_permissions().await {
                    wallet_state.write().permissions = permissions;
                }
                
                Ok(true)
            }
            Err(_) => {
                wallet_state.write().connected = false;
                wallet_state.write().address = None;
                wallet_state.write().permissions.clear();
                Ok(false)
            }
        }
    }
    
    /// Format address for display (show first 6 and last 4 characters)
    pub fn format_address(address: &str) -> String {
        if address.len() <= 10 {
            address.to_string()
        } else {
            format!("{}...{}", &address[..6], &address[address.len()-4..])
        }
    }
    
    /// Get wallet state for reactive UI updates
    pub fn get_state() -> Signal<WalletState> {
        use_wallet_state().signal()
    }
}

// Dioxus component for wallet connection button
#[component]
pub fn WalletButton() -> Element {
    let wallet_state = WalletService::get_state();
    
    // Check wallet status on component mount
    use_effect(move || {
        spawn(async move {
            let _ = WalletService::check_connection().await;
        });
    });
    
    let connect_handler = move |_| {
        spawn(async move {
            if wallet_state.read().connected {
                let _ = WalletService::disconnect().await;
            } else {
                let _ = WalletService::connect().await;
            }
        });
    };
    
    let button_text = if wallet_state.read().connecting {
        "Connecting..."
    } else if wallet_state.read().connected {
        "Disconnect Wallet"
    } else if !wallet_state.read().available {
        "Install Wander Wallet"
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
            
            // Main wallet button
            button {
                class: button_class,
                disabled: wallet_state.read().connecting || !wallet_state.read().available,
                onclick: connect_handler,
                "{button_text}"
            }
            
            // Address display when connected
            if wallet_state.read().connected {
                div {
                    class: "mt-2 text-xs text-gray-600",
                    "Connected: {WalletService::format_address(wallet_state.read().address.as_ref().unwrap_or(&\"Unknown\".to_string()))}"
                }
            }
            
            // Error display
            if let Some(error) = &wallet_state.read().error {
                div {
                    class: "mt-2 text-xs text-red-600",
                    "{error}"
                }
            }
        }
    }
}

// Initialize wallet service on app startup
pub fn init_wallet_service() {
    WalletService::init();
}