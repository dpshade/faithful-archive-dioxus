use dioxus::prelude::*;
use crate::services::wallet::{
    WalletService, WalletStrategyType, WalletError, 
    ExtendedWalletState, WalletCapabilities
};

/// Props for the main wallet connect component
#[derive(Clone, PartialEq, Props)]
pub struct WalletConnectProps {
    /// Optional custom CSS class for styling
    #[props(default = "")]
    pub class: &'static str,
    
    /// Whether to show strategy selector
    #[props(default = true)]
    pub show_strategy_selector: bool,
    
    /// Whether to show connection status
    #[props(default = true)]
    pub show_status: bool,
    
    /// Whether to show address when connected
    #[props(default = true)]
    pub show_address: bool,
    
    /// Custom button text for connect state
    #[props(default = "Connect Wallet")]
    pub connect_text: &'static str,
    
    /// Custom button text for disconnect state
    #[props(default = "Disconnect")]
    pub disconnect_text: &'static str,
    
    /// Size variant for the component
    #[props(default = WalletConnectSize::Medium)]
    pub size: WalletConnectSize,
    
    /// Variant/style for the component
    #[props(default = WalletConnectVariant::Primary)]
    pub variant: WalletConnectVariant,
    
    /// Optional callback when connection state changes
    pub on_connection_change: Option<EventHandler<ConnectionChangeEvent>>,
    
    /// Optional callback when strategy changes
    pub on_strategy_change: Option<EventHandler<WalletStrategyType>>,
}

#[derive(Clone, PartialEq)]
pub enum WalletConnectSize {
    Small,
    Medium, 
    Large,
}

#[derive(Clone, PartialEq)]
pub enum WalletConnectVariant {
    Primary,
    Secondary,
    Outline,
    Ghost,
}

#[derive(Clone, PartialEq)]
pub struct ConnectionChangeEvent {
    pub connected: bool,
    pub address: Option<String>,
    pub strategy: WalletStrategyType,
}

/// Main composable wallet connect component
/// 
/// This component provides a complete wallet connection interface that can be easily
/// embedded in any Dioxus application. It supports multiple wallet strategies
/// and provides extensive customization options.
/// 
/// # Example Usage
/// 
/// ```rust
/// use dioxus::prelude::*;
/// use faithful_archive::components::WalletConnect;
/// 
/// #[component]
/// fn MyApp() -> Element {
///     rsx! {
///         WalletConnect {
///             class: "my-custom-wallet-button",
///             show_strategy_selector: true,
///             variant: WalletConnectVariant::Primary,
///             on_connection_change: |event| {
///                 println!("Wallet connection changed: {}", event.connected);
///             }
///         }
///     }
/// }
/// ```
#[component]
pub fn WalletConnect(props: WalletConnectProps) -> Element {
    let extended_state = WalletService::get_extended_state();
    let mut wallet_service = use_signal(|| WalletService::new());
    
    // Initialize wallet service on mount
    use_effect(move || {
        spawn(async move {
            let service = WalletService::init().await;
            wallet_service.set(service);
        });
    });
    
    // Connection handler
    let connect_handler = {
        let mut wallet_service = wallet_service.clone();
        let on_connection_change = props.on_connection_change.clone();
        move |_| {
            let mut wallet_service = wallet_service.clone();
            let on_connection_change = on_connection_change.clone();
            spawn(async move {
                let state = extended_state();
                let result = if state.base_state.connected {
                    let mut temp_service = WalletService::new();
                    let res = temp_service.disconnect().await;
                    wallet_service.set(temp_service);
                    res
                } else {
                    let mut temp_service = WalletService::new();
                    let res = temp_service.connect().await.map(|_| ());
                    wallet_service.set(temp_service);
                    res
                };
                
                // Trigger callback if provided
                if let Some(callback) = on_connection_change {
                    let new_state = extended_state();
                    callback.call(ConnectionChangeEvent {
                        connected: new_state.base_state.connected,
                        address: new_state.base_state.address.clone(),
                        strategy: new_state.strategy,
                    });
                }
                
                if let Err(e) = result {
                    web_sys::console::log_1(&format!("Wallet operation failed: {}", e).into());
                }
            });
        }
    };
    
    let base_class = format!("wallet-connect {}", props.class);
    let state = extended_state();
    
    rsx! {
        div {
            class: "{base_class}",
            
            // Strategy selector (if enabled)
            if props.show_strategy_selector && !state.available_strategies.is_empty() {
                WalletStrategySelector {
                    current_strategy: state.strategy,
                    available_strategies: state.available_strategies.clone(),
                    on_strategy_change: props.on_strategy_change.clone(),
                    wallet_service: wallet_service.clone(),
                }
            }
            
            // Main connect/disconnect button
            WalletConnectButton {
                state: state.clone(),
                connect_text: props.connect_text,
                disconnect_text: props.disconnect_text,
                size: props.size.clone(),
                variant: props.variant.clone(),
                onclick: connect_handler,
            }
            
            // Connection status and address display
            if props.show_status {
                WalletStatus {
                    state: state.clone(),
                    show_address: props.show_address,
                    size: props.size.clone(),
                }
            }
        }
    }
}

