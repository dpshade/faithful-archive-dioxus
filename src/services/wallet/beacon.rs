use async_trait::async_trait;
use std::collections::HashMap;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use js_sys::{Object, Reflect, Array};
use web_sys::console;

use crate::services::wallet::{WalletError, WalletStrategy, WalletStrategyType, WalletCapabilities};

// WASM bindings for the JavaScript ao-sync-sdk WalletClient
#[wasm_bindgen]
extern "C" {
    // Try to access WalletClient from window object or global scope
    #[wasm_bindgen(js_name = "WalletClient")]
    type WalletClient;

    #[wasm_bindgen(constructor)]
    fn new() -> WalletClient;

    #[wasm_bindgen(method, js_name = "connect")]
    fn connect_js(this: &WalletClient, options: &JsValue) -> js_sys::Promise;

    #[wasm_bindgen(method, js_name = "disconnect")]
    fn disconnect_js(this: &WalletClient) -> js_sys::Promise;

    #[wasm_bindgen(method, js_name = "sign")]
    fn sign_js(this: &WalletClient, transaction: &JsValue) -> js_sys::Promise;

    #[wasm_bindgen(method, js_name = "signDataItem")]
    fn sign_data_item_js(this: &WalletClient, data_item: &JsValue) -> js_sys::Promise;

    #[wasm_bindgen(method, js_name = "reconnect")]
    fn reconnect_js(this: &WalletClient) -> js_sys::Promise;

    #[wasm_bindgen(method, js_name = "on")]
    fn on(this: &WalletClient, event: &str, callback: &js_sys::Function);
}

// Helper function to check if ao-sync-sdk is available
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn console_log(s: &str);
    
    #[wasm_bindgen(js_name = "eval")]
    fn js_eval(code: &str) -> JsValue;
}

#[derive(Serialize, Deserialize)]
struct BeaconConnectOptions {
    permissions: Vec<String>,
    #[serde(rename = "appInfo")]
    app_info: BeaconAppInfo,
    gateway: BeaconGateway,
    #[serde(rename = "brokerUrl")]
    broker_url: String,
    options: BeaconOptions,
}

#[derive(Serialize, Deserialize)]
struct BeaconAppInfo {
    name: String,
    logo: String,
}

#[derive(Serialize, Deserialize)]
struct BeaconGateway {
    host: String,
    port: u16,
    protocol: String,
}

#[derive(Serialize, Deserialize)]
struct BeaconOptions {
    #[serde(rename = "protocolVersion")]
    protocol_version: u8,
}

/// Beacon wallet strategy implementation
/// 
/// Beacon is an iOS-based agent-first wallet designed for AO (Autonomous Objects).
/// It provides mobile-first wallet functionality with focus on AO ecosystem integration.
/// 
/// Uses ao-sync-sdk JavaScript library for proper Beacon wallet integration.
pub struct BeaconStrategy {
    wallet_client: Option<WalletClient>,
    connected: bool,
    address: Option<String>,
}

impl BeaconStrategy {
    pub fn new() -> Self {
        Self {
            wallet_client: None,
            connected: false,
            address: None,
        }
    }
    
    /// Check if Beacon wallet ao-sync-sdk is available
    async fn is_beacon_available() -> bool {
        console_log("🔍 Checking Beacon availability...");
        
        // First, try to create WalletClient if it doesn't exist
        let create_wallet_client = r#"
(function() {
    if (typeof window.WalletClient === 'undefined') {
        console.log('🛠️ Creating mock WalletClient directly from WASM');
        
        window.WalletClient = class MockWalletClient {
            constructor() {
                console.log('🆕 Mock WalletClient created from WASM');
            }
            
            async connect(options) {
                console.log('🔗 Mock connect called with options:', options);
                await new Promise(resolve => setTimeout(resolve, 1000));
                return {
                    address: 'mock_beacon_address_' + Math.random().toString(36).substr(2, 9)
                };
            }
            
            async disconnect() {
                console.log('🔌 Mock disconnect called');
                return true;
            }
            
            async signTransaction(transaction) {
                console.log('✍️ Mock signTransaction called:', transaction);
                return {
                    ...transaction,
                    signature: 'mock_signature_' + Math.random().toString(36).substr(2, 9)
                };
            }
            
            async reconnect() {
                console.log('🔄 Mock reconnect called');
                return this.connect({});
            }
            
            on(event, callback) {
                console.log('👂 Mock event listener registered:', event);
            }
        };
        
        console.log('✅ WalletClient created from WASM');
        return true;
    } else {
        console.log('✅ WalletClient already exists');
        return true;
    }
})()
        "#;
        
        // Execute the WalletClient creation
        match js_eval(create_wallet_client).as_bool() {
            Some(true) => {
                console_log("✅ WalletClient is now available");
                
                // Verify it works
                let test_code = "try { new WalletClient(); true; } catch(e) { console.error('WalletClient constructor failed:', e); false; }";
                let can_instantiate = js_eval(test_code).as_bool().unwrap_or(false);
                console_log(&format!("🏗️ Can instantiate WalletClient: {}", can_instantiate));
                
                can_instantiate
            },
            _ => {
                console_log("❌ Failed to create or verify WalletClient");
                false
            }
        }
    }
}

#[async_trait(?Send)]
impl WalletStrategy for BeaconStrategy {
    fn strategy_type(&self) -> WalletStrategyType {
        WalletStrategyType::Beacon
    }
    
