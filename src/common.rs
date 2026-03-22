use evdev::{AbsoluteAxisCode, Device as EvdevDevice};
use serde::Deserialize;
use std::path::PathBuf;

pub const RUNAS_SCRIPT_PATH: &str = "/home/arcanzu/scripts/gaming/run_as_user_gaming.sh";

#[derive(Debug)]
pub struct AppGamepad {
    pub name: String,
    pub devnode: PathBuf,
    pub evdev_device: EvdevDevice,
    pub _leftstick_x_prev: f32,
    pub _leftstick_y_prev: f32,
    pub _leftstick_x_min: Option<i32>,
    pub _leftstick_x_max: Option<i32>,
    pub _leftstick_y_min: Option<i32>,
    pub _leftstick_y_max: Option<i32>,
}

impl AppGamepad {
    pub fn new(devnode: PathBuf, name: String, dev: EvdevDevice) -> AppGamepad {
        let mut xmin: Option<i32> = None;
        let mut xmax: Option<i32> = None;
        let mut ymin: Option<i32> = None;
        let mut ymax: Option<i32> = None;

        for (axis, info) in dev.get_absinfo().expect("Unable to get axis info") {
            if axis == AbsoluteAxisCode::ABS_X {
                xmin = Some(info.minimum());
                xmax = Some(info.maximum());
            }

            if axis == AbsoluteAxisCode::ABS_Y {
                ymin = Some(info.minimum());
                ymax = Some(info.maximum());
            }
        }

        return AppGamepad {
            devnode: devnode,
            name: name,
            evdev_device: dev,
            _leftstick_x_prev: 0.0,
            _leftstick_y_prev: 0.0,
            _leftstick_x_min: xmin,
            _leftstick_x_max: xmax,
            _leftstick_y_min: ymin,
            _leftstick_y_max: ymax,
        };
    }
}

#[derive(Debug, Clone, Copy, Deserialize)]
pub enum SplitscreenType {
    Horizontal,
    Vertical,
}