/// Strategy selector component
#[component]
fn WalletStrategySelector(
    current_strategy: WalletStrategyType,
    available_strategies: Vec<WalletStrategyType>,
    on_strategy_change: Option<EventHandler<WalletStrategyType>>,
    wallet_service: Signal<WalletService>,
) -> Element {
    let strategy_change_handler = {
        let mut wallet_service = wallet_service.clone();
        let on_strategy_change = on_strategy_change.clone();
        move |evt: Event<FormData>| {
            let strategy_str = evt.value();
            if let Ok(strategy) = strategy_str.parse::<WalletStrategyType>() {
                let mut wallet_service = wallet_service.clone();
                let on_strategy_change = on_strategy_change.clone();
                spawn(async move {
                    let mut temp_service = WalletService::new();
                    if let Err(e) = temp_service.set_strategy(strategy).await {
                        web_sys::console::log_1(&format!("Failed to set strategy: {}", e).into());
                    } else {
                        wallet_service.set(temp_service);
                        if let Some(callback) = on_strategy_change {
                            callback.call(strategy);
                        }
                    }
                });
            }
        }
    };
    
    rsx! {
        div {
            class: "wallet-strategy-selector mb-3",
            
            label {
                class: "block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1",
                "Wallet Strategy"
            }
            
            select {
                class: "w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md shadow-sm bg-white dark:bg-gray-800 text-gray-900 dark:text-gray-100 focus:outline-none focus:ring-2 focus:ring-green-500 focus:border-green-500",
                value: "{current_strategy}",
                onchange: strategy_change_handler,
                
                for strategy in available_strategies {
                    option {
                        value: "{strategy}",
                        selected: strategy == current_strategy,
                        "{strategy.display_name()}"
                    }
                }
            }
        }
    }
}

