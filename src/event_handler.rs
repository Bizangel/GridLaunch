use crate::{
    events::{
        fromwebview_event::FromWebViewEvent, gridlaunch_event::GridLaunchEvent,
        towebview_event::ToWebViewEvent, worker_event::GridLaunchWorkerEvent,
    },
    game_handler::GameHandler,
    user_profile::UserProfile,
    wry_ui_helper::WryWebViewApp,
};

pub struct GridLaunchState {
    pub game_handlers: Vec<GameHandler>,
    pub profiles: Vec<UserProfile>,
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

                // validate all users exist.
                for user in launch_event.users.iter() {
                    let Some(_) = app.state.profiles.iter().find(|&prof| &prof.user == user) else {
                        eprintln!("Given non-existing user: {}", user);
                        return;
                    };
                }

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
                app.emit(GridLaunchEvent::ForwardToWebViewEvent(
                    ToWebViewEvent::ProfilesUpdate {
                        profiles: app.state.profiles.clone(),
                    },
                ));
            }
        },
    }
}