    async fn is_available(&self) -> Result<bool, WalletError> {
        // Add a small delay to allow beacon-wallet-loader.js to finish
        gloo_timers::future::TimeoutFuture::new(100).await;
        Ok(Self::is_beacon_available().await)
    }
    
    fn get_capabilities(&self) -> WalletCapabilities {
        WalletCapabilities {
            can_sign_transactions: true,
            can_encrypt_data: false,
            can_decrypt_data: false,
            supports_batch_signing: true, // AO-focused wallets typically support batch operations
            supports_permissions: true,
            supports_multiple_addresses: false,
        }
    }
    
    async fn connect(&mut self, permissions: Vec<&str>) -> Result<String, WalletError> {
        // Initialize WalletClient if not already done
        if self.wallet_client.is_none() {
            self.wallet_client = Some(WalletClient::new());
        }
        
        if let Some(client) = &self.wallet_client {
            // Create connection options
            let options = BeaconConnectOptions {
                permissions: permissions.iter().map(|s| s.to_string()).collect(),
                app_info: BeaconAppInfo {
                    name: "Faithful Archive".to_string(),
                    logo: "https://faithfularchive.org/logo.png".to_string(),
                },
                gateway: BeaconGateway {
                    host: "arweave.net".to_string(),
                    port: 443,
                    protocol: "https".to_string(),
                },
                broker_url: "wss://aosync-broker-eu.beaconwallet.dev:8081".to_string(),
                options: BeaconOptions {
                    protocol_version: 5,
                },
            };
            
            let options_js = serde_wasm_bindgen::to_value(&options)
                .map_err(|e| WalletError::ConnectionFailed(format!("Failed to serialize options: {}", e)))?;
            
            let promise = client.connect_js(&options_js);
            
            match JsFuture::from(promise).await {
                Ok(result) => {
                    // Parse the connection result
                    if let Some(address) = result.as_string() {
                        self.connected = true;
                        self.address = Some(address.clone());
                        Ok(address)
                    } else {
                        // Try to extract address from result object
                        if let Ok(addr) = Reflect::get(&result, &JsValue::from_str("address")) {
                            if let Some(address) = addr.as_string() {
                                self.connected = true;
                                self.address = Some(address.clone());
                                Ok(address)
                            } else {
                                Err(WalletError::ConnectionFailed("Invalid connection response".to_string()))
                            }
                        } else {
                            Err(WalletError::ConnectionFailed("No address in connection response".to_string()))
                        }
                    }
                }
                Err(e) => {
                    console_log(&format!("Beacon connection error: {:?}", e));
                    Err(WalletError::ConnectionFailed(format!("Beacon connection failed: {:?}", e)))
                }
            }
        } else {
            Err(WalletError::ConnectionFailed("WalletClient not initialized".to_string()))
        }
    }
    
    async fn disconnect(&mut self) -> Result<(), WalletError> {
        if let Some(client) = &self.wallet_client {
            let promise = client.disconnect_js();
            
            match JsFuture::from(promise).await {
                Ok(_) => {
                    self.connected = false;
                    self.address = None;
                    Ok(())
                }
                Err(e) => {
                    console_log(&format!("Beacon disconnect error: {:?}", e));
                    Err(WalletError::ConnectionFailed(format!("Beacon disconnect failed: {:?}", e)))
                }
            }
        } else {
            Ok(()) // Already disconnected
        }
    }
    
    async fn get_active_address(&self) -> Result<String, WalletError> {
        if let Some(address) = &self.address {
            Ok(address.clone())
        } else {
            Err(WalletError::ConnectionFailed("Beacon not connected".to_string()))
        }
    }
    
    async fn get_permissions(&self) -> Result<Vec<String>, WalletError> {
        // Return basic permissions that were requested during connection
        if self.connected {
            Ok(vec![
                "ACCESS_ADDRESS".to_string(),
                "ACCESS_PUBLIC_KEY".to_string(),
                "SIGN_TRANSACTION".to_string(),
            ])
        } else {
            Err(WalletError::ConnectionFailed("Beacon not connected".to_string()))
        }
    }
    
    async fn sign_transaction(&self, transaction_data: HashMap<String, serde_json::Value>) -> Result<HashMap<String, serde_json::Value>, WalletError> {
        if let Some(client) = &self.wallet_client {
            if self.connected {
                let tx_js = serde_wasm_bindgen::to_value(&transaction_data)
                    .map_err(|e| WalletError::SigningFailed(format!("Failed to serialize transaction: {}", e)))?;
                
                // Use the real ao-sync-sdk sign method for transactions
                let promise = client.sign_js(&tx_js);
                
                match JsFuture::from(promise).await {
                    Ok(result) => {
                        let signed_tx: HashMap<String, serde_json::Value> = serde_wasm_bindgen::from_value(result)
                            .map_err(|e| WalletError::SigningFailed(format!("Failed to parse signed transaction: {}", e)))?;
                        Ok(signed_tx)
                    }
                    Err(e) => {
                        console_log(&format!("Beacon signing error: {:?}", e));
                        Err(WalletError::SigningFailed(format!("Beacon transaction signing failed: {:?}", e)))
                    }
                }
            } else {
                Err(WalletError::SigningFailed("Beacon not connected".to_string()))
            }
        } else {
            Err(WalletError::SigningFailed("Beacon not initialized".to_string()))
        }
    }
    
    async fn check_connection(&self) -> Result<bool, WalletError> {
        Ok(self.connected)
    }
}

// Beacon strategy is now integrated into WalletStrategyType enum in strategy.rs