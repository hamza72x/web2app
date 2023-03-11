// use serde::{Deserialize, Serialize};
use serde::{Deserialize, Serialize};

use tao::{
    accelerator::{Accelerator, SysMods},
    keyboard::KeyCode,
    menu::{MenuId, MenuItem, MenuItemAttributes},
};

use wry::{
    application::{
        event::{Event, StartCause, WindowEvent},
        event_loop::{ControlFlow, EventLoop},
        menu::MenuBar,
        window::WindowBuilder,
    },
    webview,
    webview::WebViewBuilder,
};

// menu id with enum, but integer
const MENU_ID_THEME_DARK_READER: MenuId = MenuId(4001);
const MENU_ID_ZOOM_IN: MenuId = MenuId(4003);
const MENU_ID_ZOOM_OUT: MenuId = MenuId(4004);
const MENU_ID_RELOAD: MenuId = MenuId(4005);

const SCALE_FACTOR: f64 = 1.1;

const JS_LOAD_SCRIPTS: &str = r#"
    // load darkreader.js
    (function() {
        window.onload = function() {
            console.log('Window is loaded');
            console.log('Loading DarkReader');
            var script = document.createElement('script');
            script.src = 'https://cdn.jsdelivr.net/npm/darkreader@4.9.58/darkreader.min.js';
            script.onload = function() {
                console.log('DarkReader is loaded');
            };
            document.head.appendChild(script);
        };
    })();
"#;

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

// $HOME/.config/nativefier_tauri_apps/app_name/config.json
#[derive(Serialize, Deserialize, Debug)]
struct AppConfig {
    #[serde(default)]
    zoom_factor: f64,

    #[serde(default)]
    dark_reader_enabled: bool,
}

impl Default for AppConfig {
    fn default() -> Self {
        return Self {
            zoom_factor: 1.0,
            dark_reader_enabled: false,
        };
    }
}

impl AppConfig {
    fn default() -> Self {
        return Self {
            zoom_factor: 1.0,
            dark_reader_enabled: false,
        };
    }

    fn config_dir() -> std::path::PathBuf {
        return home::home_dir()
            .unwrap()
            .join(".config")
            .join("nativefier_tauri_apps")
            .join("app_name");
    }

    fn config_path() -> std::path::PathBuf {
        return Self::config_dir().join("config.json");
    }

    fn create_config_dir() {
        let config_dir = Self::config_dir();

        if !config_dir.exists() {
            std::fs::create_dir_all(config_dir).unwrap();
        }
    }

    fn load() -> Option<Self> {
        let config_path = Self::config_path();

        if !config_path.exists() {
            Self::create_config_dir();
            return None;
        }

        let config_file = std::fs::File::open(config_path).unwrap();
        let config: Self = serde_json::from_reader(config_file).unwrap();

        return Some(config);
    }

    fn save(&self) {
        let config_path = Self::config_path();

        let config_file = std::fs::File::create(config_path).unwrap();
        serde_json::to_writer_pretty(config_file, self).unwrap();
    }

    fn dark_reader_text(&self) -> &'static str {
        if self.dark_reader_enabled {
            return "Disable DarkReader";
        } else {
            return "Enable DarkReader";
        }
    }
}

fn main() -> wry::Result<()> {
    let app_config = AppConfig::load().unwrap_or(AppConfig::default());

    let event_loop = EventLoop::new();

    let window = WindowBuilder::new()
        .with_title("app_name")
        .with_menu(build_menu(&app_config))
        .build(&event_loop)?;

    let web_view = WebViewBuilder::new(window)?
        .with_devtools(true)
        .with_initialization_script(JS_LOAD_SCRIPTS)
        .with_url("https://www.notion.so")?
        .build()?;

    return run_event_loop(event_loop, web_view, app_config);
}

fn build_menu(app_config: &AppConfig) -> MenuBar {
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

fn run_event_loop(
    event_loop: EventLoop<()>,
    web_view: webview::WebView,
    mut app_config: AppConfig,
) -> wry::Result<()> {
    web_view.zoom(app_config.zoom_factor);

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::NewEvents(StartCause::Init) => println!("app is started!"),
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,
            Event::MenuEvent { menu_id, .. } => {
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
            _ => (),
        }
    });
}
