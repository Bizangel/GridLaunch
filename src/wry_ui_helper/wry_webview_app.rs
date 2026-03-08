use super::WryWebViewApp;

impl<T: Send + 'static> WryWebViewApp<T> {
    pub fn webview_eval(&self, script: &str) {
        let _ = self.webview.evaluate_script(script);
    }

    pub fn emit(&self, event: T) {
        let _ = self.ui_proxy.send_event(event);
    }
}
