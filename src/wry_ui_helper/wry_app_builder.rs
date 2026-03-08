use super::WryWebViewApp;
use super::{WebViewIPCHandler, WryAppEventHandler, WryWebViewAppBuilder, WryWorkerFunction};
use crate::wry_ui_helper::common::{
    LOCAL_APP_WEBVIEW_URL, WRY_APP_BUILDER_DEFAULT_HEIGHT_PX, WRY_APP_BUILDER_DEFAULT_TITLE,
    WRY_APP_BUILDER_DEFAULT_WIDTH_PX,
};
use crate::wry_ui_helper::stop_signal::StopSignal;
use std::thread::JoinHandle;
use tao::event_loop::EventLoopBuilder;

impl<T: Send + 'static> WryWebViewAppBuilder<T> {
    pub fn new() -> Self {
        let devtools_enabled = true;
        #[cfg(not(debug_assertions))]
        {
            devtools_enabled = false;
        }

        Self {
            ui_size_width_px: WRY_APP_BUILDER_DEFAULT_WIDTH_PX,
            ui_size_height_px: WRY_APP_BUILDER_DEFAULT_HEIGHT_PX,
            worker_functions: vec![],
            webview_html: None,
            webview_url: LOCAL_APP_WEBVIEW_URL.to_string(),
            devtools_enabled,
            event_handler: None,

            // Required
            ui_title_name: WRY_APP_BUILDER_DEFAULT_TITLE.to_string(),
            webview_ipc_handler: None,
        }
    }

    pub fn with_event_handler(mut self, event_handler: WryAppEventHandler<T>) -> Self {
        self.event_handler = Some(event_handler);
        self
    }

    pub fn with_worker_thread(mut self, thread: WryWorkerFunction<T>) -> Self {
        self.worker_functions.push(thread);
        self
    }

    pub fn with_ipc_handler(mut self, ipc_handler: WebViewIPCHandler<T>) -> Self {
        self.webview_ipc_handler = Some(ipc_handler);
        self
    }

    pub fn with_title_name(mut self, title: impl Into<String>) -> Self {
        self.ui_title_name = title.into();
        self
    }

    pub fn with_window_size(mut self, width: impl Into<u32>, height: impl Into<u32>) -> Self {
        self.ui_size_width_px = width.into();
        self.ui_size_height_px = height.into();
        self
    }

    pub fn with_html(mut self, html: impl Into<String>) -> Self {
        self.webview_html = Some(html.into());
        self
    }

    pub fn with_url(mut self, url: impl Into<String>) -> Self {
        self.webview_url = url.into();
        self
    }

    pub fn with_devtools_enabled(mut self, enabled: impl Into<bool>) -> Self {
        self.devtools_enabled = enabled.into();
        self
    }

    // Actually build
    pub fn build(&mut self) -> WryWebViewApp<T> {
        let event_handler = self
            .event_handler
            .expect("No event handler given for WryApp unable to start");
        let event_loop = EventLoopBuilder::<T>::with_user_event().build();
        let worker_stop_signal = StopSignal::new();
        let (window, webview) = self.build_window_with_webview(&event_loop);
        let worker_threads: Vec<JoinHandle<()>> = self
            .worker_functions
            .drain(..)
            .map(|workmain| {
                let thread_proxy = event_loop.create_proxy();
                let stop_signal = worker_stop_signal.clone();
                return std::thread::spawn(move || workmain(stop_signal, &thread_proxy));
            })
            .collect();

        return WryWebViewApp::<T> {
            webview: webview,
            worker_threads: worker_threads,
            workers_stop_signal: worker_stop_signal,
            _window: window,
            event_handler: event_handler,
            event_loop: Some(event_loop),
        };
    }
}
