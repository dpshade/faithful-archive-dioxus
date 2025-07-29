use dioxus::prelude::*;
use std::collections::HashMap;
use crate::services::wallet::{
    WalletService, WalletStrategyType, WalletError, ExtendedWalletState,
    WalletCapabilities
};

/// Context for wallet state and operations
/// 
/// This provides a centralized way to manage wallet state across your entire
/// Dioxus application. The context includes the wallet service instance,
/// current state, and common operations.
#[derive(Clone)]
pub struct WalletContext {
    /// The wallet service instance
    pub service: Signal<WalletService>,
    /// Extended wallet state
    pub state: Signal<ExtendedWalletState>,
    /// Connection operations
    pub connect: Callback<(), Result<String, WalletError>>,
    pub disconnect: Callback<(), Result<(), WalletError>>,
    /// Strategy management
    pub set_strategy: Callback<WalletStrategyType, Result<(), WalletError>>,
    pub get_available_strategies: Callback<(), Vec<WalletStrategyType>>,
    /// Transaction operations
    pub sign_transaction: Callback<HashMap<String, serde_json::Value>, Result<HashMap<String, serde_json::Value>, WalletError>>,
    /// Utility functions
    pub format_address: fn(&str) -> String,
}

/// Props for the wallet context provider
#[derive(Props, Clone, PartialEq)]
pub struct WalletProviderProps {
    /// Child components
    children: Element,
    /// Optional initial strategy preference
    #[props(default)]
    initial_strategy: Option<WalletStrategyType>,
    /// Whether to auto-connect on mount if a session exists
    #[props(default = true)]
    auto_reconnect: bool,
}

/// Wallet context provider component
/// 
/// Wrap your application or components with this provider to give them access
/// to wallet functionality through the `use_wallet_context` hook.
/// 
/// # Example
/// 
/// ```rust
/// use dioxus::prelude::*;
/// use faithful_archive::services::wallet::{WalletProvider, use_wallet_context};
/// 
/// #[component]
/// fn App() -> Element {
///     rsx! {
///         WalletProvider {
///             auto_reconnect: true,
///             
///             Router::<Route> {}
///         }
///     }
/// }
/// 
/// #[component]
/// fn MyComponent() -> Element {
///     let wallet = use_wallet_context();
///     
///     rsx! {
///         button {
///             onclick: move |_| {
///                 spawn(async move {
///                     if let Err(e) = wallet.connect.call(()).await {
///                         log::error!("Connection failed: {}", e);
///                     }
///                 });
///             },
///             "Connect Wallet"
///         }
///     }
/// }
/// ```
#[component]
pub fn WalletProvider(props: WalletProviderProps) -> Element {
    let mut wallet_service = use_signal(|| WalletService::new());
    let mut wallet_state = use_signal(|| ExtendedWalletState::default());
    
    // Initialize wallet service
    use_effect(move || {
        spawn(async move {
            let mut service = WalletService::init().await;
            
            // Set initial strategy if provided
            if let Some(initial_strategy) = props.initial_strategy {
                if let Err(e) = service.set_strategy(initial_strategy).await {
                    log::warn!("Failed to set initial strategy: {}", e);
                }
            }
            
            // Auto-reconnect if enabled
            if props.auto_reconnect {
                // Check for existing session or connection state
                // This would depend on your persistence strategy
                if let Ok(connected) = service.check_connection().await {
                    if connected {
                        let _ = service.connect().await;
                    }
                }
            }
            
            wallet_service.set(service);
        });
    });
    
    // Sync wallet state with service state
    use_effect(move || {
        let service_state = WalletService::get_extended_state();
        wallet_state.set(service_state());
    });
    
    // Connect callback
    let connect = use_callback({
        let wallet_service = wallet_service.clone();
        move |_: ()| {
            let mut wallet_service = wallet_service.clone();
            // Spawn async task and return placeholder result
            spawn(async move {
                // Use a different approach - create a temporary service for the async call
                let mut temp_service = WalletService::new();
                let _ = temp_service.connect().await;
                // Update the main service after the async operation
                wallet_service.set(temp_service);
            });
            Ok("connecting".to_string())
        }
    });
    
    // Disconnect callback
    let disconnect = use_callback({
        let wallet_service = wallet_service.clone();
        move |_: ()| {
            let mut wallet_service = wallet_service.clone();
            // Spawn async task and return placeholder result
            spawn(async move {
                // Use a different approach - create a temporary service for the async call
                let mut temp_service = WalletService::new();
                let _ = temp_service.disconnect().await;
                // Update the main service after the async operation
                wallet_service.set(temp_service);
            });
            Ok(())
        }
    });
    
    // Set strategy callback
    let set_strategy = use_callback({
        let wallet_service = wallet_service.clone();
        move |strategy: WalletStrategyType| {
            let mut wallet_service = wallet_service.clone();
            // Spawn async task and return placeholder result
            spawn(async move {
                // Use a different approach - create a temporary service for the async call
                let mut temp_service = WalletService::new();
                let _ = temp_service.set_strategy(strategy).await;
                // Update the main service after the async operation
                wallet_service.set(temp_service);
            });
            Ok(())
        }
    });
    
    // Get available strategies callback
    let get_available_strategies = use_callback({
        let wallet_state = wallet_state.clone();
        move |_: ()| {
            wallet_state.read().available_strategies.clone()
        }
    });
    
    // Sign transaction callback
    let sign_transaction = use_callback({
        let wallet_service = wallet_service.clone();
        move |transaction_data: HashMap<String, serde_json::Value>| {
            let wallet_service = wallet_service.clone();
            // Spawn async task and return placeholder result
            spawn(async move {
                // Use a different approach - create a temporary service for the async call
                let temp_service = WalletService::new();
                let _ = temp_service.sign_transaction(transaction_data.clone()).await;
            });
            Ok(HashMap::new())
        }
    });
    
    let wallet_context = WalletContext {
        service: wallet_service,
        state: wallet_state,
        connect,
        disconnect,
        set_strategy,
        get_available_strategies,
        sign_transaction,
        format_address: WalletService::format_address,
    };
    
    use_context_provider(|| wallet_context);
    
    rsx! {
        {props.children}
    }
}

