use std::{cell::RefCell, rc::Rc};
use tao::{event::Event, event_loop::ControlFlow, window::Window};

use crate::ui::{common::AppEvent, handle_window_event::handle_window_event};
use wry::WebView;

pub fn handle_main_loop_event(
    event: Event<AppEvent>,
    control_flow: &mut ControlFlow,
    webview: &Rc<RefCell<WebView>>,
    window: &Window,
) {
    match event {
        Event::WindowEvent { event, .. } => {
            handle_window_event(&event, control_flow, window, webview)
        }
        Event::UserEvent(event) => match event {
            // handle your custom UIEvent
            _ => println!("event: {:#?}", event),
        },
        _ => {}
    }
}
