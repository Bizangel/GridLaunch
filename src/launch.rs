use std::io::{BufRead, BufReader};
use std::process::{Command, Stdio};
use std::thread;

use crate::events::LaunchRequestedEvent;

fn launch_game(runas: String, gamepads_to_hide: Vec<String>) {
    unsafe {
        std::env::set_var("SDL_VIDEODRIVER", "x11");
    }

    let pre_bwrap_args = vec![
        "gamescope",
        "-w",
        "1920",
        "-h",
        "1080",
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

pub fn spawn_games(event: LaunchRequestedEvent) {
    let mut handles = Vec::new();

    let users = event.users;
    let gamepads = event.gamepads;

    for (i, user) in users.into_iter().enumerate() {
        let other_gamepads: Vec<String> = gamepads
            .iter()
            .enumerate()
            .filter(|(idx, _)| *idx != i)
            .map(|(_, g)| g.clone())
            .collect();

        handles.push(thread::spawn(move || {
            launch_game(user, other_gamepads);
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }
    println!("All handles returned")
}
