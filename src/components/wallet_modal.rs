use dioxus::prelude::*;
use crate::services::wallet::{WalletService, WalletStrategyType};

#[component]
pub fn WalletModal(show: Signal<bool>, on_connect: EventHandler<WalletStrategyType>) -> Element {
    let mut available_strategies = use_signal(|| Vec::<WalletStrategyType>::new());
    
    // Load available strategies when modal opens
    use_effect(move || {
        if show.read().clone() {
            log::info!("🪟 Wallet modal opened, loading strategies...");
            spawn(async move {
                let service = WalletService::init().await; // Use init() instead of new() to get proper initialization
                let strategies = service.get_available_strategies().await;
                log::info!("🔍 Modal loaded {} strategies: {:?}", strategies.len(), strategies);
                available_strategies.set(strategies);
            });
        }
    });
    
    let close_modal = move |_| {
        show.set(false);
    };
    
    let mut connect_wallet = move |strategy: WalletStrategyType| {
        on_connect.call(strategy);
        show.set(false);
    };
    
    if !show.read().clone() {
        return rsx! {};
    }
    
    rsx! {
        // Modal backdrop
        div {
            class: "fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50",
            onclick: close_modal,
            
            // Modal content
            div {
                class: "bg-gray-800 rounded-2xl p-6 max-w-md w-full mx-4 relative shadow-2xl transform transition-all",
                onclick: |e| e.stop_propagation(), // Prevent backdrop close when clicking modal content
                
                // Close button
                button {
                    class: "absolute top-4 right-4 text-gray-400 hover:text-white transition-colors",
                    onclick: close_modal,
                    "✕"
                }
                
                // Modal header
                h2 {
                    class: "text-white text-xl font-semibold mb-6",
                    "Connect wallet"
                }
                
                // Wallet options
                div {
                    class: "space-y-3",
                    
                    // Beacon wallet
                    if available_strategies.read().contains(&WalletStrategyType::Beacon) {
                        WalletOption {
                            strategy: WalletStrategyType::Beacon,
                            icon: "🔴", // Blue circle like in the image
                            name: "Beacon",
                            description: "iOS based agent first wallet for AO", 
                            on_click: move |_| connect_wallet(WalletStrategyType::Beacon),
                        }
                    }
                    
                    // Wander wallet
                    if available_strategies.read().contains(&WalletStrategyType::Wander) {
                        WalletOption {
                            strategy: WalletStrategyType::Wander,
                            icon: "🟣", // Purple butterfly-like icon
                            name: "Wander",
                            description: "Non-custodial Arweave & AO wallet for your favorite browser",
                            on_click: move |_| connect_wallet(WalletStrategyType::Wander),
                        }
                    }
                    
                    // Other available strategies
                    for strategy in available_strategies.read().iter() {
                        if !matches!(strategy, WalletStrategyType::Beacon | WalletStrategyType::Wander) {
                            WalletOption {
                                strategy: *strategy,
                                icon: "💼",
                                name: match strategy {
                                    WalletStrategyType::WalletKit => "Arweave Wallet Kit",
                                    WalletStrategyType::WebWallet => "Web Wallet",
                                    _ => "Unknown Wallet",
                                },
                                description: match strategy {
                                    WalletStrategyType::WalletKit => "Universal wallet connection library",
                                    WalletStrategyType::WebWallet => "Web-based wallet connection",
                                    _ => "Unknown wallet type",
                                },
                                on_click: {
                                    let current_strategy = *strategy;
                                    move |_| connect_wallet(current_strategy)
                                },
                            }
                        }
                    }
                }
                
                // Don't have a wallet section
                div {
                    class: "mt-6 pt-4 border-t border-gray-700",
                    
                    div {
                        class: "flex items-center justify-between",
                        
                        div {
                            h3 {
                                class: "text-white font-medium mb-1",
                                "Don't have a wallet?"
                            }
                            p {
                                class: "text-gray-400 text-sm",
                                "Click to learn more about the permaweb & wallets."
                            }
                        }
                        
                        button {
                            class: "bg-white text-black px-4 py-2 rounded-lg font-medium hover:bg-gray-100 transition-colors",
                            onclick: move |_| {
                                // Open wallet information page
                                web_sys::window()
                                    .unwrap()
                                    .open_with_url_and_target("https://arweave.org/wallet", "_blank")
                                    .unwrap();
                            },
                            "GET"
                        }
                    }
                }
                
                // Footer text
                div {
                    class: "mt-4 text-center text-xs text-gray-500",
                    "Faithful Archive Wallet Connection"
                }
            }
        }
    }
}

