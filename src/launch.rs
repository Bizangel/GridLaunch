use crate::common::SplitscreenType;
use crate::events::fromwebview_event::LaunchRequestedEvent;
use crate::game_handler::GameHandler;
use crate::game_instance::GameInstance;
use crate::kwin_window_handling::load_kwin_script_dbus;
use crate::kwin_window_handling::unload_kwin_script_dbus;
use crate::monitor::find_user_game_display;
use crate::monitor::x11_get_main_monitor;
use crate::remapper_thread::RemapperThread;
use std::env;
use std::io;
use std::path::PathBuf;

pub fn find_kwin_script_path() -> Result<PathBuf, io::Error> {
    let binary_path = env::current_exe()?;
    let splitscreen_path = binary_path
        .parent()
        .map(|x| x.join("assets").join("kwin_splitscreen.js"));

    let debug_path = binary_path
        .parent()
        .and_then(|x| x.parent())
        .and_then(|x| x.parent())
        // .and_then(|x| x.parent())
        .map(|x| x.join("src"))
        .map(|x| x.join("assets"))
        .map(|x| x.join("kwin_splitscreen.js"));

    let possible_paths: Vec<Option<PathBuf>> = [splitscreen_path, debug_path].to_vec();

    for pos_path in possible_paths.iter() {
        let Some(path) = pos_path else {
            continue;
        };

        if path.is_file() {
            return path.canonicalize();
        }
    }

    Err(io::Error::new(
        io::ErrorKind::NotFound,
        format!("Unable find KWin script path at {:#?}", possible_paths),
    ))
}

pub fn calc_instance_size(
    player_index: u32,
    split_type: SplitscreenType,
    total_players: u32,
    monitor_width: u32,
    monitor_height: u32,
) -> (u32, u32) {
    match split_type {
        SplitscreenType::Horizontal => {
            if total_players == 2 {
                return (monitor_width, monitor_height / 2);
            }
            if total_players == 3 {
                if player_index == 0 {
                    // P1 gets full bar at top
                    return (monitor_width, monitor_height / 2);
                }
                // quarter for rest
                return (monitor_width / 2, monitor_height / 2);
            }
            if total_players == 4 {
                // quarter
                return (monitor_width / 2, monitor_height / 2);
            }
            panic!("Unhandled more than 4 players");
        }
        SplitscreenType::Vertical => {
            // if total_players == 2 {
            //     return (monitor_width / 2, monitor_height);
            // }
            // TODO: FIX
            return (100, 100);
        }
    }
}

pub fn spawn_games_and_wait(event: LaunchRequestedEvent, game_handler: GameHandler) {
    let mut instances: Vec<GameInstance> = Vec::new();
    let mut remapper_threads: Vec<RemapperThread> = Vec::new();

    let gamepads = event.gamepads;

    let kwin_script_path = match find_kwin_script_path() {
        Ok(path) => path,
        Err(err) => {
            eprintln!("{}", err);
            return;
        }
    };

    let Some(monitor) = x11_get_main_monitor() else {
        eprintln!("Unable to find monitor");
        return;
    };

    load_kwin_script_dbus(kwin_script_path).expect("Unable to load kwin script");

    for (i, user) in event.users.iter().enumerate() {
        let (instance_width, instance_height) = calc_instance_size(
            i as u32,
            event.splitscreen_type,
            event.users.len() as u32,
            monitor.width,
            monitor.height,
        );

        instances.push(GameInstance::launch(
            &user,
            game_handler
                .executable_args
                .iter()
                .map(|s| s.as_str())
                .collect(),
            gamepads
                .iter()
                .enumerate()
                .filter(|(idx, _)| *idx != i)
                .map(|(_, g)| g.as_str()),
            instance_width,
            instance_height,
        ));

        // gives time for windows to settle etc.
        std::thread::sleep(std::time::Duration::from_millis(100));
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

    unload_kwin_script_dbus().expect("Unable to unload kwin script");

    println!("All handles returned")
}
