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
use gridlaunch::gamepad::parse_button_event;
use gridlaunch::ui;
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
    gamepads: HashMap<PathBuf, AppGamepad>,
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
            gamepads: HashMap::new(),
            udev_monitor: monitor,
        })
    }

    fn scan_refresh_devices(&mut self) -> Result<(), String> {
        let mut enumerator = Enumerator::new().map_err(|err| err.to_string())?;
        enumerator
            .match_subsystem("input")
            .map_err(|err| err.to_string())?;

        self.gamepads.clear(); // remove all gamepads as we will be re-scanning
        for device in enumerator.scan_devices().map_err(|err| err.to_string())? {
            if !is_joystick(&device) {
                continue;
            }
            let Some(devnode) = device.devnode() else {
                continue;
            };

            match EvdevDevice::open(&devnode) {
                Ok(dev) => {
                    dev.set_nonblocking(true).map_err(|err| err.to_string())?;
                    let gamepad = AppGamepad {
                        devnode: devnode.to_path_buf(),
                        name: get_device_name_with_unk_default(&device),
                        evdev_device: dev,
                    };
                    let gamepadname = gamepad.name.clone();
                    self.gamepads.insert(devnode.to_path_buf(), gamepad);
                    println!("Added controller: {}", gamepadname);
                }
                Err(e) => eprintln!("Failed to open {:?}: {}", devnode, e),
            }
        }

        Ok(())
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
                    if self.gamepads.contains_key(devnode) {
                        continue;
                    }
                    match EvdevDevice::open(&devnode) {
                        Ok(dev) => {
                            if !dev.set_nonblocking(true).is_ok() {
                                continue;
                            }

                            let gamepad = AppGamepad {
                                devnode: devnode.to_path_buf(),
                                name: get_device_name_with_unk_default(&event),
                                evdev_device: dev,
                            };
                            let gamepadname = gamepad.name.clone();
                            self.gamepads.insert(devnode.to_path_buf(), gamepad);
                            println!("Added controller: {}", gamepadname);
                        }
                        Err(e) => eprintln!("Failed to open {:?}: {}", devnode, e),
                    }
                }
                EventType::Remove => match self.gamepads.remove(devnode) {
                    Some(dev) => println!("Removed controller: {:#?}", dev.name),
                    None => {}
                },
                _ => {}
            }
        }
    }

    fn poll_gamepad_inputs(&mut self) {
        // Poll input events
        for gamepad in self.gamepads.values_mut() {
            let Ok(events) = gamepad.evdev_device.fetch_events() else {
                continue;
            };

            for ev in events {
                let btn = parse_button_event(ev);

                match btn {
                    Some(button) => println!("Pressed {:#?}", button),
                    None => {}
                }
            }
        }
    }
    fn main_poll(&mut self) {
        loop {
            self.handle_udev_input_monitor_events();
            self.poll_gamepad_inputs();
            thread::sleep(Duration::from_millis(10));
        }
    }
}

fn main() -> Result<(), String> {
    ui::ui_loop_main::ui_loop_main()

    // let mut app = GridLaunch::new()?;
    // app.scan_refresh_devices()?;
    // app.main_poll();
    // Ok(())
}
