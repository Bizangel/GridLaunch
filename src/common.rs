use evdev::Device as EvdevDevice;
use std::path::PathBuf;

pub const RUNAS_SCRIPT_PATH: &str = "/home/arcanzu/scripts/gaming/run_as_user_gaming.sh";

#[derive(Debug)]
pub struct AppGamepad {
    pub name: String,
    pub devnode: PathBuf,
    pub evdev_device: EvdevDevice,
    pub _leftstick_x_prev: f32,
    pub _leftstick_y_prev: f32,
}

impl AppGamepad {
    pub fn new(devnode: PathBuf, name: String, dev: EvdevDevice) -> AppGamepad {
        return AppGamepad {
            devnode: devnode,
            name: name,
            evdev_device: dev,
            _leftstick_x_prev: 0.0,
            _leftstick_y_prev: 0.0,
        };
    }
}
