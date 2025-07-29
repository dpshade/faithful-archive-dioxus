/**
 * Beacon Wallet Bridge Entry Point
 * 
 * This file imports the ao-sync-sdk and exposes WalletClient on the global window object
 * for WASM consumption.
 */

import AoSyncSdk from '@vela-ventures/ao-sync-sdk';

console.log('🔄 Beacon Wallet Bridge Entry Point Loading...');
console.log('📦 AoSyncSdk imported:', AoSyncSdk);

// Try to extract WalletClient from the SDK
let WalletClient;
if (AoSyncSdk && typeof AoSyncSdk === 'function') {
    // If the default export is the WalletClient
    WalletClient = AoSyncSdk;
} else if (AoSyncSdk && AoSyncSdk.WalletClient) {
    // If WalletClient is a named export
    WalletClient = AoSyncSdk.WalletClient;
} else if (AoSyncSdk && AoSyncSdk.default) {
    // If it's wrapped in a default export
    WalletClient = AoSyncSdk.default;
} else {
    console.error('❌ Could not find WalletClient in ao-sync-sdk');
    WalletClient = null;
}

if (WalletClient) {
    // Expose WalletClient on global window object
    window.WalletClient = WalletClient;
    console.log('✅ WalletClient exposed on window:', typeof window.WalletClient);
    
    // Test instantiation
    try {
        const testInstance = new window.WalletClient();
        console.log('✅ Successfully created WalletClient instance:', testInstance);
    } catch (e) {
        console.error('❌ Failed to create WalletClient instance:', e);
    }
} else {
    console.error('❌ WalletClient not found, creating mock implementation');
    
    // Create a mock implementation as fallback
    window.WalletClient = class MockWalletClient {
        constructor() {
            console.log('🆕 Mock WalletClient created as fallback');
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
        
        async sign(transaction) {
            console.log('✍️ Mock sign called:', transaction);
            return {
                ...transaction,
                signature: 'mock_signature_' + Math.random().toString(36).substr(2, 9)
            };
        }
        
        async signDataItem(dataItem) {
            console.log('✍️ Mock signDataItem called:', dataItem);
            return new ArrayBuffer(64);
        }
        
        async reconnect() {
            console.log('🔄 Mock reconnect called');
            return this.connect({});
        }
        
        on(event, callback) {
            console.log('👂 Mock event listener registered:', event);
        }
    };
}

// Dispatch ready event
window.dispatchEvent(new CustomEvent('walletClientReady', {
    detail: {
        available: typeof window.WalletClient !== 'undefined',
        type: 'beacon',
        isReal: WalletClient !== null
    }
}));

console.log('📡 Beacon Wallet Bridge Ready');