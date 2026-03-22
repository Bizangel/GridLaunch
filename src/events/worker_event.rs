use crate::{events::fromwebview_event::LaunchRequestedEvent, game_handler::GameHandler};
use serde::Serialize;
use std::{collections::HashMap, path::PathBuf};

#[derive(Debug, Clone, Serialize)]
pub struct GamepadsUpdateEvent {
    pub gamepads: HashMap<PathBuf, String>,
}

#[derive(Debug, Clone)]
pub enum GridLaunchWorkerEvent {
    EmitGamepadUpdate,
    SpawnInstances {
        request: LaunchRequestedEvent,
        handler: GameHandler,
    },
}
