use nix::sys::signal::{self, Signal};
use nix::unistd::Pid;
use std::{process, thread::JoinHandle};

use crate::common::RUNAS_SCRIPT_PATH;
use crate::utils::spawn_process_with_thread_readers;

pub struct RemapperThread {
    user: String,
    process: process::Child,
    stdout_thread: JoinHandle<()>,
    stderr_thread: JoinHandle<()>,
}

impl RemapperThread {
    pub fn new(user: &str, display: &str, remap_config_path: &str) -> RemapperThread {
        let args = vec![
            &user,
            "gamepad2key",
            "--display",
            display,
            remap_config_path,
        ];

        let (child, stdout, stderr) = spawn_process_with_thread_readers(
            RUNAS_SCRIPT_PATH,
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
