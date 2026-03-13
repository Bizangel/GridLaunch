use std::collections::HashMap;
use std::io::{BufRead, BufReader};
use std::thread;
use std::{process, thread::JoinHandle};

pub fn spawn_process_with_thread_readers(
    executable: &str,
    args: &[&str],
    log_prefix: &str,
) -> (process::Child, JoinHandle<()>, JoinHandle<()>) {
    return spawn_process_with_thread_readers_with_env(
        executable,
        args,
        log_prefix,
        &HashMap::new(),
    );
}

pub fn spawn_process_with_thread_readers_with_env(
    executable: &str,
    args: &[&str],
    log_prefix: &str,
    env_vars: &HashMap<&str, &str>,
) -> (process::Child, JoinHandle<()>, JoinHandle<()>) {
    println!("Executing: {} {}", executable, args.join(" "));
    let mut child = process::Command::new(executable)
        .args(args)
        .stdout(process::Stdio::piped())
        .stderr(process::Stdio::piped())
        .envs(env_vars)
        .spawn()
        .expect("failed to execute process");

    let stdout = BufReader::new(child.stdout.take().unwrap());
    let stderr = BufReader::new(child.stderr.take().unwrap());

    let stdout_log_prefix = log_prefix.to_string();
    let stdout_handle = thread::spawn(move || {
        for line in stdout.lines() {
            println!("[{}]{}", stdout_log_prefix, line.unwrap());
        }
    });

    let stderr_log_prefix = log_prefix.to_string();
    let stderr_handle = thread::spawn(move || {
        for line in stderr.lines() {
            eprintln!("[{}]{}", stderr_log_prefix, line.unwrap());
        }
    });

    return (child, stdout_handle, stderr_handle);
}
