// Prevents additional console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[cfg(target_os = "macos")]
#[macro_use]
extern crate objc;

mod app_config;
mod app_data;
mod app_menu;
mod builder;
mod generated;
mod util;

fn main() {
    builder::build_tauri_app();
}
