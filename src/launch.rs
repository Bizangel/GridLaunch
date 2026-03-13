use crate::events::LaunchRequestedEvent;
use crate::game_instance::GameInstance;
use crate::remapper_thread::RemapperThread;
use std::env;
use std::fs;
use std::path::Path;
use std::process;
use x11rb::connection::Connection;
use x11rb::protocol::randr::ConnectionExt;

#[derive(Debug)]
pub struct Monitor {
    name: String,
    width: u32,
    height: u32,
}

fn x11_get_main_monitor() -> Option<Monitor> {
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

pub fn spawn_games(event: LaunchRequestedEvent) {
    let mut instances: Vec<GameInstance> = Vec::new();
    let mut remapper_threads: Vec<RemapperThread> = Vec::new();

    // let users = event.users;
    let gamepads = event.gamepads;

    let Some(monitor) = x11_get_main_monitor() else {
        eprintln!("Unable to find monitor");
        return;
    };

    // hardcode 2 player horizontal split-screen for now
    let instance_height = monitor.height / 2;
    let instance_width = monitor.width;

    for (i, user) in event.users.iter().enumerate() {
        instances.push(GameInstance::launch(
            &user,
            gamepads
                .iter()
                .enumerate()
                .filter(|(idx, _)| *idx != i)
                .map(|(_, g)| g.as_str()),
            instance_width,
            instance_height,
        ));
    }
    std::thread::sleep(std::time::Duration::from_secs(3));

    for (i, user) in event.users.iter().enumerate() {
        let Some(display) = find_user_game_display(&user) else {
            continue;
        };

        if user != "game-user" {
            continue;
        }

        remapper_threads.push(RemapperThread::new(
            &user,
            &display,
            "/home/game-user/terrariasplitscreenmapping.cfg",
            gamepads
                .iter()
                .enumerate()
                .filter(|(idx, _)| *idx != i)
                .map(|(_, g)| g.as_str()),
        ));
    }

    for instance in instances {
        instance
            .join()
            .expect("Error waiting for game instances to stop");
    }

    for handle in remapper_threads.drain(..) {
        handle.stop().expect("Error stopping remapper threads");
    }

    println!("All handles returned")
}
