use std::path::PathBuf;

use crate::common::AppGamepad;
use evdev::InputEvent;
use evdev::{AbsoluteAxisCode, EventSummary, KeyCode};
use serde::Serialize;
use udev::Device;

pub const GAMEPAD_STICK_THRESHOLD_PRESS: f32 = 0.7;

#[derive(Debug, Clone, Copy, Serialize)]
pub enum AppGamepadButton {
    A,
    B,
    X,
    Y,
    LB,
    RB,
    LS,
    RS,
    Start,
    Select,
    DpadLeft,
    DpadRight,
    DpadUp,
    DpadDown,
    LeftStickRight,
    LeftStickLeft,
    LeftStickUp,
    LeftStickDown,
}

#[derive(Debug, Clone, Serialize)]
pub struct AppGamepadButtonEvent {
    pub button: AppGamepadButton,
    pub release: bool,
    pub gamepad_name: String,
    pub gamepad_devpath: PathBuf,
}

pub fn parse_button_event(
    event: InputEvent,
    gamepad: &mut AppGamepad,
) -> Option<(AppGamepadButton, bool)> {
    match event.destructure() {
        EventSummary::Key(_, code, val) => {
            let button = match code {
                KeyCode::BTN_SOUTH => AppGamepadButton::A,
                KeyCode::BTN_EAST => AppGamepadButton::B,
                KeyCode::BTN_NORTH => AppGamepadButton::X,
                KeyCode::BTN_WEST => AppGamepadButton::Y,
                KeyCode::BTN_START => AppGamepadButton::Start,
                KeyCode::BTN_SELECT => AppGamepadButton::Select,
                KeyCode::BTN_TL => AppGamepadButton::LB,
                KeyCode::BTN_TR => AppGamepadButton::RB,
                KeyCode::BTN_THUMBL => AppGamepadButton::LS,
                KeyCode::BTN_THUMBR => AppGamepadButton::RS,
                _ => return None,
            };

            Some((button, val == 0))
        }

        EventSummary::AbsoluteAxis(_, AbsoluteAxisCode::ABS_HAT0X, val) => match val {
            -1 => Some((AppGamepadButton::DpadLeft, false)),
            1 => Some((AppGamepadButton::DpadRight, false)),
            _ => None,
        },

        EventSummary::AbsoluteAxis(_, AbsoluteAxisCode::ABS_HAT0Y, val) => match val {
            -1 => Some((AppGamepadButton::DpadUp, false)),
            1 => Some((AppGamepadButton::DpadDown, false)),
            _ => None,
        },

        // Left stick X axis
        EventSummary::AbsoluteAxis(_, AbsoluteAxisCode::ABS_X, val) => {
            let Some(max) = gamepad._leftstick_x_max else {
                return None;
            };
            let Some(min) = gamepad._leftstick_x_min else {
                return None;
            };

            let prev = gamepad._leftstick_x_prev;
            let curr = normalize(val, min, max);

            // update previous
            gamepad._leftstick_x_prev = curr;
            return find_stick_threshold_release(
                curr,
                prev,
                (
                    AppGamepadButton::LeftStickLeft,
                    AppGamepadButton::LeftStickRight,
                ),
            );
        }

        // Left stick Y axis
        EventSummary::AbsoluteAxis(_, AbsoluteAxisCode::ABS_Y, val) => {
            let Some(max) = gamepad._leftstick_y_max else {
                return None;
            };
            let Some(min) = gamepad._leftstick_y_min else {
                return None;
            };

            let prev = gamepad._leftstick_y_prev;
            let curr = normalize(val, min, max);

            // update previous
            gamepad._leftstick_y_prev = curr;
            return find_stick_threshold_release(
                curr,
                prev,
                (
                    AppGamepadButton::LeftStickUp,
                    AppGamepadButton::LeftStickDown,
                ),
            );
        }

        _ => None,
    }
}

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
pub fn get_device_name(mut event: udev::Device) -> Option<String> {
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

pub fn get_device_name_with_unk_default(dev: &Device) -> String {
    get_device_name(dev.clone()).unwrap_or_else(|| "Unknown".into())
}

// expects val to be [0-255]
fn normalize(val: i32, min: i32, max: i32) -> f32 {
    let center = (min + max) / 2;
    let half_range = (max - min) as f32 / 2.0;

    let normalized = (val - center) as f32 / half_range;
    return normalized.clamp(-1.0, 1.0);
}

fn find_stick_threshold_release<T>(curr: f32, prev: f32, options: (T, T)) -> Option<(T, bool)> {
    if curr <= -GAMEPAD_STICK_THRESHOLD_PRESS && -GAMEPAD_STICK_THRESHOLD_PRESS < prev {
        return Some((options.0, false));
    }

    if prev <= -GAMEPAD_STICK_THRESHOLD_PRESS && -GAMEPAD_STICK_THRESHOLD_PRESS < curr {
        return Some((options.0, true));
    }

    if curr >= GAMEPAD_STICK_THRESHOLD_PRESS && GAMEPAD_STICK_THRESHOLD_PRESS > prev {
        return Some((options.1, false));
    }

    if prev >= GAMEPAD_STICK_THRESHOLD_PRESS && GAMEPAD_STICK_THRESHOLD_PRESS > curr {
        return Some((options.1, true));
    }

    return None;
}
