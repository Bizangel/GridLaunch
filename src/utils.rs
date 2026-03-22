use std::collections::HashMap;
use std::env;
use std::fs;
use std::io;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
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

pub fn ensure_handler_dir_exists() -> io::Result<PathBuf> {
    let home = env::var("HOME")
        .map(PathBuf::from)
        .map_err(|_| io::Error::new(io::ErrorKind::NotFound, "HOME not set"))?;

    let handlers_dir = home.join(".local/share/gridlaunch/handlers");

    fs::create_dir_all(&handlers_dir)?;

    Ok(handlers_dir)
}

pub fn find_handler_json_files() -> io::Result<Vec<PathBuf>> {
    let handlers_dir = ensure_handler_dir_exists()?;

    let mut json_files = Vec::new();

    for entry in fs::read_dir(&handlers_dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.extension().and_then(|e| e.to_str()) == Some("json") {
            json_files.push(path);
        }
    }

    Ok(json_files)
}

pub fn mime_from_extension(path: &str) -> &'static str {
    if path.ends_with(".png") {
        "image/png"
    } else if path.ends_with(".jpg") || path.ends_with(".jpeg") {
        "image/jpeg"
    } else if path.ends_with(".ico") {
        "image/x-icon"
    } else if path.ends_with(".gif") {
        "image/gif"
    } else {
        "application/octet-stream"
    }
}

pub fn capitalize_display(s: &str) -> String {
    s.replace("-", " ")
        .replace("_", " ")
        .split_whitespace()
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
                None => String::new(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}
