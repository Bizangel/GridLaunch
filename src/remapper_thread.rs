use nix::sys::signal::{self, Signal};
use nix::unistd::Pid;
use std::path::Path;
use std::{process, thread::JoinHandle};

use crate::utils::spawn_process_with_thread_readers;

pub struct RemapperThread {
    user: String,
    process: process::Child,
    stdout_thread: JoinHandle<()>,
    stderr_thread: JoinHandle<()>,
}

impl RemapperThread {
    pub fn new<'a>(
        run_as_script: &Path,
        user: &str,
        display: &str,
        remap_config_path: &str,
        gamepads_to_hide: impl IntoIterator<Item = &'a str>,
    ) -> RemapperThread {
        let bwrap_args = vec!["bwrap", "--die-with-parent", "--dev-bind", "/", "/"];
        let gamepad2keyargs = vec!["gamepad2key", "--display", display, remap_config_path];
        let bwrap_hide_args: Vec<&str> = gamepads_to_hide
            .into_iter()
            .flat_map(|x| ["--bind", "/dev/null", &x])
            .collect();

        let args: Vec<&str> = std::iter::once(user)
            .chain(bwrap_args.into_iter())
            .chain(bwrap_hide_args.into_iter())
            .chain(gamepad2keyargs.into_iter())
            .collect();

        let (child, stdout, stderr) = spawn_process_with_thread_readers(
            &run_as_script.to_string_lossy(),
            &args,
            &format!("GAMEPAD2KEY {}", user),
        );

        return RemapperThread {
            user: user.to_string(),
            process: child,
            stdout_thread: stdout,
            stderr_thread: stderr,
        };
    }

    pub fn stop(mut self) -> Result<(), ()> {
        let pid = Pid::from_raw(self.process.id() as i32);
        println!(
            "Sending interrupt to gamepad2key {} to PID {}",
            self.user, pid
        );
        signal::kill(pid, Signal::SIGINT).map_err(|_| ())?;
        self.process.wait().map_err(|_| ())?;
        println!("Gamepad2key ended for {}", self.user);
        self.stdout_thread.join().map_err(|_| ())?;
        self.stderr_thread.join().map_err(|_| ())?;

        Ok(())
    }
}