#[component]
fn WalletOption(
    strategy: WalletStrategyType,
    icon: &'static str,
    name: &'static str, 
    description: &'static str,
    on_click: EventHandler<MouseEvent>,
) -> Element {
    rsx! {
        button {
            class: "w-full flex items-center justify-between p-4 bg-gray-700 hover:bg-gray-600 rounded-xl transition-colors group",
            onclick: on_click,
            
            div {
                class: "flex items-center space-x-4",
                
                // Wallet icon
                div {
                    class: format!("w-12 h-12 rounded-xl flex items-center justify-center {}",
                        match strategy {
                            WalletStrategyType::Beacon => "beacon-wallet-bg",
                            WalletStrategyType::Wander => "wander-wallet-bg",
                            _ => "bg-gradient-to-br from-gray-500 to-gray-600",
                        }
                    ),
                    if strategy == WalletStrategyType::Beacon {
                        img {
                            src: asset!("/assets/beaconwallet.svg"),
                            alt: "Beacon Wallet",
                            class: "w-8 h-8 object-contain",
                            style: "width: 32px; height: 32px;",
                            draggable: "false"
                        }
                    } else if strategy == WalletStrategyType::Wander {
                        img {
                            src: asset!("/assets/wanderapp.svg"),
                            alt: "Wander Wallet",
                            class: "w-8 h-8 object-contain",
                            style: "width: 32px; height: 32px;",
                            draggable: "false"
                        }
                    } else {
                        "💼" // Generic wallet icon for other wallets
                    }
                }
                
                // Wallet info
                div {
                    class: "text-left",
                    h3 {
                        class: "text-white font-medium text-base mb-1",
                        "{name}"
                    }
                    p {
                        class: "text-gray-400 text-sm",
                        "{description}"
                    }
                }
            }
            
            // Connect button
            div {
                class: "bg-white text-black px-4 py-2 rounded-lg text-sm font-medium group-hover:bg-gray-100 transition-colors shadow-sm",
                "GO"
            }
        }
    }
}

// Enhanced wallet button that opens modal
#[component]
pub fn WalletConnectButton() -> Element {
    let mut show_modal = use_signal(|| false);
    let wallet_state = crate::services::wallet::use_wallet_state();
    
    let handle_wallet_connect = move |strategy: WalletStrategyType| {
        spawn(async move {
            let mut service = WalletService::new();
            let _ = service.set_strategy(strategy).await;
            let _ = service.connect().await;
        });
    };
    
    let wallet_state_clone = wallet_state.clone();
    let button_click = move |_| {
        if wallet_state_clone.read().connected {
            // Disconnect if already connected
            spawn(async move {
                let mut service = WalletService::new();
                let _ = service.disconnect().await;
            });
        } else {
            // Show modal to select wallet
            show_modal.set(true);
        }
    };
    
    let button_text = if wallet_state.read().connecting {
        "Connecting..."
    } else if wallet_state.read().connected {
        "Disconnect"
    } else {
        "Connect"
    };
    
    let button_class = if wallet_state.read().connected {
        "bg-red-600 hover:bg-red-700 text-white px-4 py-2 rounded-lg text-sm font-medium transition-colors shadow-sm hover:shadow-md"
    } else if !wallet_state.read().available && !show_modal.read().clone() {
        "bg-gray-300 cursor-not-allowed text-gray-600 px-4 py-2 rounded-lg text-sm font-medium border border-gray-200"
    } else {
        "bg-green-600 hover:bg-green-700 text-white px-4 py-2 rounded-lg text-sm font-medium transition-colors shadow-sm hover:shadow-md"
    };
    
    rsx! {
        div {
            class: "wallet-connect-container",
            
            button {
                class: button_class,
                disabled: wallet_state.read().connecting,
                onclick: button_click,
                "{button_text}"
            }
            
            // Connected address display
            if wallet_state.read().connected {
                div {
                    class: "mt-2 text-xs text-green-700 bg-green-50 px-2 py-1 rounded border border-green-200",
                    "Connected: {crate::services::wallet::WalletService::format_address(wallet_state.read().address.as_ref().unwrap_or(&\"Unknown\".to_string()))}"
                }
            }
            
            // Error display
            if let Some(error) = &wallet_state.read().error {
                div {
                    class: "mt-2 text-xs text-red-700 bg-red-50 px-2 py-1 rounded border border-red-200 max-w-xs",
                    "{error}"
                }
            }
            
            // Wallet selection modal
            WalletModal {
                show: show_modal,
                on_connect: handle_wallet_connect,
            }
        }
    }
}