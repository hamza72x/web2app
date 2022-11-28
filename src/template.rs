pub const MAIN_RS: &str = "fn main() -> wry::Result<()> {
    use wry::{
      application::{
        event::{Event, StartCause, WindowEvent},
        event_loop::{ControlFlow, EventLoop},
        window::WindowBuilder,
      },
      webview::WebViewBuilder,
    };
  
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
      .with_title(\"%name%\")
      .build(&event_loop)?;

    let _webview = WebViewBuilder::new(window)?
      .with_url(\"%url%\")?
      .build()?;
  
    event_loop.run(move |event, _, control_flow| {
      *control_flow = ControlFlow::Wait;
  
      match event {
        Event::NewEvents(StartCause::Init) => println!(\"Wry has started!\"),
        Event::WindowEvent {
          event: WindowEvent::CloseRequested,
          ..
        } => *control_flow = ControlFlow::Exit,
        _ => (),
      }
    });
  }
";

pub static CARGO_TOML: &str = "[package]
name = \"%name_lower_cased%\"
description = \"%description%\"
version = \"%version%\"
edition = \"2021\"
path = \"src/main.rs\"

[dependencies]
wry = \"*\"

[package.metadata.bundle]
name = \"%name%\"
identifier = \"%identifier%\"
icon = [\"32x32.png\", \"128x128.png\"]
version = \"%version%\"
resources = []
copyright = \"Copyright © %author%\"
category = \"Developer Tool\"
short_description = \"An example application.\"
long_description = \"\"\"
Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do
eiusmod tempor incididunt ut labore et dolore magna aliqua.  Ut
enim ad minim veniam, quis nostrud exercitation ullamco laboris
nisi ut aliquip ex ea commodo consequat.
\"\"\"
deb_depends = []
osx_frameworks = []
osx_url_schemes = []
";