/// Hook to access wallet context
/// 
/// This hook provides access to the wallet context from any component that is
/// a child of `WalletProvider`. It returns the full wallet context with all
/// operations and state.
/// 
/// # Panics
/// 
/// This hook will panic if called outside of a `WalletProvider`.
/// 
/// # Example
/// 
/// ```rust
/// use dioxus::prelude::*;
/// use faithful_archive::services::wallet::use_wallet_context;
/// 
/// #[component]
/// fn WalletInfo() -> Element {
///     let wallet = use_wallet_context();
///     let state = wallet.state.read();
///     
///     rsx! {
///         div {
///             if state.base_state.connected {
///                 p { "Connected to: {wallet.format_address(state.base_state.address.as_ref().unwrap())}" }
///                 p { "Strategy: {state.strategy.display_name()}" }
///             } else {
///                 p { "Not connected" }
///             }
///         }
///     }
/// }
/// ```
pub fn use_wallet_context() -> WalletContext {
    use_context::<WalletContext>()
}

/// Hook for wallet connection state only
/// 
/// A simpler hook that returns just the connection-related state.
/// Useful when you only need to check if a wallet is connected.
pub fn use_wallet_connection() -> (bool, Option<String>) {
    let wallet = use_wallet_context();
    let state = wallet.state.read();
    (state.base_state.connected, state.base_state.address.clone())
}

/// Hook for wallet operations only
/// 
/// Returns just the operation callbacks without state.
/// Useful for components that trigger wallet actions.
pub fn use_wallet_operations() -> WalletOperations {
    let wallet = use_wallet_context();
    
    WalletOperations {
        connect: wallet.connect,
        disconnect: wallet.disconnect,
        set_strategy: wallet.set_strategy,
        sign_transaction: wallet.sign_transaction,
    }
}

/// Simplified wallet operations interface
#[derive(Clone)]
pub struct WalletOperations {
    pub connect: Callback<(), Result<String, WalletError>>,
    pub disconnect: Callback<(), Result<(), WalletError>>,
    pub set_strategy: Callback<WalletStrategyType, Result<(), WalletError>>,
    pub sign_transaction: Callback<HashMap<String, serde_json::Value>, Result<HashMap<String, serde_json::Value>, WalletError>>,
}

/// Hook for wallet capabilities
/// 
/// Returns the current wallet strategy's capabilities.
/// Useful for conditionally showing UI elements based on wallet features.
pub fn use_wallet_capabilities() -> WalletCapabilities {
    let wallet = use_wallet_context();
    let state = wallet.state.read();
    state.capabilities.clone()
}

/// Hook for wallet strategy management
/// 
/// Returns current strategy and available strategies with a setter.
/// Useful for strategy selection components.
pub fn use_wallet_strategies() -> (WalletStrategyType, Vec<WalletStrategyType>, Callback<WalletStrategyType, Result<(), WalletError>>) {
    let wallet = use_wallet_context();
    let state = wallet.state.read();
    
    (
        state.strategy,
        state.available_strategies.clone(),
        wallet.set_strategy,
    )
}