/// Main connect/disconnect button component
#[component]
fn WalletConnectButton(
    state: ExtendedWalletState,
    connect_text: &'static str,
    disconnect_text: &'static str,
    size: WalletConnectSize,
    variant: WalletConnectVariant,
    onclick: EventHandler<MouseEvent>,
) -> Element {
    let button_text = if state.base_state.connecting {
        "Connecting..."
    } else if state.base_state.connected {
        disconnect_text
    } else if !state.base_state.available {
        "No Wallet Available"
    } else {
        connect_text
    };
    
    let size_classes = match size {
        WalletConnectSize::Small => "px-3 py-1.5 text-sm",
        WalletConnectSize::Medium => "px-4 py-2 text-base",
        WalletConnectSize::Large => "px-6 py-3 text-lg",
    };
    
    let variant_classes = match variant {
        WalletConnectVariant::Primary => {
            if state.base_state.connected {
                "bg-red-600 hover:bg-red-700 text-white border-red-600"
            } else if !state.base_state.available {
                "bg-gray-400 text-white border-gray-400 cursor-not-allowed"
            } else {
                "bg-green-600 hover:bg-green-700 text-white border-green-600"
            }
        },
        WalletConnectVariant::Secondary => {
            if state.base_state.connected {
                "bg-gray-600 hover:bg-gray-700 text-white border-gray-600"
            } else if !state.base_state.available {
                "bg-gray-300 text-gray-500 border-gray-300 cursor-not-allowed"
            } else {
                "bg-blue-600 hover:bg-blue-700 text-white border-blue-600"
            }
        },
        WalletConnectVariant::Outline => {
            if state.base_state.connected {
                "border-red-600 text-red-600 hover:bg-red-50 dark:hover:bg-red-900/20"
            } else if !state.base_state.available {
                "border-gray-300 text-gray-400 cursor-not-allowed"
            } else {
                "border-green-600 text-green-600 hover:bg-green-50 dark:hover:bg-green-900/20"
            }
        },
        WalletConnectVariant::Ghost => {
            if state.base_state.connected {
                "text-red-600 hover:bg-red-50 dark:hover:bg-red-900/20"
            } else if !state.base_state.available {
                "text-gray-400 cursor-not-allowed"
            } else {
                "text-green-600 hover:bg-green-50 dark:hover:bg-green-900/20"
            }
        },
    };
    
    let base_classes = "inline-flex items-center justify-center font-medium rounded-lg border transition-colors duration-200 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-green-500 disabled:opacity-50 disabled:cursor-not-allowed";
    let button_class = format!("{} {} {}", base_classes, size_classes, variant_classes);
    
    rsx! {
        button {
            class: "{button_class}",
            disabled: state.base_state.connecting || !state.base_state.available,
            onclick: move |evt| onclick.call(evt),
            
            // Loading spinner for connecting state
            if state.base_state.connecting {
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
                        stroke_width: "4",
                    }
                    
                    path {
                        class: "opacity-75",
                        fill: "currentColor",
                        d: "M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z",
                    }
                }
            }
            
            "{button_text}"
        }
    }
}

/// Wallet status and information component
#[component]
fn WalletStatus(
    state: ExtendedWalletState,
    show_address: bool,
    size: WalletConnectSize,
) -> Element {
    let text_size = match size {
        WalletConnectSize::Small => "text-xs",
        WalletConnectSize::Medium => "text-sm",
        WalletConnectSize::Large => "text-base",
    };
    
    rsx! {
        div {
            class: "wallet-status mt-2 space-y-1",
            
            // Connected address
            if state.base_state.connected && show_address {
                if let Some(address) = state.base_state.address.clone() {
                    div {
                        class: "flex items-center space-x-2 {text_size} text-gray-600 dark:text-gray-400",
                        
                        // Status indicator
                        div {
                            class: "w-2 h-2 bg-green-500 rounded-full flex-shrink-0",
                        }
                        
                        span {
                            "Connected: "
                        }
                        
                        code {
                            class: "bg-gray-100 dark:bg-gray-800 px-1 py-0.5 rounded font-mono",
                            "{WalletService::format_address(&address)}"
                        }
                        
                        // Copy button
                        button {
                            class: "ml-1 p-1 hover:bg-gray-200 dark:hover:bg-gray-700 rounded",
                            onclick: move |_| {
                                if let Some(window) = web_sys::window() {
                                    let navigator = window.navigator();
                                    let clipboard = navigator.clipboard();
                                    let address = address.clone();
                                    spawn(async move {
                                        let _ = wasm_bindgen_futures::JsFuture::from(
                                            clipboard.write_text(&address)
                                        ).await;
                                    });
                                }
                            },
                            title: "Copy address",
                            
                            svg {
                                class: "w-3 h-3",
                                fill: "none",
                                stroke: "currentColor",
                                view_box: "0 0 24 24",
                                
                                path {
                                    stroke_linecap: "round",
                                    stroke_linejoin: "round",
                                    stroke_width: "2",
                                    d: "M8 16H6a2 2 0 01-2-2V6a2 2 0 012-2h8a2 2 0 012 2v2m-6 12h8a2 2 0 002-2v-8a2 2 0 00-2-2h-8a2 2 0 00-2 2v8a2 2 0 002 2z",
                                }
                            }
                        }
                    }
                }
            }
            
            // Current strategy info
            div {
                class: "flex items-center space-x-2 {text_size} text-gray-500 dark:text-gray-500",
                
                span {
                    "Strategy: {state.strategy.display_name()}"
                }
                
                // Capabilities indicator
                if state.capabilities.can_sign_transactions {
                    span {
                        class: "inline-flex items-center px-1.5 py-0.5 rounded-full text-xs bg-green-100 text-green-800 dark:bg-green-900 dark:text-green-200",
                        "Signing"
                    }
                }
            }
            
            // Error display
            if let Some(error) = &state.base_state.error {
                div {
                    class: "flex items-start space-x-2 {text_size} text-red-600 dark:text-red-400",
                    
                    svg {
                        class: "w-4 h-4 mt-0.5 flex-shrink-0",
                        fill: "currentColor",
                        view_box: "0 0 20 20",
                        
                        path {
                            fill_rule: "evenodd",
                            d: "M18 10a8 8 0 11-16 0 8 8 0 0116 0zm-7 4a1 1 0 11-2 0 1 1 0 012 0zm-1-9a1 1 0 00-1 1v4a1 1 0 102 0V6a1 1 0 00-1-1z",
                            clip_rule: "evenodd",
                        }
                    }
                    
                    span {
                        "{error}"
                    }
                }
            }
        }
    }
}

