use crate::{
    common::AppGamepad,
    gamepad::{get_device_name_with_unk_default, is_joystick, parse_button_event},
    ui::common::{AppEvent, ToWebViewEvent},
};
use evdev::Device as EvdevDevice;
use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
};
use std::{collections::HashMap, path::PathBuf, thread, time::Duration};
use tao::event_loop::EventLoopProxy;
use udev::{Enumerator, EventType, MonitorBuilder, MonitorSocket};

pub struct GamepadMonitor {
    gamepads: HashMap<PathBuf, AppGamepad>,
    udev_monitor: MonitorSocket,
    ui_loop_proxy: EventLoopProxy<AppEvent>,
}

impl GamepadMonitor {
    fn new(ui_loop_proxy: EventLoopProxy<AppEvent>) -> Result<GamepadMonitor, String> {
        let monitor: MonitorSocket = MonitorBuilder::new()
            .unwrap()
            .match_subsystem("input")
            .unwrap()
            .listen()
            .map_err(|err| err.to_string())?;

        Ok(GamepadMonitor {
            gamepads: HashMap::new(),
            udev_monitor: monitor,
            ui_loop_proxy,
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
                    Some(button_event) => {
                        let _ = self
                            .ui_loop_proxy
                            .send_event(AppEvent::ForwardToWebViewEvent(
                                ToWebViewEvent::AppGamepadButtonEvent(button_event),
                            ));
                    }
                    None => {}
                }
            }
        }
    }
    fn main_poll(&mut self, stop_signal: Arc<AtomicBool>) {
        while !stop_signal.load(Ordering::Relaxed) {
            self.handle_udev_input_monitor_events();
            self.poll_gamepad_inputs();
            thread::sleep(Duration::from_millis(10));
        }
        println!("Stopped polling")
    }
}

fn _gamepad_monitor_worker_main(
    stop_signal: Arc<AtomicBool>,
    ui_proxy: EventLoopProxy<AppEvent>,
) -> Result<(), String> {
    let mut gamepad_monitor = GamepadMonitor::new(ui_proxy)?;
    gamepad_monitor.scan_refresh_devices()?;
    gamepad_monitor.main_poll(stop_signal);

    Ok(())
}

pub fn gamepad_monitor_worker_main(
    stop_signal: Arc<AtomicBool>,
    ui_proxy: EventLoopProxy<AppEvent>,
) {
    let _ = _gamepad_monitor_worker_main(stop_signal, ui_proxy);
}
