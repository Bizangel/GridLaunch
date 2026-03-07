use evdev::InputEvent;
use evdev::{AbsoluteAxisCode, EventSummary, KeyCode};
use serde::Serialize;

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

#[derive(Debug, Clone, Copy)]
pub struct AppGamepadButtonEvent {
    button: AppGamepadButton,
    release: bool,
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
