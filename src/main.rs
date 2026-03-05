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

// pub fn scan_devices(vendor_id: &str, product_id: &str, name: &str) -> Option<PathBuf> {
//     let mut enumerator = Enumerator::new().ok()?;

//     // Only search input devices
//     enumerator.match_subsystem("input").ok()?;

//     for device in enumerator.scan_devices().ok()? {
//         // We only want event devices
//         let Some(devnode) = device.devnode() else {
//             continue;
//         };
//         if !devnode.to_string_lossy().contains("event") {
//             continue;
//         }

//         let props: std::collections::HashMap<_, _> = device
//             .properties()
//             .map(|p| {
//                 (
//                     p.name().to_string_lossy().to_string(),
//                     p.value().to_string_lossy().to_string(),
//                 )
//             })
//             .collect();

//         let is_joystick = props.get("ID_INPUT_JOYSTICK") == Some(&"1".to_string());
//         if !is_joystick {
//             continue;
//         }
//         let devname = props.get("NAME");
//     }

//     None
// }

// fn main() {
//     let vendor = "045e";
//     let product = "0b00";
//     let name = "Xbox Wireless Controller";

//     match find_event_device(vendor, product, name) {
//         Some(path) => println!("Matched device: {}", path.display()),
//         None => println!("Device not found"),
//     }
// }

use std::{thread, time::Duration};
use udev::{EventType, MonitorBuilder};

/// Returns the first non-empty NAME property from the device or any of its parents
fn get_device_name(mut event: udev::Device) -> Option<String> {
    loop {
        if let Some(name) = event.property_value("NAME") {
            let name_str = name.to_string_lossy().trim_matches('"').to_string();
            if !name_str.is_empty() {
                return Some(name_str);
            }
        }
        // Move to parent, stop if there’s none
        match event.parent() {
            Some(parent) => event = parent,
            None => return None,
        }
    }
}

fn main() -> std::io::Result<()> {
    // Build a monitor for input devices
    let monitor = MonitorBuilder::new()
        .unwrap()
        .match_subsystem("input")
        .unwrap()
        .listen()
        .unwrap();

    println!("Waiting for devices...");

    // Iterate over events
    loop {
        let event = monitor.iter().next();

        let Some(event) = event else {
            thread::sleep(Duration::from_millis(100));
            continue;
        };

        let Some(devnode) = event.devnode() else {
            continue;
        };

        if !devnode.to_string_lossy().contains("event") {
            continue;
        }

        let is_joystick = event
            .property_value("ID_INPUT_JOYSTICK")
            .map(|val| val.to_string_lossy().to_string())
            == Some("1".to_string());

        if !is_joystick {
            continue;
        }

        println!(
            "Action: {:?}, Devnode: {:?}, Name: {:?}",
            event.event_type(),
            event.devnode(),
            get_device_name(event.clone()) // event.property_value("NAME")
        );

        match event.event_type() {
            EventType::Add => println!("Device added!"),
            EventType::Remove => println!("Device removed!"),
            EventType::Change => println!("Device changed!"),
            _ => {}
        }
    }
}
