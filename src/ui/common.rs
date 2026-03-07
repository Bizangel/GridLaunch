use serde::Deserialize;

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
#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "type", content = "event")]
pub enum AppEvent {
    FromWebViewEvent(FromWebViewEvent),
}
