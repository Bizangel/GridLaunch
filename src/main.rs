pub const UI_TITLE_NAME: &str = "GridLaunch";
pub const VITE_DEV_LOCALHOST_URL: &str = "http://localhost:5173";

use gridlaunch::events::{FromWebViewEvent, GridLaunchEvent};
use gridlaunch::gamepad_monitor::gamepad_monitor_worker_main;
use gridlaunch::launch::spawn_games;
use gridlaunch::wry_ui_helper::{WryWebViewApp, WryWebViewAppBuilder};
use tao::event_loop::EventLoopProxy;
use wry::http::Request;

fn main() -> Result<(), String> {
    let webview_ipc_handler =
        move |req: Request<String>, event_proxy: &EventLoopProxy<GridLaunchEvent>| {
            match serde_json::from_str::<FromWebViewEvent>(req.body()) {
                Ok(ipc_req) => {
                    let _ = event_proxy.send_event(GridLaunchEvent::FromWebViewEvent(ipc_req));
                }
                Err(err) => eprintln!("Unrecognized Event from Webview: {}", err),
            };
        };

    let event_handler =
        move |event: GridLaunchEvent, app: &mut WryWebViewApp<GridLaunchEvent>| match event {
            GridLaunchEvent::ForwardToWebViewEvent(event) => {
                let Ok(evpayload) = serde_json::to_string(&event) else {
                    return;
                };

                let script = format!("window.postMessage({}, '*');", evpayload);
                app.webview_eval(&script);
            }
            GridLaunchEvent::FromWebViewEvent(event) => match event {
                FromWebViewEvent::LaunchRequested(launch_event) => {
                    spawn_games(launch_event);
                }
            },
            _ => println!("Received event: {:#?}", event),
        };

    let mut builder = WryWebViewAppBuilder::new()
        .with_title_name(UI_TITLE_NAME)
        .with_ipc_handler(webview_ipc_handler)
        .with_worker_thread(gamepad_monitor_worker_main)
        .with_event_handler(event_handler);
    #[cfg(debug_assertions)]
    {
        builder = builder.with_url(VITE_DEV_LOCALHOST_URL);
    }

    let app = builder.build();
    app.run()
}
