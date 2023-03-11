use tao::{
    accelerator::{Accelerator, SysMods},
    keyboard::KeyCode,
    menu::{MenuBar, MenuId, MenuItem, MenuItemAttributes},
};
use wry::webview::WebView;

use crate::app_config::AppConfig;

const SCALE_FACTOR: f64 = 1.1;

const JS_DARK_THEME: &str = r#"
    // hook darkreader.js into the page
    (function() {
        if (typeof DarkReader === 'undefined') {
            console.log('DarkReader is not loaded');
            return;
        }
        DarkReader.setFetchMethod(window.fetch);
        DarkReader.enable();
    })();
"#;

const JS_LIGHT_THEME: &str = r#"
    // check if darkreader.js is loaded
    (function() {
        if (typeof DarkReader === 'undefined') {
            console.log('DarkReader is not loaded');
            return;
        }
        DarkReader.disable();
    })();
"#;
// menu id with enum, but integer
pub const MENU_ID_THEME_DARK_READER: MenuId = MenuId(4001);
pub const MENU_ID_ZOOM_IN: MenuId = MenuId(4003);
pub const MENU_ID_ZOOM_OUT: MenuId = MenuId(4004);
pub const MENU_ID_RELOAD: MenuId = MenuId(4005);

pub fn build_menu(app_config: &AppConfig) -> MenuBar {
    let mut menu_bar = MenuBar::new();

    let mut file_menu = MenuBar::new();
    let mut edit_menu = MenuBar::new();
    let mut window_menu = MenuBar::new();
    let mut tools_menu = MenuBar::new();

    file_menu.add_native_item(MenuItem::Quit);

    edit_menu.add_native_item(MenuItem::Copy);
    edit_menu.add_native_item(MenuItem::Cut);
    edit_menu.add_native_item(MenuItem::Paste);
    edit_menu.add_native_item(MenuItem::SelectAll);
    edit_menu.add_native_item(MenuItem::Undo);
    edit_menu.add_native_item(MenuItem::Redo);

    window_menu.add_native_item(MenuItem::Minimize);

    // zoom
    window_menu.add_item(
        MenuItemAttributes::new("Zoom In")
            .with_id(MENU_ID_ZOOM_IN)
            .with_accelerators(&Accelerator::new(SysMods::Cmd, KeyCode::Plus)),
    );
    window_menu.add_item(
        MenuItemAttributes::new("Zoom Out")
            .with_id(MENU_ID_ZOOM_OUT)
            .with_accelerators(&Accelerator::new(SysMods::Cmd, KeyCode::Minus)),
    );

    // reload
    window_menu.add_item(
        MenuItemAttributes::new("Reload")
            .with_id(MENU_ID_RELOAD)
            .with_accelerators(&Accelerator::new(SysMods::Cmd, KeyCode::KeyR)),
    );

    tools_menu.add_item(
        MenuItemAttributes::new(app_config.dark_reader_text())
            .with_id(MENU_ID_THEME_DARK_READER)
            .with_accelerators(&Accelerator::new(SysMods::CmdShift, KeyCode::KeyI)),
    );

    menu_bar.add_submenu("File", true, file_menu);
    menu_bar.add_submenu("Edit", true, edit_menu);
    menu_bar.add_submenu("Window", true, window_menu);
    menu_bar.add_submenu("Tools", true, tools_menu);

    menu_bar
}

pub fn handle_menu_click(web_view: &WebView, app_config: &mut AppConfig, menu_id: MenuId) {
    println!("Menu clicked! {:?}", menu_id);

    // switch case
    match menu_id {
        MENU_ID_THEME_DARK_READER => {
            app_config.dark_reader_enabled = !app_config.dark_reader_enabled;

            if app_config.dark_reader_enabled {
                web_view.evaluate_script(JS_DARK_THEME).unwrap();
            } else {
                web_view.evaluate_script(JS_LIGHT_THEME).unwrap();
            }

            app_config.save();

            // change menu text
            web_view.window().set_menu(Some(build_menu(&app_config)));
        }
        MENU_ID_ZOOM_IN => {
            if app_config.zoom_factor < 2.0 {
                app_config.zoom_factor *= SCALE_FACTOR;
                web_view.zoom(app_config.zoom_factor);
                app_config.save();
            }
        }
        MENU_ID_ZOOM_OUT => {
            if app_config.zoom_factor > 0.1 {
                app_config.zoom_factor /= SCALE_FACTOR;
                web_view.zoom(app_config.zoom_factor);
                app_config.save();
            }
        }
        MENU_ID_RELOAD => {
            web_view.load_url(web_view.url().as_str());
        }
        _ => (),
    }
}
