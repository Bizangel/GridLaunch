use std::{
    cell::RefCell,
    path::PathBuf,
    sync::mpsc::{self, Receiver, Sender},
};

use tao::event_loop::EventLoopBuilder;
use wry::http::Request;

use crate::ui::{
    common::AppEvent, handle_main_loop_event::handle_main_loop_event,
    window_builder::build_window_with_webview,
};

pub fn ui_loop_main() -> Result<(), String> {
    let event_loop = EventLoopBuilder::<AppEvent>::with_user_event().build();
    let event_proxy = event_loop.create_proxy();
    let webview_ipc_handler = move |req: Request<String>| {
        match serde_json::from_str::<AppEvent>(req.body()) {
            Ok(ipc_req) => {
                let _ = event_proxy.send_event(ipc_req);
            }
            Err(err) => eprintln!("Unrecognized Event from Webview: {}", err),
        };
    };
    let (window, webview) = build_window_with_webview(&event_loop, webview_ipc_handler);

    event_loop.run(move |event, _, control_flow| {
        handle_main_loop_event(event, control_flow, &webview, &window);
    });
}
