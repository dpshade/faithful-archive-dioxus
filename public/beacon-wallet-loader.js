// Beacon Wallet Loader
// This script loads the real ao-sync-sdk and exposes it to the WASM environment

console.log("🔄 Loading Beacon Wallet SDK from ao-sync-sdk...");

// Try to import the real WalletClient from ao-sync-sdk
let WalletClient;
try {
    console.log("📦 Attempting to import ao-sync-sdk via CDN...");
    const module = await import('@vela-ventures/ao-sync-sdk');
    WalletClient = module.default;
    console.log("✅ ao-sync-sdk loaded successfully from CDN:", WalletClient);
} catch (error) {
    console.error("❌ Failed to import ao-sync-sdk via CDN:", error);
    console.log("⚠️ Will fall back to mock implementation");
    WalletClient = null;
}

// Expose WalletClient to global scope for WASM access
if (WalletClient) {
    window.WalletClient = WalletClient;
    console.log("✅ Real WalletClient from ao-sync-sdk assigned to window.WalletClient");
    console.log("🔍 window.WalletClient type:", typeof window.WalletClient);
    console.log("🧪 Can instantiate:", typeof window.WalletClient === 'function');

    // Verify real SDK functionality
    try {
        const testInstance = new window.WalletClient();
        console.log("✅ Successfully created ao-sync-sdk instance:", testInstance);
    
    // Test if it has the expected methods
    const hasConnect = typeof testInstance.connect === 'function';
    const hasDisconnect = typeof testInstance.disconnect === 'function';
    const hasSign = typeof testInstance.sign === 'function';
    const hasSignDataItem = typeof testInstance.signDataItem === 'function';
    const hasOn = typeof testInstance.on === 'function';
    
    console.log("🔍 Method availability:", {
        connect: hasConnect,
        disconnect: hasDisconnect,
        sign: hasSign,
        signDataItem: hasSignDataItem,
        on: hasOn
    });
    
    if (hasConnect && hasDisconnect && hasSign && hasSignDataItem && hasOn) {
        console.log("✅ All required methods are available");
        } else {
            console.warn("⚠️ Some required methods are missing");
        }
        
    } catch (e) {
        console.error("❌ Failed to create ao-sync-sdk instance:", e);
        console.log("⚠️ Real SDK failed, will use mock instead");
        WalletClient = null;
    }
}

// If we don't have real SDK, create mock
if (!WalletClient) {
    console.warn("🛠️ Creating mock WalletClient as fallback");
    
    window.WalletClient = class MockWalletClient {
        constructor() {
            console.log("🆕 Fallback Mock WalletClient created");
        }
        
        async connect(options) {
            console.log("🔗 Mock connect called with options:", options);
            await new Promise(resolve => setTimeout(resolve, 1000));
            return {
                address: "mock_beacon_address_" + Math.random().toString(36).substr(2, 9)
            };
        }
        
        async disconnect() {
            console.log("🔌 Mock disconnect called");
            return true;
        }
        
        async sign(transaction) {
            console.log("✍️ Mock sign called:", transaction);
            return {
                ...transaction,
                signature: "mock_signature_" + Math.random().toString(36).substr(2, 9)
            };
        }
        
        async signDataItem(dataItem) {
            console.log("✍️ Mock signDataItem called:", dataItem);
            return new ArrayBuffer(64); // Mock signature buffer
        }
        
        async reconnect() {
            console.log("🔄 Mock reconnect called");
            return this.connect({});
        }
        
        on(event, callback) {
            console.log("👂 Mock event listener registered:", event);
        }
    };
}

// Final verification
console.log("🔍 Final check - window.WalletClient exists:", typeof window.WalletClient !== 'undefined');
console.log("🏗️ Final check - window.WalletClient is function:", typeof window.WalletClient === 'function');

// Dispatch event to notify WASM that WalletClient is ready
const isReal = WalletClient && window.WalletClient && window.WalletClient.name !== 'MockWalletClient';
window.dispatchEvent(new CustomEvent('walletClientReady', {
    detail: {
        available: typeof window.WalletClient !== 'undefined',
        type: 'beacon',
        isReal: isReal
    }
}));

console.log(`📡 Dispatched walletClientReady event - Using ${isReal ? 'Real' : 'Mock'} SDK`);