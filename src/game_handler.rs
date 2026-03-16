use base64::{Engine as _, engine::general_purpose};
use serde::{Deserialize, Serialize};
use std::{
    fs,
    path::{Path, PathBuf},
};

use crate::utils::{find_handler_json_files, mime_from_extension};

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct GameHandler {
    // display
    name: String,
    description: String,
    image: Option<PathBuf>,
    image_base_64: Option<String>,
    max_players: u32,
    // executable
    executable_args: Vec<String>,
}

impl GameHandler {
    fn load_and_validate(&mut self) -> Result<(), String> {
        if self.name.is_empty() {
            return Err(format!("Empty game name in game handler"));
        }

        match &self.image {
            Some(imagepath) => {
                if !imagepath.is_file() {
                    return Err(format!("Non existant image path given for game handler"));
                }

                match fs::read(imagepath) {
                    Ok(bytes) => {
                        let icon_b64 = general_purpose::STANDARD.encode(bytes);
                        let icon_mime = mime_from_extension(&imagepath.to_string_lossy());
                        // Set image
                        self.image_base_64 =
                            Some(format!("data:{};base64,{}", icon_mime, icon_b64));
                    }
                    Err(_) => {}
                }
            }
            None => {}
        };

        if self.executable_args.len() == 0 {
            return Err(format!("No executable args for game handler"));
        }

        Ok(())
    }
}

fn _get_handler_entry(handler_path: &Path) -> Result<GameHandler, std::io::Error> {
    let jsonstr = fs::read_to_string(handler_path)?;
    let handler = serde_json::from_str::<GameHandler>(&jsonstr)?;
    return Ok(handler);
}

fn get_handler_entry(handler_path: &Path) -> Option<GameHandler> {
    match _get_handler_entry(handler_path) {
        Ok(entry) => return Some(entry),
        Err(err) => {
            eprintln!(
                "Error reading handler {} {}",
                handler_path.to_string_lossy(),
                err
            );
            return None;
        }
    }
}

fn _get_game_entries() -> Result<Vec<GameHandler>, std::io::Error> {
    let handler_files = find_handler_json_files()?;
    let handlers = handler_files
        .iter()
        .filter_map(|entry| get_handler_entry(&entry))
        .collect();

    Ok(handlers)
}

fn get_game_entries() -> Vec<GameHandler> {
    match _get_game_entries() {
        Ok(entries) => return entries,
        Err(err) => {
            eprintln!("Error getting game entriees {}", err);
            return vec![];
        }
    }
}

pub fn get_valid_game_handlers() -> Vec<GameHandler> {
    return get_game_entries()
        .into_iter()
        .filter_map(|mut handler| match handler.load_and_validate() {
            Ok(()) => Some(handler),
            Err(err) => {
                eprint!("Error: {}", err);
                return None;
            }
        })
        .collect();
}
