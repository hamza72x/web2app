use tao::menu::{MenuId, MenuItem, MenuItemAttributes};

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

const MENU_ID_THEME_SYSTEM: MenuId = MenuId(4000);
const MENU_ID_THEME_DARK: MenuId = MenuId(4001);
const MENU_ID_THEME_LIGHT: MenuId = MenuId(4002);

const JS_LOAD_SCRIPTS: &str = r#"
    // load darkreader.js
    (function() {
        window.onload = function() {
            console.log('Window is loaded');
            console.log('Loading DarkReader');
            var script = document.createElement('script');
            script.src = 'https://unpkg.com/darkreader@4.9.58/darkreader.js';

            script.onload = function() {
                console.log('DarkReader is loaded');
                // check localstorage for theme
                var theme = localStorage.getItem('theme');
                if (theme === 'dark') {
                    DarkReader.enable();
                } else if (theme === 'light') {
                    DarkReader.disable();
                } else {
                    DarkReader.auto();
                }
            };

            document.head.appendChild(script);
        };
    })();
"#;

const JS_SYSTEM_THEME: &str = r#"
    // check if darkreader.js is loaded
    (function() {
        if (typeof DarkReader === 'undefined') {
            console.log('DarkReader is not loaded');
            return;
        }
        DarkReader.auto();
        localStorage.setItem('theme', 'system');
    })();
"#;

const JS_DARK_THEME: &str = r#"
    // hook darkreader.js into the page
    (function() {
        if (typeof DarkReader === 'undefined') {
            console.log('DarkReader is not loaded');
            return;
        }
        DarkReader.enable();
        localStorage.setItem('theme', 'dark');
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
        localStorage.setItem('theme', 'light');
    })();
"#;

fn main() -> wry::Result<()> {
    let event_loop = EventLoop::new();

    let window = WindowBuilder::new()
        .with_title("%name%")
        .with_menu(build_menu())
        .build(&event_loop)?;

    let web_view = WebViewBuilder::new(window)?
        .with_devtools(true)
        .with_initialization_script(JS_LOAD_SCRIPTS)
        .with_url("%url%")?
        .build()?;

    return run_event_loop(event_loop, web_view);
}

fn run_event_loop(event_loop: EventLoop<()>, web_view: webview::WebView) -> wry::Result<()> {
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
                    MENU_ID_THEME_DARK => web_view.evaluate_script(JS_DARK_THEME).unwrap(),
                    MENU_ID_THEME_LIGHT => web_view.evaluate_script(JS_LIGHT_THEME).unwrap(),
                    MENU_ID_THEME_SYSTEM => web_view.evaluate_script(JS_SYSTEM_THEME).unwrap(),
                    _ => (),
                }
            }
            _ => (),
        }
    });
}

fn build_menu() -> MenuBar {
    let mut menu_bar = MenuBar::new();

    let mut file_menu = MenuBar::new();
    let mut edit_menu = MenuBar::new();
    let mut window_menu = MenuBar::new();
    let mut theme_menu = MenuBar::new();

    file_menu.add_native_item(MenuItem::Quit);

    edit_menu.add_native_item(MenuItem::Copy);
    edit_menu.add_native_item(MenuItem::Cut);
    edit_menu.add_native_item(MenuItem::Paste);
    edit_menu.add_native_item(MenuItem::SelectAll);
    edit_menu.add_native_item(MenuItem::Undo);
    edit_menu.add_native_item(MenuItem::Redo);

    window_menu.add_native_item(MenuItem::Minimize);

    theme_menu.add_item(MenuItemAttributes::new("System").with_id(MENU_ID_THEME_SYSTEM));
    theme_menu.add_item(MenuItemAttributes::new("Dark").with_id(MENU_ID_THEME_DARK));
    theme_menu.add_item(MenuItemAttributes::new("Light").with_id(MENU_ID_THEME_LIGHT));

    menu_bar.add_submenu("File", true, file_menu);
    menu_bar.add_submenu("Edit", true, edit_menu);
    menu_bar.add_submenu("Window", true, window_menu);
    menu_bar.add_submenu("Theme", true, theme_menu);

    menu_bar
}
