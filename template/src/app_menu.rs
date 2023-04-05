use super::app_config::AppConfig;
use super::util;

use tauri::{CustomMenuItem, Menu, MenuItem, Submenu, Window};

const SCALE_FACTOR: f64 = 1.1;

const JS_ENABLE_DARK_READER: &str = r#"
    (function() {
        if (typeof DarkReader === 'undefined') {
            console.log('DarkReader is not loaded');
            return;
        }
        DarkReader.setFetchMethod(window.fetch);
        DarkReader.enable();
    })();
"#;

const JS_DISABLE_DARK_READER: &str = r#"
    (function() {
        if (typeof DarkReader === 'undefined') {
            console.log('DarkReader is not loaded');
            return;
        }
        DarkReader.disable();
    })();
"#;

// build_menu creates a tauri::Menu
pub fn build_menu() -> Menu {
    return Menu::new()
        .add_submenu(get_file_submenu())
        .add_submenu(get_edit_submenu())
        .add_submenu(get_window_submenu())
        .add_submenu(get_tools_submenu());
}

pub fn handle_menu_click(window: &Window, menu_id: &str) {
    let mut app_config = AppConfig::load();

    match menu_id {
        "zoom_in" => {
            if app_config.zoom_factor < 2.0 {
                app_config.zoom_factor *= SCALE_FACTOR;
                util::zoom_webview(window, app_config.zoom_factor);
                app_config.save();
            }
        }
        "zoom_out" => {
            if app_config.zoom_factor > 0.1 {
                app_config.zoom_factor /= SCALE_FACTOR;
                util::zoom_webview(window, app_config.zoom_factor);
                app_config.save();
            }
        }
        "reload" => {
            window.eval("location.reload();").unwrap();
        }
        "dark_reader" => {
            app_config.dark_reader_enabled = !app_config.dark_reader_enabled;

            if app_config.dark_reader_enabled {
                window.eval(JS_ENABLE_DARK_READER).unwrap();
            } else {
                window.eval(JS_DISABLE_DARK_READER).unwrap();
            }

            app_config.save();

            let menu_handle = window.menu_handle();

            std::thread::spawn(move || {
                menu_handle
                    .get_item("dark_reader")
                    .set_title(app_config.dark_reader_text())
                    .expect("failed to set dark reader menu text");
            });
        }
        "back" => {
            window.eval("window.history.back();").unwrap();
        }
        "forward" => {
            window.eval("window.history.forward();").unwrap();
        }
        _ => {
            println!("unhandled menu click: {}", menu_id);
        }
    }
}

// File
fn get_file_submenu() -> Submenu {
    return Submenu::new("File", Menu::new().add_native_item(MenuItem::Quit));
}

// Edit
fn get_edit_submenu() -> Submenu {
    return Submenu::new(
        "Edit",
        Menu::new()
            .add_native_item(MenuItem::Copy)
            .add_native_item(MenuItem::Cut)
            .add_native_item(MenuItem::Paste)
            .add_native_item(MenuItem::SelectAll)
            .add_native_item(MenuItem::Undo)
            .add_native_item(MenuItem::Redo),
    );
}

// Window
fn get_window_submenu() -> Submenu {
    let reload = CustomMenuItem::new("reload".to_string(), "Reload")
        .accelerator("CmdOrCtrl+R");
    let zoom_in = CustomMenuItem::new("zoom_in".to_string(), "Zoom In")
        .accelerator("CmdOrCtrl+Plus");
    let zoom_out = CustomMenuItem::new("zoom_out".to_string(), "Zoom Out")
        .accelerator("CmdOrCtrl+-");
    let back = CustomMenuItem::new("back".to_string(), "Back".to_string())
        .accelerator("CmdOrCtrl+[");
    let forward =
        CustomMenuItem::new("forward".to_string(), "Forward".to_string())
            .accelerator("CmdOrCtrl+]");

    return Submenu::new(
        "Window",
        Menu::new()
            .add_native_item(MenuItem::Minimize)
            .add_item(zoom_in)
            .add_item(zoom_out)
            .add_item(back)
            .add_item(forward)
            .add_item(reload),
    );
}

// Tools
fn get_tools_submenu() -> Submenu {
    let dark_reader = CustomMenuItem::new(
        "dark_reader".to_string(),
        "Enable Dark Reader".to_string(),
    )
    .accelerator("CmdOrCtrl+Shift+I");

    return Submenu::new("Tools", Menu::new().add_item(dark_reader));
}
