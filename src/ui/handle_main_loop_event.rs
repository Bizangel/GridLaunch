use crate::ui::{
    common::{AppEvent, send_event_to_webview},
    handle_window_event::handle_window_event,
};
use std::{
    sync::{Arc, atomic::AtomicBool},
    thread::JoinHandle,
};

use std::{cell::RefCell, rc::Rc};
use tao::{event::Event, event_loop::ControlFlow, window::Window};
use wry::WebView;

pub fn handle_main_loop_event(
    event: Event<AppEvent>,
    control_flow: &mut ControlFlow,
    webview: &Rc<RefCell<WebView>>,
    window: &Window,
    stop_signal: &Arc<AtomicBool>,
    worker_threads: &mut Vec<JoinHandle<()>>,
) {
    match event {
        Event::WindowEvent { event, .. } => handle_window_event(
            &event,
            control_flow,
            window,
            webview,
            &stop_signal,
            worker_threads,
        ),
        Event::UserEvent(event) => match event {
            // handle your custom UIEvent
            AppEvent::ForwardToWebViewEvent(event) => {
                let _ = send_event_to_webview(&webview.borrow(), &event);
            }
            _ => println!("event: {:#?}", event),
        },
        _ => {}
    }
}
