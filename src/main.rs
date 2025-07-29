#![allow(non_snake_case)]

use dioxus::prelude::*;

mod app;
mod components;
mod services;

fn main() {
    // Initialize logging for web console
    console_log::init_with_level(log::Level::Info).expect("Failed to initialize logger");
    
    log::info!("Starting Faithful Archive application");

    // Launch the Dioxus web app  
    launch(app::App);
}
