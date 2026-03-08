use std::{
    sync::{Arc, atomic::AtomicBool},
    thread::JoinHandle,
};
use tao::event_loop::EventLoopBuilder;
use wry::http::Request;

use crate::ui::{
    common::AppEvent, handle_main_loop_event::handle_main_loop_event,
    window_builder::build_window_with_webview,
};

type WorkerThreadMain = fn(Arc<AtomicBool>);

pub fn ui_loop_main(workers: Vec<WorkerThreadMain>) -> Result<(), String> {
    let event_loop = EventLoopBuilder::<AppEvent>::with_user_event().build();
    let event_proxy = event_loop.create_proxy();
    let worker_stop_signal = Arc::new(AtomicBool::new(false));

    let webview_ipc_handler = move |req: Request<String>| {
        match serde_json::from_str::<AppEvent>(req.body()) {
            Ok(ipc_req) => {
                let _ = event_proxy.send_event(ipc_req);
            }
            Err(err) => eprintln!("Unrecognized Event from Webview: {}", err),
        };
    };
    let (window, webview) = build_window_with_webview(&event_loop, webview_ipc_handler);
    let mut worker_threads: Vec<JoinHandle<()>> = workers
        .into_iter()
        .map(|workmain| {
            let stop_signal = Arc::clone(&worker_stop_signal);
            return std::thread::spawn(move || workmain(stop_signal));
        })
        .collect();

    event_loop.run(move |event, _, control_flow| {
        handle_main_loop_event(
            event,
            control_flow,
            &webview,
            &window,
            &worker_stop_signal,
            &mut worker_threads,
        );
    });
}
