use async_trait::async_trait;
use std::collections::HashMap;
use wasm_bindgen::prelude::*;
// use wasm_bindgen_futures::JsFuture; // Not used currently
use web_sys::js_sys;
use anyhow::Result;

use crate::services::wallet::{WalletError, WalletStrategy, WalletStrategyType, WalletCapabilities};

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

/// Wander wallet strategy implementation
pub struct WanderStrategy;

impl WanderStrategy {
    pub fn new() -> Self {
        Self
    }
    
    /// Check if Wander wallet extension is available
    fn is_wallet_available() -> bool {
        !ARWEAVE_WALLET.is_undefined() && !ARWEAVE_WALLET.is_null()
    }
}

#[async_trait(?Send)]
impl WalletStrategy for WanderStrategy {
    fn strategy_type(&self) -> WalletStrategyType {
        WalletStrategyType::Wander
    }
    
    async fn is_available(&self) -> Result<bool, WalletError> {
        Ok(Self::is_wallet_available())
    }
    
    fn get_capabilities(&self) -> WalletCapabilities {
        WalletCapabilities {
            can_sign_transactions: true,
            can_encrypt_data: true,
            can_decrypt_data: true,
            supports_batch_signing: false,
            supports_permissions: true,
            supports_multiple_addresses: true,
        }
    }
    
    async fn connect(&mut self, permissions: Vec<&str>) -> Result<String, WalletError> {
        if !Self::is_wallet_available() {
            return Err(WalletError::NotInstalled);
        }
        
        // Convert permissions to JS array
        let js_permissions = js_sys::Array::new();
        for permission in permissions {
            js_permissions.push(&JsValue::from_str(permission));
        }
        
        match connect(js_permissions.into()).await {
            Ok(_) => {
                // Get the active address after successful connection
                self.get_active_address().await
            }
            Err(js_error) => {
                let error = WalletError::from(js_error);
                log::error!("Wander wallet connection failed: {}", error);
                Err(error)
            }
        }
    }
    
    async fn disconnect(&mut self) -> Result<(), WalletError> {
        match disconnect().await {
            Ok(_) => {
                log::info!("Successfully disconnected from Wander wallet");
                Ok(())
            }
            Err(js_error) => {
                let error = WalletError::from(js_error);
                log::error!("Wander wallet disconnection failed: {}", error);
                Err(error)
            }
        }
    }
    
    async fn get_active_address(&self) -> Result<String, WalletError> {
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
    
    async fn get_permissions(&self) -> Result<Vec<String>, WalletError> {
        match getPermissions().await {
            Ok(js_permissions) => {
                let permissions: Vec<String> = serde_wasm_bindgen::from_value(js_permissions)
                    .unwrap_or_else(|_| vec![]);
                Ok(permissions)
            }
            Err(js_error) => Err(WalletError::from(js_error))
        }
    }
    
    async fn sign_transaction(&self, transaction_data: HashMap<String, serde_json::Value>) -> Result<HashMap<String, serde_json::Value>, WalletError> {
        // Convert transaction data to JS value
        let tx_js = serde_wasm_bindgen::to_value(&transaction_data)
            .map_err(|e| WalletError::SigningFailed(format!("Serialization error: {}", e)))?;
        
        match sign(tx_js).await {
            Ok(signed_tx_js) => {
                let signed_tx: HashMap<String, serde_json::Value> = serde_wasm_bindgen::from_value(signed_tx_js)
                    .map_err(|e| WalletError::SigningFailed(format!("Deserialization error: {}", e)))?;
                
                log::info!("Transaction signed successfully with Wander wallet");
                Ok(signed_tx)
            }
            Err(js_error) => {
                let error = WalletError::from(js_error);
                log::error!("Wander wallet transaction signing failed: {}", error);
                Err(error)
            }
        }
    }
    
    async fn check_connection(&self) -> Result<bool, WalletError> {
        if !Self::is_wallet_available() {
            return Ok(false);
        }
        
        // Try to get active address to check if already connected
        match self.get_active_address().await {
            Ok(_) => Ok(true),
            Err(_) => Ok(false),
        }
    }
    
    /// Get all available addresses (Wander wallet supports multiple addresses)
    async fn get_all_addresses(&self) -> Result<Vec<String>, WalletError> {
        match getAllAddresses().await {
            Ok(js_addresses) => {
                let addresses: Vec<String> = serde_wasm_bindgen::from_value(js_addresses)
                    .unwrap_or_else(|_| vec![]);
                Ok(addresses)
            }
            Err(_js_error) => {
                // Fallback to single address if getAllAddresses not supported
                log::warn!("getAllAddresses failed, falling back to single address");
                match self.get_active_address().await {
                    Ok(address) => Ok(vec![address]),
                    Err(e) => Err(e),
                }
            }
        }
    }
    
    /// Encrypt data with Wander wallet (if supported)
    async fn encrypt(&self, _data: &[u8], _options: Option<HashMap<String, String>>) -> Result<Vec<u8>, WalletError> {
        // TODO: Implement encryption if Wander wallet supports it
        // For now, return error as feature not implemented
        Err(WalletError::InvalidPermissions)
    }
    
    /// Decrypt data with Wander wallet (if supported)
    async fn decrypt(&self, _data: &[u8], _options: Option<HashMap<String, String>>) -> Result<Vec<u8>, WalletError> {
        // TODO: Implement decryption if Wander wallet supports it
        // For now, return error as feature not implemented
        Err(WalletError::InvalidPermissions)
    }
}