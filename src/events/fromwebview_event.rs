use serde::Deserialize;

use crate::common::SplitscreenType;

#[derive(Debug, Clone, Deserialize)]
pub struct LaunchRequestedEvent {
    pub splitscreen_type: SplitscreenType,
    pub users: Vec<String>,
    pub gamepads: Vec<String>,
    pub game: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "type")]
pub enum FromWebViewEvent {
    LaunchRequested(LaunchRequestedEvent),
    WebViewReady,
}
