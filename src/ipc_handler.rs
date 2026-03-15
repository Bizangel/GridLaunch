use tao::event_loop::EventLoopProxy;
use wry::http::Request;

use crate::events::{fromwebview_event::FromWebViewEvent, gridlaunch_event::GridLaunchEvent};

pub fn ipc_handler(req: Request<String>, event_proxy: &EventLoopProxy<GridLaunchEvent>) {
    match serde_json::from_str::<FromWebViewEvent>(req.body()) {
        Ok(ipc_req) => {
            let _ = event_proxy.send_event(GridLaunchEvent::FromWebViewEvent(ipc_req));
        }
        Err(err) => eprintln!("Unrecognized Event from Webview: {}", err),
    };
}
