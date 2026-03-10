use crate::events::LaunchRequestedEvent;
use std::env;
use std::fs;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::process::{Command, Stdio};
use std::thread;
use x11rb::connection::Connection;
use x11rb::protocol::randr::ConnectionExt;

#[derive(Debug)]
pub struct Monitor {
    name: String,
    width: u32,
    height: u32,
}

fn launch_game(runas: String, gamepads_to_hide: Vec<String>, width: u32, height: u32) {
    unsafe {
        std::env::set_var("SDL_VIDEODRIVER", "x11");
    }

    let instance_width = width.to_string();
    let instance_height = height.to_string();
    let pre_bwrap_args = vec![
        "gamescope",
        "-w",
        &instance_width,
        "-h",
        &instance_height,
        "--",
        "bwrap",
        "--die-with-parent",
        "--dev-bind",
        "/",
        "/",
    ];

    let bwrap_hide_args: Vec<_> = gamepads_to_hide
        .iter()
        .flat_map(|x| ["--bind", "/dev/null", &x])
        .collect();

    let actual_args = vec!["steam", "-console", "steam://open/bigpicture"];

    let mut child = Command::new("/home/arcanzu/scripts/gaming/run_as_user_gaming.sh")
        .arg(runas)
        .args(pre_bwrap_args)
        .args(bwrap_hide_args)
        .args(actual_args)
        .env("ENABLE_GAMESCOPE_WSI", "0")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("failed to execute process");

    let stdout = BufReader::new(child.stdout.take().unwrap());
    let stderr = BufReader::new(child.stderr.take().unwrap());

    let out_handle = thread::spawn(move || {
        for line in stdout.lines() {
            println!("{}", line.unwrap());
        }
    });
    let err_handle = thread::spawn(move || {
        for line in stderr.lines() {
            eprintln!("{}", line.unwrap());
        }
    });

    child.wait().unwrap();
    out_handle.join().unwrap();
    err_handle.join().unwrap();
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

        let output = Command::new("/home/arcanzu/scripts/gaming/run_as_user_gaming.sh")
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

pub fn launch_gamepad2key() {
    // todo launch remapper for program
}

pub fn spawn_games(event: LaunchRequestedEvent) {
    let mut handles = Vec::new();

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
        let other_gamepads: Vec<String> = gamepads
            .iter()
            .enumerate()
            .filter(|(idx, _)| *idx != i)
            .map(|(_, g)| g.clone())
            .collect();

        let thread_user = user.clone();
        handles.push(thread::spawn(move || {
            launch_game(thread_user, other_gamepads, instance_width, instance_height);
        }));
    }
    std::thread::sleep(std::time::Duration::from_millis(100));

    for user in event.users.iter() {
        let display = find_user_game_display(&user);
        println!("Found display {:#?} for user {}", display, user);
        // handles.push(thread::spawn(move || {
        //     launch_game(thread_user, other_gamepads, instance_width, instance_height);
        // }));
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("All handles returned")
}
