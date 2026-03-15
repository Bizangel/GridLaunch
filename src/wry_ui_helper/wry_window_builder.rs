use super::WryWebViewAppBuilder;
use std::borrow::Cow;
use tao::{
    event_loop::EventLoop,
    window::{Window, WindowBuilder},
};
use wry::{
    WebView,
    http::{Response, header::CONTENT_TYPE},
};
use wry::{WebViewBuilder, http::Request};

impl<T: Send + 'static, S: 'static, M: 'static + Send> WryWebViewAppBuilder<T, S, M> {
    pub fn build_window_with_webview(&mut self, event_loop: &EventLoop<T>) -> (Window, WebView) {
        let window = WindowBuilder::new()
            .with_title(self.ui_title_name.clone())
            .with_inner_size(tao::dpi::LogicalSize::new(
                self.ui_size_width_px,
                self.ui_size_height_px,
            ))
            .build(&event_loop)
            .expect("Failed to create window");

        let mut disable_right_click_menu_script = "";
        if !self.devtools_enabled {
            disable_right_click_menu_script = r#"""
            document.addEventListener('contextmenu', event => event.preventDefault());
        """#;
        }

        let htmlbytes: Vec<u8> = match &self.webview_html {
            Some(content) => content.as_bytes().to_vec(),
            None => Vec::new(),
        };
        let builder = WebViewBuilder::new()
            .with_url(self.webview_url.clone())
            .with_devtools(self.devtools_enabled)
            .with_initialization_script(disable_right_click_menu_script)
            .with_custom_protocol("app".into(), move |_, request| {
                let path = request.uri().path();
                if path == "/" || path == "/index.html" {
                    Response::builder()
                        .header(CONTENT_TYPE, "text/html")
                        .body(Cow::Owned(htmlbytes.clone()))
                        .unwrap()
                } else {
                    Response::builder()
                        .status(404)
                        .body(Vec::new().into())
                        .unwrap()
                }
            });

        let builder = match self.webview_ipc_handler {
            Some(handler) => {
                let ipc_handler_proxy = event_loop.create_proxy();
                let ipc_handler = move |req: Request<String>| {
                    handler(req, &ipc_handler_proxy);
                };
                builder.with_ipc_handler(ipc_handler)
            }
            None => builder,
        };

        #[cfg(any(
            target_os = "windows",
            target_os = "macos",
            target_os = "ios",
            target_os = "android"
        ))]
        let _webview = builder.build(&window)?;
        #[cfg(not(any(
            target_os = "windows",
            target_os = "macos",
            target_os = "ios",
            target_os = "android"
        )))]
        let webview = {
            use tao::platform::unix::WindowExtUnix;
            use wry::WebViewBuilderExtUnix;
            let vbox = window.default_vbox().unwrap();
            builder
                .build_gtk(vbox)
                .expect("Failed to build GTK webview")
        };

        return (window, webview);
    }
}
