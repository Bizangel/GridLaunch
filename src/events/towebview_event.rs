use serde::Serialize;

use crate::{events::worker_event::GamepadsUpdateEvent, gamepad::AppGamepadButtonEvent};

#[derive(Debug, Clone, Serialize)]
#[serde(tag = "type")]
pub enum ToWebViewEvent {
    AppGamepadButtonEvent(AppGamepadButtonEvent),
    GamepadsUpdate(GamepadsUpdateEvent),
}