/// Error boundary component for wallet operations
/// 
/// Catches and displays wallet-related errors in a user-friendly way.
#[component]
pub fn WalletErrorBoundary(
    children: Element,
    #[props(default)] fallback: Option<Element>,
) -> Element {
    let wallet = use_wallet_context();
    let state = wallet.state.read();
    
    if let Some(error) = &state.base_state.error {
        if let Some(fallback_ui) = fallback {
            return fallback_ui;
        }
        
        rsx! {
            div {
                class: "wallet-error-boundary bg-red-50 dark:bg-red-900/20 border border-red-200 dark:border-red-800 rounded-lg p-4 my-4",
                
                div {
                    class: "flex items-start",
                    
                    svg {
                        class: "w-5 h-5 text-red-400 mt-0.5 mr-3 flex-shrink-0",
                        fill: "currentColor",
                        view_box: "0 0 20 20",
                        
                        path {
                            fill_rule: "evenodd",
                            d: "M18 10a8 8 0 11-16 0 8 8 0 0116 0zm-7 4a1 1 0 11-2 0 1 1 0 012 0zm-1-9a1 1 0 00-1 1v4a1 1 0 102 0V6a1 1 0 00-1-1z",
                            clip_rule: "evenodd",
                        }
                    }
                    
                    div {
                        h3 {
                            class: "text-sm font-medium text-red-800 dark:text-red-200",
                            "Wallet Error"
                        }
                        
                        p {
                            class: "mt-1 text-sm text-red-700 dark:text-red-300",
                            "{error}"
                        }
                        
                        button {
                            class: "mt-2 text-sm text-red-600 dark:text-red-400 hover:text-red-500 dark:hover:text-red-300 underline",
                            onclick: move |_| {
                                // Clear error by attempting reconnection or reset
                                spawn(async move {
                                    let _ = wallet.disconnect.call(());
                                });
                            },
                            "Retry Connection"
                        }
                    }
                }
            }
        }
    } else {
        children
    }
}

/// Wrapper component that conditionally renders content based on wallet connection
#[component]
pub fn WalletGated(
    children: Element,
    #[props(default)] fallback: Option<Element>,
    #[props(default = false)] require_specific_strategy: bool,
    #[props(default)] required_strategy: Option<WalletStrategyType>,
) -> Element {
    let (connected, _) = use_wallet_connection();
    let wallet = use_wallet_context();
    let state = wallet.state.read();
    
    let should_show = if require_specific_strategy {
        connected && required_strategy.map_or(true, |rs| rs == state.strategy)
    } else {
        connected
    };
    
    if should_show {
        children
    } else if let Some(fallback_ui) = fallback {
        fallback_ui
    } else {
        rsx! {
            div {
                class: "wallet-gated bg-yellow-50 dark:bg-yellow-900/20 border border-yellow-200 dark:border-yellow-800 rounded-lg p-4 text-center",
                
                svg {
                    class: "mx-auto h-8 w-8 text-yellow-400 mb-2",
                    fill: "none",
                    stroke: "currentColor",
                    view_box: "0 0 24 24",
                    
                    path {
                        stroke_linecap: "round",
                        stroke_linejoin: "round",
                        stroke_width: "2",
                        d: "M12 15v2m-6 4h12a2 2 0 002-2v-6a2 2 0 00-2-2H6a2 2 0 00-2 2v6a2 2 0 002 2zm10-10V7a4 4 0 00-8 0v4h8z",
                    }
                }
                
                h3 {
                    class: "text-lg font-medium text-yellow-800 dark:text-yellow-200 mb-2",
                    if require_specific_strategy {
                        "Specific Wallet Required"
                    } else {
                        "Wallet Connection Required"
                    }
                }
                
                p {
                    class: "text-sm text-yellow-700 dark:text-yellow-300 mb-4",
                    if require_specific_strategy {
                        "This feature requires a connection with {required_strategy.unwrap_or(WalletStrategyType::Beacon).display_name()}."
                    } else {
                        "Please connect your wallet to access this feature."
                    }
                }
                
                button {
                    class: "inline-flex items-center px-4 py-2 border border-transparent text-sm font-medium rounded-md text-yellow-700 bg-yellow-100 hover:bg-yellow-200 dark:bg-yellow-800 dark:text-yellow-200 dark:hover:bg-yellow-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-yellow-500",
                    onclick: move |_| {
                        spawn(async move {
                            let _ = wallet.connect.call(());
                        });
                    },
                    "Connect Wallet"
                }
            }
        }
    }
}