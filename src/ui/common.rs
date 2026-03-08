use serde::{Deserialize, Serialize};
use wry::WebView;

use crate::gamepad::AppGamepadButtonEvent;

pub const UI_TITLE_NAME: &str = "GridLaunch";
pub const UI_INITIAL_SIZE_WIDTH_PX: f64 = 1280.0;
pub const UI_INITIAL_SIZE_HEIGHT_PX: f64 = 720.0;
pub const VITE_DEV_LOCALHOST_URL: &str = "http://localhost:5173";

#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "type")]
pub enum FromWebViewEvent {
    LaunchRequested,
}

// Events generated to be handled for the main loop
#[derive(Debug, Clone)]
pub enum AppEvent {
    FromWebViewEvent(FromWebViewEvent),
    ForwardToWebViewEvent(ToWebViewEvent),
}

#[derive(Debug, Clone, Serialize)]
#[serde(tag = "type")]
pub enum ToWebViewEvent {
    AppGamepadButtonEvent(AppGamepadButtonEvent),
}

pub fn send_event_to_webview(webview: &WebView, ev: &ToWebViewEvent) {
    let Ok(evpayload) = serde_json::to_string(&ev) else {
        return;
    };

    let script = format!("window.postMessage({}, '*');", evpayload);
    let _ = webview.evaluate_script(&script);
}
