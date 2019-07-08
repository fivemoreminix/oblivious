//! This entire source file is responsible for holding any major game data,
//! such as, but not limited to, the player's name, race, wealth, items;
//! the world's living and dead entities, items, and explored and unexplored
//! places.
//! 
//! The GameData struct likes to read and write data about the active game,
//! using a PathBuf to a save's root directory, where all of the individual
//! data configuration files are saved.

use std::path::{Path, PathBuf};

pub struct GameData {
    folder_path: PathBuf
}

impl GameData {
    /// Create a new game save.
    pub fn new(save_dir_path: &Path) -> std::io::Result<GameData> {
        std::fs::create_dir(save_dir_path)?;
        let mut data = GameData { folder_path: save_dir_path.to_owned() };
        // write data to disk
        Ok(data)
    }

    /// Load an existing game save.
    pub fn load(save_dir_path: &Path) -> std::io::Result<GameData> {
        unimplemented!()
    }
}
