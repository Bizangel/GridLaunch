use evdev::Device as EvdevDevice;
use std::path::PathBuf;

#[derive(Debug)]
pub struct AppGamepad {
    pub name: String,
    pub devnode: PathBuf,
    pub evdev_device: EvdevDevice,
}
