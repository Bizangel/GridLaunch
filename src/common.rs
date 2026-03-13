use evdev::Device as EvdevDevice;
use std::path::PathBuf;

pub const RUNAS_SCRIPT_PATH: &str = "/home/arcanzu/scripts/gaming/run_as_user_gaming.sh";

#[derive(Debug)]
pub struct AppGamepad {
    pub name: String,
    pub devnode: PathBuf,
    pub evdev_device: EvdevDevice,
}
