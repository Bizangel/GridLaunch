use crate::events::fromwebview_event::LaunchRequestedEvent;
use crate::game_instance::GameInstance;
use crate::kwin_window_handling::load_kwin_script_dbus;
use crate::kwin_window_handling::unload_kwin_script_dbus;
use crate::monitor::find_user_game_display;
use crate::monitor::x11_get_main_monitor;
use crate::remapper_thread::RemapperThread;
use std::path::PathBuf;

pub fn spawn_games_and_wait(event: LaunchRequestedEvent) {
    println!("{:#?}", event);
    return;

    let mut instances: Vec<GameInstance> = Vec::new();
    let mut remapper_threads: Vec<RemapperThread> = Vec::new();

    // let users = event.users;
    let gamepads = event.gamepads;

    let Some(monitor) = x11_get_main_monitor() else {
        eprintln!("Unable to find monitor");
        return;
    };

    load_kwin_script_dbus(PathBuf::from(
        "/home/arcanzu/workplace/gridlaunch/src/assets/kwin_splitscreen.js",
    ))
    .expect("Unable to load kwin script");

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
