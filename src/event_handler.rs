use crate::{
    events::{
        fromwebview_event::FromWebViewEvent, gridlaunch_event::GridLaunchEvent,
        worker_event::GridLaunchWorkerEvent,
    },
    wry_ui_helper::WryWebViewApp,
};

pub fn handle_event(
    event: GridLaunchEvent,
    app: &mut WryWebViewApp<GridLaunchEvent, (), GridLaunchWorkerEvent>,
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
                app.broadcast_to_workers(GridLaunchWorkerEvent::SpawnInstances(launch_event));
            }
            FromWebViewEvent::WebViewReady => {
                app.broadcast_to_workers(GridLaunchWorkerEvent::EmitGamepadUpdate);
            }
        },
    }
}
