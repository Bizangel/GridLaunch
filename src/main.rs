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

use gridlaunch::gamepad_monitor::gamepad_monitor_worker_main;
use gridlaunch::ui;

fn main() -> Result<(), String> {
    ui::ui_loop_main::ui_loop_main(vec![gamepad_monitor_worker_main])
}
