use crate::gamepad::AppGamepadButtonEvent;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize)]
pub struct LaunchRequestedEvent {
    splitscreen_type: String,
    users: Vec<String>,
    gamepads: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "type")]
pub enum FromWebViewEvent {
    LaunchRequested(LaunchRequestedEvent),
}

// Events generated to be handled for the main loop
#[derive(Debug, Clone)]
pub enum GridLaunchEvent {
    FromWebViewEvent(FromWebViewEvent),
    ForwardToWebViewEvent(ToWebViewEvent),
}

#[derive(Debug, Clone, Serialize)]
#[serde(tag = "type")]
pub enum ToWebViewEvent {
    AppGamepadButtonEvent(AppGamepadButtonEvent),
}
