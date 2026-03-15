use super::WryWebViewApp;

impl<T: Send + 'static, S: 'static, M: 'static + Clone> WryWebViewApp<T, S, M> {
    pub fn webview_eval(&self, script: &str) {
        let _ = self.webview.evaluate_script(script);
    }

    pub fn emit(&self, event: T) {
        let _ = self.ui_proxy.send_event(event);
    }

    pub fn broadcast_to_workers(&self, event: M) {
        for tx in self.worker_txs.iter() {
            let _ = tx.send(event.clone());
        }
    }
}
