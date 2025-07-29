use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use anyhow::Result;
use crate::services::wallet::{WalletError, WalletState};

/// Supported wallet connection strategies
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum WalletStrategyType {
    /// Wander wallet (formerly ArConnect) - browser extension
    Wander,
    /// Beacon wallet - iOS-based agent-first wallet for AO
    Beacon,
    /// ArweaveWalletKit - unified wallet connection library
    WalletKit,
    /// ArweaveWebWallet - web-based wallet connection
    WebWallet,
}

impl WalletStrategyType {
    pub fn display_name(&self) -> &'static str {
        match self {
            WalletStrategyType::Wander => "Wander",
            WalletStrategyType::Beacon => "Beacon",
            WalletStrategyType::WalletKit => "Arweave Wallet Kit", 
            WalletStrategyType::WebWallet => "Web Wallet",
        }
    }
    
    pub fn description(&self) -> &'static str {
        match self {
            WalletStrategyType::Wander => "Non-custodial Arweave & AO wallet for your favorite browser",
            WalletStrategyType::Beacon => "iOS based agent first wallet for AO",
            WalletStrategyType::WalletKit => "Universal wallet connection library",
            WalletStrategyType::WebWallet => "Web-based wallet connection",
        }
    }
    
    pub fn requires_extension(&self) -> bool {
        match self {
            WalletStrategyType::Wander => true,
            WalletStrategyType::Beacon => false,
            WalletStrategyType::WalletKit => false,
            WalletStrategyType::WebWallet => false,
        }
    }
}

/// Wallet capability flags
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WalletCapabilities {
    pub can_sign_transactions: bool,
    pub can_encrypt_data: bool,
    pub can_decrypt_data: bool,
    pub supports_batch_signing: bool,
    pub supports_permissions: bool,
    pub supports_multiple_addresses: bool,
}

impl Default for WalletCapabilities {
    fn default() -> Self {
        Self {
            can_sign_transactions: true,
            can_encrypt_data: false,
            can_decrypt_data: false,
            supports_batch_signing: false,
            supports_permissions: true,
            supports_multiple_addresses: false,
        }
    }
}

/// Extended wallet state with strategy information
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExtendedWalletState {
    pub base_state: WalletState,
    pub strategy: WalletStrategyType,
    pub capabilities: WalletCapabilities,
    pub available_strategies: Vec<WalletStrategyType>,
}

impl Default for ExtendedWalletState {
    fn default() -> Self {
        Self {
            base_state: WalletState::default(),
            strategy: WalletStrategyType::Wander,
            capabilities: WalletCapabilities::default(),
            available_strategies: vec![],
        }
    }
}

/// Wallet connection strategy trait
#[async_trait(?Send)]
pub trait WalletStrategy {
    /// Get the strategy type
    fn strategy_type(&self) -> WalletStrategyType;
    
    /// Check if this wallet strategy is available in the current environment
    async fn is_available(&self) -> Result<bool, WalletError>;
    
    /// Get wallet capabilities
    fn get_capabilities(&self) -> WalletCapabilities;
    
    /// Connect to the wallet with specified permissions
    async fn connect(&mut self, permissions: Vec<&str>) -> Result<String, WalletError>;
    
    /// Disconnect from the wallet
    async fn disconnect(&mut self) -> Result<(), WalletError>;
    
    /// Get the currently active address
    async fn get_active_address(&self) -> Result<String, WalletError>;
    
    /// Get current permissions
    async fn get_permissions(&self) -> Result<Vec<String>, WalletError>;
    
    /// Sign a transaction
    async fn sign_transaction(&self, transaction_data: HashMap<String, serde_json::Value>) -> Result<HashMap<String, serde_json::Value>, WalletError>;
    
    /// Check current connection status
    async fn check_connection(&self) -> Result<bool, WalletError>;
    
    /// Optional: Get all available addresses (for multi-address wallets)
    async fn get_all_addresses(&self) -> Result<Vec<String>, WalletError> {
        // Default implementation returns single address
        match self.get_active_address().await {
            Ok(address) => Ok(vec![address]),
            Err(e) => Err(e),
        }
    }
    
    /// Optional: Encrypt data with wallet
    async fn encrypt(&self, _data: &[u8], _options: Option<HashMap<String, String>>) -> Result<Vec<u8>, WalletError> {
        Err(WalletError::InvalidPermissions)
    }
    
    /// Optional: Decrypt data with wallet
    async fn decrypt(&self, _data: &[u8], _options: Option<HashMap<String, String>>) -> Result<Vec<u8>, WalletError> {
        Err(WalletError::InvalidPermissions)
    }
}

