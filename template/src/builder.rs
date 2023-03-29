use std::path::PathBuf;

use crate::util;

use super::app_config::AppConfig;
use super::app_data;
use super::app_menu;
use super::generated;

use tauri::App;
use tauri::WindowBuilder;
use tauri::WindowUrl;

pub fn build_tauri_app() {
    let mut builder = tauri::Builder::default();

    // menu
    builder = builder.menu(app_menu::build_menu(&AppConfig::default()));
    builder = builder.on_menu_event(|event| {
        app_menu::handle_menu_click(event.window(), event.menu_item_id());
    });

    // setup, create window
    builder = builder.setup(|app| {
        create_main_window(&app);
        Ok(())
    });

    // run
    builder
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn create_main_window(app: &App) {
    let app_config = AppConfig::load();
    let url = WindowUrl::App(PathBuf::from(app_data::URL));
    let mut builder = WindowBuilder::new(app, app_data::MAIN_WINDOW, url);

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
