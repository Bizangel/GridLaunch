use crate::wry_ui_helper::stop_signal::StopSignal;
use std::thread::JoinHandle;
use tao::event_loop::{EventLoop, EventLoopProxy};
use tao::window::Window;
use wry::{WebView, http::Request};

pub type AppProxy<T> = EventLoopProxy<T>;
pub type AppEventHandler<T> = fn(event: T, &mut WryWebViewApp<T>); // TODO: add state
pub type IPCHandler<T> = fn(Request<String>, &AppProxy<T>);
pub type WorkerTask<T> = fn(StopSignal, &AppProxy<T>);

pub struct WryWebViewAppBuilder<T: Send + 'static> {
    ui_title_name: String,
    event_handler: Option<AppEventHandler<T>>,
    worker_functions: Vec<WorkerTask<T>>,
    webview_ipc_handler: Option<IPCHandler<T>>,
    webview_url: String,
    webview_html: Option<String>,
    devtools_enabled: bool,

    ui_size_width_px: u32,
    ui_size_height_px: u32,
}

pub struct WryWebViewApp<T: Send + 'static> {
    worker_threads: Vec<JoinHandle<()>>,
    workers_stop_signal: StopSignal,
    webview: WebView,
    _window: Window,
    event_handler: AppEventHandler<T>,
    event_loop: Option<EventLoop<T>>,
    ui_proxy: AppProxy<T>,
}

pub mod common;
pub mod stop_signal;
pub mod wry_app_builder;
pub mod wry_app_handle_window_event;
pub mod wry_app_run;
pub mod wry_webview_app;
pub mod wry_window_builder;
