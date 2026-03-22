use serde::Serialize;
use std::fs::File;
use std::io::{BufRead, BufReader};

use crate::utils::capitalize_display;

#[derive(Debug, Clone, Serialize)]
pub struct UserProfile {
    pub user: String,
    pub display_name: String,
}

fn get_display_name(username: &str) -> String {
    if username == "game-user" {
        return String::from("Game User");
    }

    let after = username.split_once("game-user").map(|(_, after)| after);

    return match after {
        Some(after) => capitalize_display(after).to_string(),
        None => capitalize_display(username).to_string(),
    };
}

fn get_all_users() -> std::io::Result<Vec<String>> {
    let file = File::open("/etc/passwd")?;
    let reader = BufReader::new(file);
    let mut users: Vec<String> = vec![];
    for line in reader.lines() {
        let line = line?;
        if let Some(username) = line.split(':').next() {
            users.push(username.to_string());
        }
    }

    Ok(users)
}

pub fn get_all_profiles() -> Vec<UserProfile> {
    let Ok(users) = get_all_users() else {
        return vec![];
    };

    // TODO: Make this more generic in the future - for now it's okay.
    let game_users: Vec<String> = users
        .into_iter()
        .filter(|x| x.starts_with("game-user"))
        .collect();

    let profiles = game_users
        .into_iter()
        .map(|user| UserProfile {
            display_name: get_display_name(&user),
            user: user,
        })
        .collect();

    return profiles;
}
