pub const UI_TITLE_NAME: &str = "GridLaunch";
pub const VITE_DEV_LOCALHOST_URL: &str = "http://localhost:5173";

use gridlaunch::events::fromwebview_event::FromWebViewEvent;
use gridlaunch::events::gridlaunch_event::GridLaunchEvent;
use gridlaunch::events::worker_event::GridLaunchWorkerEvent;
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
        move |event: GridLaunchEvent,
              app: &mut WryWebViewApp<GridLaunchEvent, (), GridLaunchWorkerEvent>| {
            match event {
                GridLaunchEvent::ForwardToWebViewEvent(event) => {
                    match serde_json::to_string(&event) {
                        Ok(evpayload) => {
                            let script = format!("window.postMessage({}, '*');", evpayload);
                            println!("script: {}", script);
                            app.webview_eval(&script);
                        }
                        Err(err) => {
                            eprintln!("Unable to send event to webview {}", err.to_string())
                        }
                    }
                }
                GridLaunchEvent::FromWebViewEvent(event) => match event {
                    FromWebViewEvent::LaunchRequested(launch_event) => {
                        spawn_games(launch_event);
                    }
                    FromWebViewEvent::WebViewReady => {
                        app.broadcast_to_workers(GridLaunchWorkerEvent::EmitGamepadUpdate);
                    }
                },
                _ => println!("Received event: {:#?}", event),
            }
        };

    let mut builder = WryWebViewAppBuilder::new()
        .with_title_name(UI_TITLE_NAME)
        .with_ipc_handler(webview_ipc_handler)
        .with_worker_thread(gamepad_monitor_worker_main)
        .with_event_handler(event_handler)
        .with_initial_state(());
    #[cfg(debug_assertions)]
    {
        builder = builder.with_url(VITE_DEV_LOCALHOST_URL);
    }

    let app = builder.build();
    app.run()
}
