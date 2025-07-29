// Components module for Faithful Archive
pub mod wallet_modal;
pub mod wallet_connect;
pub mod wallet_example;

// Re-export main components
pub use wallet_modal::{WalletModal, WalletConnectButton};
pub use wallet_connect::{
    WalletConnect, WalletConnectCompact, WalletConnectWithAddress, WalletConnectFull,
    WalletConnectProps, WalletConnectSize, WalletConnectVariant, ConnectionChangeEvent
};
pub use wallet_example::WalletIntegrationExample;