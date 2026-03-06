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

use evdev::Device as EvdevDevice;
// use udev::Moni
use std::{collections::HashMap, path::PathBuf, thread, time::Duration};
use udev::{Device, Enumerator, EventType, MonitorBuilder};

pub fn is_joystick(event: &Device) -> bool {
    let Some(devnode) = event.devnode() else {
        return false;
    };

    if !devnode.to_string_lossy().contains("event") {
        return false;
    }

    let is_joystick = event
        .property_value("ID_INPUT_JOYSTICK")
        .map(|val| val.to_string_lossy().to_string())
        == Some("1".to_string());

    return is_joystick;
}

// pub fn scan_devices(vendor_id: &str, product_id: &str, name: &str) -> Option<PathBuf> {
//     let mut enumerator = Enumerator::new().ok()?;
//     enumerator.match_subsystem("input").ok()?;

//     for device in enumerator.scan_devices().ok()? {
//         if !is_joystick(&device) {
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

fn get_device_name_with_unk_default(dev: &Device) -> String {
    get_device_name(dev.clone()).unwrap_or_else(|| "Unknown".into())
}

use udev::MonitorSocket;

#[derive(Debug)]
pub struct AppGamepad {
    name: String,
    devnode: PathBuf,
    evdev_device: EvdevDevice,
}

pub struct GridLaunch {
    devices: HashMap<PathBuf, AppGamepad>,
    udev_monitor: MonitorSocket,
}

impl GridLaunch {
    fn new() -> Result<GridLaunch, String> {
        let monitor: MonitorSocket = MonitorBuilder::new()
            .unwrap()
            .match_subsystem("input")
            .unwrap()
            .listen()
            .map_err(|err| err.to_string())?;

        Ok(GridLaunch {
            devices: HashMap::new(),
            udev_monitor: monitor,
        })
    }

    fn handle_udev_input_monitor_events(&mut self) {
        for event in self.udev_monitor.iter() {
            if !is_joystick(&event) {
                continue;
            }

            let Some(devnode) = event.devnode() else {
                continue;
            };

            match event.event_type() {
                EventType::Add => {
                    if self.devices.contains_key(devnode) {
                        continue;
                    }
                    match EvdevDevice::open(&devnode) {
                        Ok(dev) => {
                            let gamepad = AppGamepad {
                                devnode: devnode.to_path_buf(),
                                name: get_device_name_with_unk_default(&event),
                                evdev_device: dev,
                            };
                            let gamepadname = gamepad.name.clone();
                            self.devices.insert(devnode.to_path_buf(), gamepad);
                            println!("Added controller: {}", gamepadname);
                        }
                        Err(e) => eprintln!("Failed to open {:?}: {}", devnode, e),
                    }
                }
                EventType::Remove => match self.devices.remove(devnode) {
                    Some(dev) => println!("Removed controller: {:#?}", dev.name),
                    None => {}
                },
                _ => {}
            }
        }
    }

    fn main_poll(&mut self) {
        loop {
            self.handle_udev_input_monitor_events();
            thread::sleep(Duration::from_millis(100));
        }
    }
}

fn main() -> Result<(), String> {
    let mut app = GridLaunch::new()?;

    app.main_poll();
    Ok(())

    // Map of devnode -> (evdev device, name)
    // let mut devices: HashMap<PathBuf, (EvdevDevice, String)> = HashMap::new();

    // Iterate over events

    // // Poll input events
    // for (devnode, (device, name)) in devices.iter_mut() {
    //     let Ok(events) = device.fetch_events() else {
    //         continue;
    //     };

    //     for ev in events {
    //         let evsumm = ev.destructure();
    //         println!("Name: {:#?} on {}{:#?}", devnode, name, evsumm);
    //     }
    // }
}
