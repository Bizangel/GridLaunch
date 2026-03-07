use std::{cell::RefCell, rc::Rc};
use wry::{
    Rect, WebView,
    dpi::{LogicalPosition, LogicalSize},
};

use crate::gamepad::AppGamepadButton;
use tao::{event::WindowEvent, event_loop::ControlFlow, keyboard::Key, window::Window};

pub fn handle_window_event(
    event: &WindowEvent,
    control_flow: &mut ControlFlow,
    window: &Window,
    webview: &Rc<RefCell<WebView>>,
) {
    match event {
        WindowEvent::Resized(new_size) => {
            let logical_size = new_size.to_logical::<f64>(window.scale_factor());
            webview
                .borrow()
                .set_bounds(Rect {
                    position: LogicalPosition::new(0, 0).into(),
                    size: LogicalSize::new(logical_size.width, logical_size.height).into(),
                })
                .unwrap();
        }
        WindowEvent::KeyboardInput { event, .. } => {
            let key = &event.logical_key;
            match key {
                Key::Character("i") => {
                    #[cfg(debug_assertions)]
                    {
                        webview.borrow().open_devtools();
                    }
                }
                Key::Character("t") => {
                    #[cfg(debug_assertions)]
                    {
                        use crate::ui::common::send_event_to_webview;

                        println!("Sending event!");
                        send_event_to_webview(
                            &webview.borrow(),
                            &crate::ui::common::ToWebViewEvent::GamepadButtonPressed {
                                button: AppGamepadButton::A,
                                release: false,
                            },
                        );
                    }
                }
                _ => {}
            }
        }
        WindowEvent::CloseRequested => {
            *control_flow = ControlFlow::ExitWithCode(1);
        }
        _ => {}
    }
}