/// Compact wallet connect component for space-constrained layouts
#[component]
pub fn WalletConnectCompact(
    #[props(default = "")] class: &'static str,
    #[props(default = WalletConnectVariant::Primary)] variant: WalletConnectVariant,
) -> Element {
    rsx! {
        WalletConnect {
            class: class,
            show_strategy_selector: false,
            show_status: false,
            show_address: false,
            size: WalletConnectSize::Small,
            variant: variant,
            connect_text: "Connect",
            disconnect_text: "Disconnect",
        }
    }
}

/// Wallet connect component with address display only
#[component] 
pub fn WalletConnectWithAddress(
    #[props(default = "")] class: &'static str,
    #[props(default = WalletConnectVariant::Primary)] variant: WalletConnectVariant,
) -> Element {
    rsx! {
        WalletConnect {
            class: class,
            show_strategy_selector: false,
            show_status: true,
            show_address: true,
            size: WalletConnectSize::Medium,
            variant: variant,
        }
    }
}

/// Full-featured wallet connect component with all options
#[component]
pub fn WalletConnectFull(
    #[props(default = "")] class: &'static str,
    on_connection_change: Option<EventHandler<ConnectionChangeEvent>>,
    on_strategy_change: Option<EventHandler<WalletStrategyType>>,
) -> Element {
    rsx! {
        WalletConnect {
            class: class,
            show_strategy_selector: true,
            show_status: true,
            show_address: true,
            size: WalletConnectSize::Medium,
            variant: WalletConnectVariant::Primary,
            on_connection_change: on_connection_change,
            on_strategy_change: on_strategy_change,
        }
    }
}

impl std::str::FromStr for WalletStrategyType {
    type Err = ();
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Beacon" => Ok(WalletStrategyType::Beacon),
            "Wander" => Ok(WalletStrategyType::Wander),
            "WalletKit" => Ok(WalletStrategyType::WalletKit),
            "WebWallet" => Ok(WalletStrategyType::WebWallet),
            _ => Err(()),
        }
    }
}


impl std::fmt::Display for WalletStrategyType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            WalletStrategyType::Beacon => "Beacon",
            WalletStrategyType::Wander => "Wander",
            WalletStrategyType::WalletKit => "WalletKit", 
            WalletStrategyType::WebWallet => "WebWallet",
        };
        write!(f, "{}", name)
    }
}