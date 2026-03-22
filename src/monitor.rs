use std::env;
use std::fs;
use std::path::Path;
use std::process;
use std::process::Command;
use x11rb::connection::Connection;
use x11rb::protocol::randr::ConnectionExt;

#[derive(Debug)]
pub struct Monitor {
    pub name: String,
    pub width: u32,
    pub height: u32,
}

pub fn get_main_monitor_xdotool() -> Option<Monitor> {
    let output = Command::new("xdotool")
        .args(["getdisplaygeometry"])
        .output()
        .ok()?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut parts = stdout.trim().split_whitespace();

    let width = parts.next()?.parse::<u32>().ok()?;
    let height = parts.next()?.parse::<u32>().ok()?;

    Some(Monitor {
        name: String::from("Display"),
        width,
        height,
    })
}

pub fn x11_get_main_monitor() -> Option<Monitor> {
    let (conn, display_idx) = x11rb::connect(None).ok()?;
    let display = &conn.setup().roots[display_idx];

    let primary_monitor_idx = conn
        .randr_get_output_primary(display.root)
        .ok()?
        .reply()
        .ok()?
        .output;

    let monitor_info = conn
        .randr_get_screen_resources(display.root)
        .ok()?
        .reply()
        .ok()?;

    let output_info = conn
        .randr_get_output_info(primary_monitor_idx, monitor_info.config_timestamp)
        .ok()?
        .reply()
        .ok()?;

    // Validate the output
    if output_info.connection != x11rb::protocol::randr::Connection::CONNECTED
        || output_info.crtc == 0
    {
        return None;
    }

    let crtc = conn
        .randr_get_crtc_info(output_info.crtc, monitor_info.config_timestamp)
        .ok()?
        .reply()
        .ok()?;

    Some(Monitor {
        name: String::from_utf8_lossy(&output_info.name).into_owned(),
        width: crtc.width.into(),
        height: crtc.height.into(),
    })
}

pub fn find_user_game_display(user: &str) -> Option<String> {
    // Current active display to exclude
    let current_display = env::var("DISPLAY").ok();

    for display in list_x11_displays() {
        if Some(&display) == current_display.as_ref() {
            continue;
        }

        let output = process::Command::new("/home/arcanzu/scripts/gaming/run_as_user_gaming.sh")
            .arg(user)
            .args(["env", &format!("DISPLAY={}", display), "xprop", "-root"])
            .output();

        if let Ok(out) = output {
            if out.status.success() {
                return Some(display.clone());
            }
        }
    }

    None
}

pub fn list_x11_displays() -> Vec<String> {
    let socket_dir = Path::new("/tmp/.X11-unix");
    let mut displays = Vec::new();

    if let Ok(entries) = fs::read_dir(socket_dir) {
        for entry in entries.flatten() {
            let name = entry.file_name();
            let name = name.to_string_lossy();

            if name.starts_with('X') {
                if let Ok(num) = name[1..].parse::<u32>() {
                    displays.push(format!(":{}", num));
                }
            }
        }
    }

    displays.sort();
    displays
}
