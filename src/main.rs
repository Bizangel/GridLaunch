pub const UI_TITLE_NAME: &str = "GridLaunch";
pub const VITE_DEV_LOCALHOST_URL: &str = "http://localhost:5173";

// use std::io::{BufRead, BufReader};
// use std::process::{Command, Stdio};
// use std::thread;
// fn launch_game(runas: &str) {
//     unsafe {
//         std::env::set_var("SDL_VIDEODRIVER", "x11");
//     }

//     let args = vec![
//         "gamescope",
//         "-w",
//         "1920",
//         "-h",
//         "1080",
//         "--",
//         "bwrap",
//         "--die-with-parent",
//         "--dev-bind",
//         "/",
//         "/",
//         "steam",
//         "-console",
//         "steam://open/bigpicture",
//     ];

//     let mut child = Command::new("/home/arcanzu/scripts/gaming/run_as_user_gaming.sh")
//         .arg(runas)
//         .args(args)
//         .env("ENABLE_GAMESCOPE_WSI", "0")
//         .stdout(Stdio::piped())
//         .stderr(Stdio::piped())
//         .spawn()
//         .expect("failed to execute process");

//     let stdout = BufReader::new(child.stdout.take().unwrap());
//     let stderr = BufReader::new(child.stderr.take().unwrap());

//     let out_handle = thread::spawn(move || {
//         for line in stdout.lines() {
//             println!("{}", line.unwrap());
//         }
//     });
//     let err_handle = thread::spawn(move || {
//         for line in stderr.lines() {
//             eprintln!("{}", line.unwrap());
//         }
//     });

//     child.wait().unwrap();
//     out_handle.join().unwrap();
//     err_handle.join().unwrap();

//     println!("done")
// }

// fn main() {
//     let game_users = vec!["game-user", "game-user-giluxe"];

//     let mut handles = Vec::new();

//     for user in game_users {
//         handles.push(thread::spawn(move || {
//             launch_game(user);
//         }));
//     }

//     for handle in handles {
//         handle.join().unwrap();
//     }
// }

// use std::path::PathBuf;
// use udev::Enumerator;

// fn main() {
//     let vendor = "045e";
//     let product = "0b00";
//     let name = "Xbox Wireless Controller";

//     match find_event_device(vendor, product, name) {
//         Some(path) => println!("Matched device: {}", path.display()),
//         None => println!("Device not found"),
//     }
// }

use gridlaunch::events::{FromWebViewEvent, GridLaunchEvent};
use gridlaunch::gamepad_monitor::gamepad_monitor_worker_main;
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
