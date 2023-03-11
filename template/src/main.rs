mod app_config;
mod app_menu;
use app_config::AppConfig;

use wry::{
    application::{
        event::{Event, StartCause, WindowEvent},
        event_loop::{ControlFlow, EventLoop},
        window::WindowBuilder,
    },
    webview,
    webview::WebViewBuilder,
};

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

fn main() -> wry::Result<()> {
    let app_config = AppConfig::load().unwrap_or(AppConfig::default());

    let event_loop = EventLoop::new();

    let window = WindowBuilder::new()
        .with_title("app_name")
        .with_menu(app_menu::build_menu(&app_config))
        .build(&event_loop)?;

    let web_view = WebViewBuilder::new(window)?
        .with_devtools(true)
        .with_initialization_script(JS_LOAD_SCRIPTS)
        .with_url("https://www.notion.so")?
        .build()?;

    return run_event_loop(event_loop, web_view, app_config);
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
                app_menu::handle_menu_click(&web_view, &mut app_config, menu_id);
            }
            _ => (),
        }
    });
}
