mod app_config;
mod app_menu;
mod app_data;
use app_config::AppConfig;
mod js_scripts;

use wry::{
    application::{
        event::{Event, StartCause, WindowEvent},
        event_loop::{ControlFlow, EventLoop},
        window::WindowBuilder,
    },
    webview,
    webview::WebViewBuilder,
};

fn main() -> wry::Result<()> {
    let user_agent: Option<&str> = None;
    let app_config = AppConfig::load().unwrap_or(AppConfig::default());
    let event_loop = EventLoop::new();

    let window = WindowBuilder::new()
        .with_title(app_data::APP_NAME)
        .with_menu(app_menu::build_menu(&app_config))
        .build(&event_loop)?;

    let mut web_view_builder = WebViewBuilder::new(window)?
        .with_devtools(true)
        .with_initialization_script(js_scripts::INIT)
        .with_url(app_data::URL)?;

    if let Some(user_agent) = user_agent {
        web_view_builder = web_view_builder.with_user_agent(user_agent);
    }

    let web_view = web_view_builder.build()?;

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
