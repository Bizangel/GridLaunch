use crate::{common::RUNAS_SCRIPT_PATH, utils::spawn_process_with_thread_readers_with_env};
use std::{collections::HashMap, process, thread::JoinHandle};

pub struct GameInstance {
    user: String,
    process: process::Child,
    stdout_thread: JoinHandle<()>,
    stderr_thread: JoinHandle<()>,
}

impl GameInstance {
    pub fn launch<'a>(
        runas: &str,
        exec_args: Vec<&str>,
        gamepads_to_hide: impl IntoIterator<Item = &'a str>,
        width: u32,
        height: u32,
    ) -> GameInstance {
        unsafe {
            std::env::set_var("SDL_VIDEODRIVER", "x11");
        }

        let instance_width = width.to_string();
        let instance_height = height.to_string();
        let gamescope_args = vec![
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
        let bwrap_hide_args: Vec<&str> = gamepads_to_hide
            .into_iter()
            .flat_map(|x| ["--bind", "/dev/null", &x])
            .collect();

        // example for steam
        // let game_args = vec!["steam", "-console", "steam://open/bigpicture"];

        let full_args: Vec<&str> = std::iter::empty()
            .chain(std::iter::once(runas))
            .chain(gamescope_args.into_iter())
            .chain(bwrap_hide_args.into_iter())
            .chain(exec_args.into_iter())
            .collect();

        let map = HashMap::from([("SDL_VIDEODRIVER", "x11"), ("ENABLE_GAMESCOPE_WSI", "0")]);
        let (child, stdout_handle, stderr_handle) = spawn_process_with_thread_readers_with_env(
            RUNAS_SCRIPT_PATH,
            &full_args,
            &format!("GAME {}", runas),
            &map,
        );

        return GameInstance {
            user: runas.to_string(),
            process: child,
            stdout_thread: stdout_handle,
            stderr_thread: stderr_handle,
        };
    }

    pub fn join(mut self) -> Result<(), ()> {
        self.process.wait().map_err(|_| ())?;
        println!("{} game instance exited", &self.user);
        self.stdout_thread.join().map_err(|_| ())?;
        self.stderr_thread.join().map_err(|_| ())?;
        Ok(())
    }
}
