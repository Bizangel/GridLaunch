use std::{
    cell::RefCell,
    path::PathBuf,
    sync::mpsc::{self, Receiver, Sender},
};

use tao::event_loop::EventLoopBuilder;
use wry::http::Request;

use crate::ui::{
    common::UIEvent, handle_main_loop_event::handle_main_loop_event,
    window_builder::build_window_with_webview,
};

pub fn ui_loop_main() -> Result<(), String> {
    let event_loop = EventLoopBuilder::<UIEvent>::with_user_event().build();
    // let event_proxy = event_loop.create_proxy();

    let webview_ipc_handler = move |req: Request<String>| {
        // IPC handler code
    };

    let (window, webview) = build_window_with_webview(&event_loop, webview_ipc_handler);

    event_loop.run(move |event, _, control_flow| {
        handle_main_loop_event(event, control_flow, &webview, &window);
    });
}
