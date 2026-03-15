use super::WryWebViewApp;
use super::{AppEventHandler, IPCHandler, WorkerTask, WryWebViewAppBuilder};
use crate::wry_ui_helper::common::{
    LOCAL_APP_WEBVIEW_URL, WRY_APP_BUILDER_DEFAULT_HEIGHT_PX, WRY_APP_BUILDER_DEFAULT_TITLE,
    WRY_APP_BUILDER_DEFAULT_WIDTH_PX,
};
use crate::wry_ui_helper::stop_signal::StopSignal;
use std::sync::mpsc::{self, Receiver, Sender};
use std::thread::JoinHandle;
use tao::event_loop::EventLoopBuilder;

impl<T: Send + 'static, S: 'static, M: 'static + Send> WryWebViewAppBuilder<T, S, M> {
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
            initial_state: None,
        }
    }

    pub fn with_event_handler(mut self, event_handler: AppEventHandler<T, S, M>) -> Self {
        self.event_handler = Some(event_handler);
        self
    }

    pub fn with_worker_thread(mut self, thread: WorkerTask<T, M>) -> Self {
        self.worker_functions.push(thread);
        self
    }

    pub fn with_ipc_handler(mut self, ipc_handler: IPCHandler<T>) -> Self {
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

    pub fn with_initial_state(mut self, state: impl Into<S>) -> Self {
        self.initial_state = Some(state.into());
        self
    }

    // Actually build
    pub fn build(&mut self) -> WryWebViewApp<T, S, M> {
        let event_handler = self
            .event_handler
            .expect("No event handler given for WryApp unable to start");
        let init_state = self
            .initial_state
            .take()
            .expect("No initial state given for WryApp unable to start");
        let event_loop = EventLoopBuilder::<T>::with_user_event().build();
        let worker_stop_signal = StopSignal::new();
        let (window, webview) = self.build_window_with_webview(&event_loop);

        let mut worker_threads: Vec<JoinHandle<()>> = vec![];
        let mut worker_txs: Vec<Sender<M>> = vec![];
        for worker_main in self.worker_functions.drain(..) {
            let (tx, rx): (Sender<M>, Receiver<M>) = mpsc::channel();
            let thread_proxy = event_loop.create_proxy();
            let stop_signal = worker_stop_signal.clone();
            worker_threads.push(std::thread::spawn(move || {
                worker_main(stop_signal, rx, thread_proxy)
            }));
            worker_txs.push(tx);
        }

        return WryWebViewApp::<T, S, M> {
            webview: webview,
            worker_threads: worker_threads,
            worker_txs: worker_txs,
            workers_stop_signal: worker_stop_signal,
            _window: window,
            event_handler: event_handler,
            ui_proxy: event_loop.create_proxy(),
            event_loop: Some(event_loop),
            state: init_state,
        };
    }
}
