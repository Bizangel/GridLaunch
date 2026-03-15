use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct LaunchRequestedEvent {
    pub splitscreen_type: String,
    pub users: Vec<String>,
    pub gamepads: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "type")]
pub enum FromWebViewEvent {
    LaunchRequested(LaunchRequestedEvent),
    WebViewReady,
}
