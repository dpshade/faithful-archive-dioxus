/**
 * Faithful Archive - Wander Wallet Bridge
 * 
 * JavaScript bridge for handling Wander wallet (formerly ArConnect) integration
 * Provides helper functions for wallet detection and availability checking
 */

window.walletBridge = {
    /**
     * Check if Wander wallet extension is available
     * @returns {boolean} true if wallet is available
     */
    checkWalletAvailable() {
        return typeof window.arweaveWallet !== 'undefined' && 
               window.arweaveWallet !== null;
    },
    
    /**
     * Wait for wallet extension to load with timeout
     * @param {number} timeout - timeout in milliseconds (default: 5000)
     * @returns {Promise<boolean>} resolves when wallet is available
     */
    async waitForWallet(timeout = 5000) {
        return new Promise((resolve, reject) => {
            if (this.checkWalletAvailable()) {
                resolve(true);
                return;
            }
            
            console.log('Waiting for Wander wallet extension to load...');
            
            const checkInterval = setInterval(() => {
                if (this.checkWalletAvailable()) {
                    clearInterval(checkInterval);
                    clearTimeout(timeoutHandle);
                    console.log('Wander wallet extension detected');
                    resolve(true);
                }
            }, 100);
            
            const timeoutHandle = setTimeout(() => {
                clearInterval(checkInterval);
                console.warn('Wander wallet extension not found within timeout period');
                reject(new Error(`Wander wallet not available within ${timeout}ms`));
            }, timeout);
        });
    },
    
    /**
     * Get wallet extension info for debugging
     * @returns {object} wallet extension information
     */
    getWalletInfo() {
        if (!this.checkWalletAvailable()) {
            return {
                available: false,
                error: 'Wander wallet extension not found'
            };
        }
        
        return {
            available: true,
            type: 'Wander Wallet (ArConnect)',
            api: Object.keys(window.arweaveWallet || {}),
            version: window.arweaveWallet.getVersion ? 
                     window.arweaveWallet.getVersion() : 'unknown'
        };
    },
    
    /**
     * Enhanced error handling for wallet operations
     * @param {Error} error - JavaScript error from wallet operation
     * @returns {object} structured error information
     */
    parseWalletError(error) {
        const message = error.message || error.toString();
        const lowerMessage = message.toLowerCase();
        
        if (lowerMessage.includes('user denied') || 
            lowerMessage.includes('user rejected')) {
            return {
                type: 'USER_DENIED',
                title: 'Connection Denied',
                message: 'You denied the wallet connection request. Please try again and approve the connection.',
                action: 'retry'
            };
        }
        
        if (lowerMessage.includes('not installed') || 
            lowerMessage.includes('undefined')) {
            return {
                type: 'NOT_INSTALLED',
                title: 'Wallet Not Found',
                message: 'Wander wallet extension is not installed. Please install it from wander.app',
                action: 'install'
            };
        }
        
        if (lowerMessage.includes('permission')) {
            return {
                type: 'PERMISSION_ERROR',
                title: 'Permission Error',
                message: 'The requested permissions were not granted. Please reconnect and approve all permissions.',
                action: 'reconnect'
            };
        }
        
        if (lowerMessage.includes('network') || 
            lowerMessage.includes('connection')) {
            return {
                type: 'NETWORK_ERROR',
                title: 'Network Error',
                message: 'Unable to connect to Arweave network. Please check your internet connection.',
                action: 'retry'
            };
        }
        
        return {
            type: 'UNKNOWN_ERROR',
            title: 'Wallet Error',
            message: message || 'An unknown error occurred with the wallet connection.',
            action: 'retry'
        };
    },
    
    /**
     * Initialize wallet monitoring
     * Sets up event listeners for wallet state changes
     */
    initWalletMonitoring() {
        // Listen for wallet extension installation
        const checkWalletPeriodically = () => {
            if (!this.checkWalletAvailable()) {
                setTimeout(checkWalletPeriodically, 1000);
            } else {
                console.log('Wander wallet extension is now available');
                // Trigger custom event for WASM to detect
                window.dispatchEvent(new CustomEvent('wallet-available'));
            }
        };
        
        // Start checking if wallet is not available
        if (!this.checkWalletAvailable()) {
            console.log('Starting wallet availability monitoring...');
            checkWalletPeriodically();
        }
        
        // Listen for wallet account changes
        if (this.checkWalletAvailable() && window.arweaveWallet.addEventListener) {
            window.arweaveWallet.addEventListener('walletSwitch', (e) => {
                console.log('Wallet account switched:', e.detail);
                window.dispatchEvent(new CustomEvent('wallet-switched', { detail: e.detail }));
            });
            
            window.arweaveWallet.addEventListener('disconnect', () => {
                console.log('Wallet disconnected');
                window.dispatchEvent(new CustomEvent('wallet-disconnected'));
            });
        }
    }
};

// Initialize wallet monitoring when the script loads
if (document.readyState === 'loading') {
    document.addEventListener('DOMContentLoaded', () => {
        window.walletBridge.initWalletMonitoring();
    });
} else {
    window.walletBridge.initWalletMonitoring();
}

// Debug helper - expose wallet info to console
console.log('Faithful Archive Wallet Bridge loaded');
console.log('Wallet info:', window.walletBridge.getWalletInfo());