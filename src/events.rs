use crate::gamepad::AppGamepadButtonEvent;
use serde::{Deserialize, Serialize};

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
