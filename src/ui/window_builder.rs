use std::{cell::RefCell, rc::Rc};
use tao::{
    event_loop::EventLoop,
    window::{Window, WindowBuilder},
};
use wry::{
    Rect, WebViewBuilder,
    dpi::{LogicalPosition, LogicalSize},
    http::Request,
};
use wry::{
    WebView,
    http::{Response, header::CONTENT_TYPE},
};

use crate::ui::common::{
    UI_INITIAL_SIZE_HEIGHT_PX, UI_INITIAL_SIZE_WIDTH_PX, UI_TITLE_NAME, UIEvent,
};
const MINIFIED_HTML_STR: &str = include_str!("../../src-ui/dist/index.html");

pub fn build_window_with_webview<F>(
    event_loop: &EventLoop<UIEvent>,
    webview_ipc_handler: F,
) -> (Window, Rc<RefCell<WebView>>)
where
    F: Fn(Request<String>) + 'static,
{
    let window = WindowBuilder::new()
        .with_title(UI_TITLE_NAME)
        .with_inner_size(tao::dpi::LogicalSize::new(
            UI_INITIAL_SIZE_WIDTH_PX,
            UI_INITIAL_SIZE_HEIGHT_PX,
        ))
        .build(&event_loop)
        .expect("Failed to create window");

    let devtool_enabled;
    let app_url;
    let disable_right_click_menu_script;
    #[cfg(debug_assertions)]
    {
        use crate::ui::common::VITE_DEV_LOCALHOST_URL;
        devtool_enabled = true;
        app_url = VITE_DEV_LOCALHOST_URL;
        disable_right_click_menu_script = "";
    }
    #[cfg(not(debug_assertions))]
    {
        devtool_enabled = false;
        app_url = "app://localhost";
        disable_right_click_menu_script = r#"""
            document.addEventListener('contextmenu', event => event.preventDefault());
        """#;
    }

    let builder = WebViewBuilder::new()
        .with_url(app_url)
        .with_devtools(devtool_enabled)
        .with_initialization_script(disable_right_click_menu_script)
        .with_custom_protocol("app".into(), move |_, request| {
            let path = request.uri().path();

            if path == "/" || path == "/index.html" {
                Response::builder()
                    .header(CONTENT_TYPE, "text/html")
                    .body(MINIFIED_HTML_STR.as_bytes().into())
                    .unwrap()
            } else {
                Response::builder()
                    .status(404)
                    .body(Vec::new().into())
                    .unwrap()
            }
        })
        .with_ipc_handler(webview_ipc_handler);

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
    let _webview = {
        use tao::platform::unix::WindowExtUnix;
        use wry::WebViewBuilderExtUnix;
        let vbox = window.default_vbox().unwrap();
        builder
            .build_gtk(vbox)
            .expect("Failed to build GTK webview")
    };

    let webview = Rc::new(RefCell::new(_webview));

    return (window, webview);
}
