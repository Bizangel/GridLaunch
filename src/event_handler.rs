use crate::{
    events::{
        fromwebview_event::FromWebViewEvent, gridlaunch_event::GridLaunchEvent,
        towebview_event::ToWebViewEvent, worker_event::GridLaunchWorkerEvent,
    },
    game_handler::{GameHandler, get_valid_game_handlers},
    wry_ui_helper::WryWebViewApp,
};

pub struct GridLaunchState {
    pub game_handlers: Vec<GameHandler>,
}

pub fn handle_event(
    event: GridLaunchEvent,
    app: &mut WryWebViewApp<GridLaunchEvent, GridLaunchState, GridLaunchWorkerEvent>,
) {
    match event {
        GridLaunchEvent::ForwardToWebViewEvent(event) => match serde_json::to_string(&event) {
            Ok(evpayload) => {
                let script = format!("window.postMessage({}, '*');", evpayload);
                app.webview_eval(&script);
            }
            Err(err) => {
                eprintln!("Unable to send event to webview {}", err.to_string())
            }
        },
        GridLaunchEvent::FromWebViewEvent(event) => match event {
            FromWebViewEvent::LaunchRequested(launch_event) => {
                // Find handler and pass handler.
                let Some(handler) = app
                    .state
                    .game_handlers
                    .iter()
                    .find(|&x| x.name == launch_event.game)
                else {
                    eprintln!("Invalid game given from UI: {}", launch_event.game);
                    return;
                };

                // Validate gamepads
                if !launch_event
                    .gamepads
                    .iter()
                    .all(|x| x.starts_with("/dev/input/event"))
                {
                    eprintln!(
                        "Invalid gamepads given from UI {:#?}",
                        launch_event.gamepads
                    );
                    return;
                }

                // TODO: validate users are valid.

                app.broadcast_to_workers(GridLaunchWorkerEvent::SpawnInstances {
                    request: launch_event,
                    handler: handler.clone(),
                });
            }
            FromWebViewEvent::WebViewReady => {
                app.broadcast_to_workers(GridLaunchWorkerEvent::EmitGamepadUpdate);
                app.emit(GridLaunchEvent::ForwardToWebViewEvent(
                    ToWebViewEvent::GameHandlersUpdate {
                        handlers: app.state.game_handlers.clone(),
                    },
                ));
            }
        },
    }
}
