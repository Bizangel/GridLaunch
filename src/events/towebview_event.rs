use serde::Serialize;

use crate::{
    events::worker_event::GamepadsUpdateEvent, game_handler::GameHandler,
    gamepad::AppGamepadButtonEvent, user_profile::UserProfile,
};

#[derive(Debug, Clone, Serialize)]
#[serde(tag = "type")]
pub enum ToWebViewEvent {
    AppGamepadButtonEvent(AppGamepadButtonEvent),
    GamepadsUpdate(GamepadsUpdateEvent),
    GameHandlersUpdate { handlers: Vec<GameHandler> },
    ProfilesUpdate { profiles: Vec<UserProfile> },
    LaunchReturned,
}
