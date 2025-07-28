#![allow(non_snake_case)]

use dioxus::prelude::*;

#[component]
pub fn App() -> Element {
    rsx! {
        div {
            id: "app",
            class: "min-h-screen bg-gradient-to-br from-green-50 to-green-100",
            
            // Header
            header {
                class: "bg-white shadow-sm border-b border-green-200",
                div {
                    class: "max-w-7xl mx-auto px-4 sm:px-6 lg:px-8",
                    div {
                        class: "flex justify-between items-center py-6",
                        
                        // Logo and title
                        div {
                            class: "flex items-center space-x-3",
                            div {
                                class: "w-10 h-10 bg-green-600 rounded-lg flex items-center justify-center",
                                "✚"
                            }
                            div {
                                h1 {
                                    class: "text-2xl font-bold text-gray-900",
                                    "Faithful Archive"
                                }
                                p {
                                    class: "text-sm text-gray-600",
                                    "Christ-honoring content on Arweave"
                                }
                            }
                        }
                        
                        // Navigation
                        nav {
                            class: "hidden md:flex space-x-8",
                            a {
                                href: "#",
                                class: "text-gray-600 hover:text-green-600 px-3 py-2 rounded-md text-sm font-medium",
                                "Browse"
                            }
                            a {
                                href: "#",
                                class: "text-gray-600 hover:text-green-600 px-3 py-2 rounded-md text-sm font-medium",
                                "Upload"
                            }
                            a {
                                href: "#",
                                class: "text-gray-600 hover:text-green-600 px-3 py-2 rounded-md text-sm font-medium",
                                "About"
                            }
                        }
                        
                        // Wallet connection button
                        button {
                            class: "bg-green-600 hover:bg-green-700 text-white px-4 py-2 rounded-lg text-sm font-medium transition-colors",
                            "Connect Wallet"
                        }
                    }
                }
            }
            
            // Main content
            main {
                class: "max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8",
                
                // Hero section
                div {
                    class: "text-center py-16",
                    h2 {
                        class: "text-4xl font-bold text-gray-900 mb-4",
                        "Preserve Spiritual Content Forever"
                    }
                    p {
                        class: "text-xl text-gray-600 mb-8 max-w-3xl mx-auto",
                        "Upload sermons, worship resources, and Bible studies to Arweave's permanent storage. "
                        "Built with Rust and WebAssembly for performance and security."
                    }
                    
                    div {
                        class: "space-x-4",
                        button {
                            class: "bg-green-600 hover:bg-green-700 text-white px-8 py-3 rounded-lg text-lg font-medium transition-colors",
                            "Start Uploading"
                        }
                        button {
                            class: "border border-green-600 text-green-600 hover:bg-green-50 px-8 py-3 rounded-lg text-lg font-medium transition-colors",
                            "Browse Content"
                        }
                    }
                }
                
                // Features section
                div {
                    class: "grid md:grid-cols-3 gap-8 py-16",
                    
                    // Feature 1
                    div {
                        class: "text-center",
                        div {
                            class: "w-16 h-16 bg-green-100 rounded-lg flex items-center justify-center mx-auto mb-4",
                            "🔗"
                        }
                        h3 {
                            class: "text-xl font-semibold text-gray-900 mb-2",
                            "Permanent Storage"
                        }
                        p {
                            class: "text-gray-600",
                            "Content stored on Arweave blockchain remains accessible for 200+ years, "
                            "ensuring your spiritual resources are preserved for future generations."
                        }
                    }
                    
                    // Feature 2
                    div {
                        class: "text-center",
                        div {
                            class: "w-16 h-16 bg-green-100 rounded-lg flex items-center justify-center mx-auto mb-4",
                            "⚡"
                        }
                        h3 {
                            class: "text-xl font-semibold text-gray-900 mb-2",
                            "High Performance"
                        }
                        p {
                            class: "text-gray-600",
                            "Built with Rust and compiled to WebAssembly for near-native performance. "
                            "Fast loading and smooth interactions for the best user experience."
                        }
                    }
                    
                    // Feature 3
                    div {
                        class: "text-center",
                        div {
                            class: "w-16 h-16 bg-green-100 rounded-lg flex items-center justify-center mx-auto mb-4",
                            "🛡️"
                        }
                        h3 {
                            class: "text-xl font-semibold text-gray-900 mb-2",
                            "Content Moderation"
                        }
                        p {
                            class: "text-gray-600",
                            "All content is reviewed to ensure only Christ-honoring material is published. "
                            "Community-driven moderation maintains high quality standards."
                        }
                    }
                }
                
                // Stats section
                div {
                    class: "bg-white rounded-xl shadow-sm border border-green-200 p-8 text-center",
                    div {
                        class: "grid grid-cols-2 md:grid-cols-4 gap-8",
                        div {
                            div {
                                class: "text-3xl font-bold text-green-600",
                                "0"
                            }
                            div {
                                class: "text-sm text-gray-600",
                                "Items Archived"
                            }
                        }
                        div {
                            div {
                                class: "text-3xl font-bold text-green-600",
                                "0"
                            }
                            div {
                                class: "text-sm text-gray-600",
                                "Churches Served"
                            }
                        }
                        div {
                            div {
                                class: "text-3xl font-bold text-green-600",
                                "∞"
                            }
                            div {
                                class: "text-sm text-gray-600",
                                "Years Preserved"
                            }
                        }
                        div {
                            div {
                                class: "text-3xl font-bold text-green-600",
                                "100%"
                            }
                            div {
                                class: "text-sm text-gray-600",
                                "Uptime Target"
                            }
                        }
                    }
                }
            }
            
            // Footer
            footer {
                class: "bg-gray-900 text-white mt-16",
                div {
                    class: "max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-12",
                    div {
                        class: "grid md:grid-cols-4 gap-8",
                        
                        // Company info
                        div {
                            h3 {
                                class: "text-lg font-semibold mb-4",
                                "Faithful Archive"
                            }
                            p {
                                class: "text-gray-400",
                                "Preserving Christ-honoring content on the blockchain for future generations."
                            }
                        }
                        
                        // Links
                        div {
                            h4 {
                                class: "font-semibold mb-4",
                                "Platform"
                            }
                            ul {
                                class: "space-y-2 text-gray-400",
                                li { a { href: "#", class: "hover:text-white", "Browse Content" } }
                                li { a { href: "#", class: "hover:text-white", "Upload" } }
                                li { a { href: "#", class: "hover:text-white", "Moderation" } }
                            }
                        }
                        
                        // Resources
                        div {
                            h4 {
                                class: "font-semibold mb-4",
                                "Resources"
                            }
                            ul {
                                class: "space-y-2 text-gray-400",
                                li { a { href: "#", class: "hover:text-white", "Documentation" } }
                                li { a { href: "#", class: "hover:text-white", "API" } }
                                li { a { href: "#", class: "hover:text-white", "GitHub" } }
                            }
                        }
                        
                        // Contact
                        div {
                            h4 {
                                class: "font-semibold mb-4",
                                "Connect"
                            }
                            ul {
                                class: "space-y-2 text-gray-400",
                                li { a { href: "#", class: "hover:text-white", "Contact" } }
                                li { a { href: "#", class: "hover:text-white", "Support" } }
                                li { a { href: "#", class: "hover:text-white", "Community" } }
                            }
                        }
                    }
                    
                    div {
                        class: "border-t border-gray-800 mt-8 pt-8 text-center text-gray-400",
                        p {
                            "© 2025 Faithful Archive. Built with ❤️ and ⚡ Rust for the glory of God."
                        }
                    }
                }
            }
        }
    }
}