/// Wallet strategy manager
pub struct WalletStrategyManager {
    strategies: HashMap<WalletStrategyType, Box<dyn WalletStrategy>>,
    current_strategy: Option<WalletStrategyType>,
}

impl WalletStrategyManager {
    pub fn new() -> Self {
        Self {
            strategies: HashMap::new(),
            current_strategy: None,
        }
    }
    
    /// Get the number of registered strategies
    pub fn strategy_count(&self) -> usize {
        self.strategies.len()
    }
    
    /// Register a wallet strategy
    pub fn register_strategy(&mut self, strategy: Box<dyn WalletStrategy>) {
        let strategy_type = strategy.strategy_type();
        self.strategies.insert(strategy_type, strategy);
    }
    
    /// Get available wallet strategies
    pub async fn get_available_strategies(&self) -> Vec<WalletStrategyType> {
        let mut available = Vec::new();
        
        log::info!("🔍 Checking availability of {} registered strategies", self.strategies.len());
        
        for (strategy_type, strategy) in &self.strategies {
            log::info!("🧪 Testing strategy: {:?}", strategy_type);
            
            match strategy.is_available().await {
                Ok(true) => {
                    log::info!("✅ Strategy {:?} is available", strategy_type);
                    available.push(*strategy_type);
                }
                Ok(false) => {
                    log::info!("❌ Strategy {:?} is not available", strategy_type);
                }
                Err(e) => {
                    log::warn!("⚠️ Error checking strategy {:?}: {}", strategy_type, e);
                }
            }
        }
        
        log::info!("📋 Final available strategies: {:?}", available);
        available
    }
    
    /// Set the active wallet strategy
    pub fn set_strategy(&mut self, strategy_type: WalletStrategyType) -> Result<(), WalletError> {
        if self.strategies.contains_key(&strategy_type) {
            self.current_strategy = Some(strategy_type);
            Ok(())
        } else {
            Err(WalletError::ConnectionFailed(format!("Strategy {:?} not registered", strategy_type)))
        }
    }
    
    /// Get the current strategy
    pub fn get_current_strategy(&self) -> Option<&dyn WalletStrategy> {
        self.current_strategy
            .and_then(|strategy_type| self.strategies.get(&strategy_type))
            .map(|boxed_strategy| boxed_strategy.as_ref())
    }
    
    /// Execute operation with current strategy mutably
    pub async fn with_current_strategy_mut<F, R>(&mut self, f: F) -> Result<R, WalletError>
    where
        F: FnOnce(&mut dyn WalletStrategy) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<R, WalletError>> + '_>>,
    {
        let strategy_type = self.current_strategy.ok_or(WalletError::NotInstalled)?;
        let strategy = self.strategies.get_mut(&strategy_type)
            .ok_or(WalletError::NotInstalled)?;
        f(strategy.as_mut()).await
    }
    
    /// Get a specific strategy
    pub fn get_strategy(&self, strategy_type: WalletStrategyType) -> Option<&dyn WalletStrategy> {
        self.strategies.get(&strategy_type)
            .map(|boxed_strategy| boxed_strategy.as_ref())
    }
    
    /// Execute operation with specific strategy mutably
    pub async fn with_strategy_mut<F, R>(&mut self, strategy_type: WalletStrategyType, f: F) -> Result<R, WalletError>
    where
        F: FnOnce(&mut dyn WalletStrategy) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<R, WalletError>> + '_>>,
    {
        let strategy = self.strategies.get_mut(&strategy_type)
            .ok_or(WalletError::ConnectionFailed(format!("Strategy {:?} not found", strategy_type)))?;
        f(strategy.as_mut()).await
    }
    
    /// Auto-select the best available strategy
    pub async fn auto_select_strategy(&mut self) -> Result<WalletStrategyType, WalletError> {
        let available = self.get_available_strategies().await;
        
        if available.is_empty() {
            return Err(WalletError::NotInstalled);
        }
        
        // Priority order: Wander > Beacon > WalletKit > WebWallet
        let preferred_order = vec![
            WalletStrategyType::Wander,
            WalletStrategyType::Beacon,
            WalletStrategyType::WalletKit,
            WalletStrategyType::WebWallet,
        ];
        
        for preferred in preferred_order {
            if available.contains(&preferred) {
                self.set_strategy(preferred)?;
                return Ok(preferred);
            }
        }
        
        // Fallback to first available
        let first_available = available[0];
        self.set_strategy(first_available)?;
        Ok(first_available)
    }
}

impl Default for WalletStrategyManager {
    fn default() -> Self {
        Self::new()
    }
}