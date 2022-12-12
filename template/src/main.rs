use tao::menu::MenuItem;

fn main() -> wry::Result<()> {
    use std::{
        fs::{canonicalize, read},
        path::PathBuf,
    };

    use wry::{
        application::{
            accelerator::Accelerator,
            event::{Event, StartCause, WindowEvent},
            event_loop::{ControlFlow, EventLoop},
            keyboard::{KeyCode, ModifiersState},
            menu::{MenuBar, MenuItemAttributes},
            window::WindowBuilder,
        },
        http::{header::CONTENT_TYPE, Response},
        webview::WebViewBuilder,
    };

    let mut menu = MenuBar::new();

    let mut file_menu = MenuBar::new();
    let mut edit_menu = MenuBar::new();
    let mut window_menu = MenuBar::new();

    file_menu.add_native_item(MenuItem::Quit);

    edit_menu.add_native_item(MenuItem::Copy);
    edit_menu.add_native_item(MenuItem::Cut);
    edit_menu.add_native_item(MenuItem::Paste);
    edit_menu.add_native_item(MenuItem::SelectAll);
    edit_menu.add_native_item(MenuItem::Undo);
    edit_menu.add_native_item(MenuItem::Redo);

    window_menu.add_native_item(MenuItem::Minimize);

    menu.add_submenu("File", true, file_menu);
    menu.add_submenu("Edit", true, edit_menu);
    menu.add_submenu("Window", true, window_menu);

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("%name%")
        .with_menu(menu)
        .build(&event_loop)?;

    let _webview = WebViewBuilder::new(window)?.with_url("%url%")?.build()?;

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::NewEvents(StartCause::Init) => println!("Wry has started!"),
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,
            Event::MenuEvent { menu_id, .. } => {
                println!("Menu clicked! {:?}", menu_id);
                // *control_flow = ControlFlow::Exit;
            }
            _ => (),
        }
    });
}
