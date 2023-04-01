use std::path::PathBuf;

use super::app_config::AppConfig;
use super::app_data;
use super::app_menu;
use super::generated;
use super::util;

use tauri::AppHandle;
use tauri::Manager;
use tauri::WindowBuilder;
use tauri::WindowUrl;

// handlers

// open new window handler
#[tauri::command]
fn open_new_window(app_handle: AppHandle, url: String) {
    println!("[rust handler] open new window: {}", url);
    create_a_window(
        &app_handle,
        &util::alphanumeric(url.as_str(), '_'),
        url.as_str(),
    );
}

// download file handler
#[tauri::command]
fn download_file(url: String) {
    println!("[rust handler] download file: {}", url);
    util::download_file(url.as_str(), "downloaded_file.txt");
}

// open_browser handler
#[tauri::command]
fn open_browser(url: String) {
    println!("[rust handler] open browser: {}", url);
    util::open_browser(url.as_str());
}

pub fn build_tauri_app() {
    let mut builder = tauri::Builder::default();

    // menu
    builder = builder.menu(app_menu::build_menu(&AppConfig::default()));
    builder = builder.on_menu_event(|event| {
        app_menu::handle_menu_click(event.window(), event.menu_item_id());
    });

    // setup, create window
    builder = builder.setup(|app| {
        create_a_window(
            &app.app_handle(),
            app_data::MAIN_WINDOW,
            app_data::URL,
        );
        Ok(())
    });

    // run
    builder
        .invoke_handler(tauri::generate_handler![
            open_new_window,
            download_file,
            open_browser,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn create_a_window(app: &AppHandle, label: &str, url: &str) {
    let app_config = AppConfig::load();
    let window_url = WindowUrl::App(PathBuf::from(url));
    let mut builder = WindowBuilder::new(app, label, window_url);

    builder = builder.initialization_script(generated::INIT_SCRIPT);

    if let Some(user_agent) = app_data::USER_AGENT {
        builder = builder.user_agent(user_agent);
    }

    let window = builder.build().expect("error while creating main window");

    // zoom
    util::zoom_webview(&window, app_config.zoom_factor);

    // title
    window
        .set_title(app_data::APP_NAME)
        .expect("error while setting window title");
}
