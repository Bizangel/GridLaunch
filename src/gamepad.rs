use evdev::InputEvent;
use evdev::{AbsoluteAxisCode, EventSummary, KeyCode};
use serde::Serialize;
use udev::Device;

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
}

#[derive(Debug, Clone, Copy, Serialize)]
pub struct AppGamepadButtonEvent {
    pub button: AppGamepadButton,
    pub release: bool,
}

pub fn parse_button_event(event: InputEvent) -> Option<AppGamepadButtonEvent> {
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

            Some(AppGamepadButtonEvent {
                button,
                release: val == 0,
            })
        }

        EventSummary::AbsoluteAxis(_, AbsoluteAxisCode::ABS_HAT0X, val) => match val {
            -1 => Some(AppGamepadButtonEvent {
                button: AppGamepadButton::DpadLeft,
                release: false,
            }),
            1 => Some(AppGamepadButtonEvent {
                button: AppGamepadButton::DpadRight,
                release: false,
            }),
            _ => None,
        },

        EventSummary::AbsoluteAxis(_, AbsoluteAxisCode::ABS_HAT0Y, val) => match val {
            -1 => Some(AppGamepadButtonEvent {
                button: AppGamepadButton::DpadUp,
                release: false,
            }),
            1 => Some(AppGamepadButtonEvent {
                button: AppGamepadButton::DpadDown,
                release: false,
            }),
            _ => None,
        },

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
