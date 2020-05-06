use dirs;
use std::{path::PathBuf};

struct Config {
    name: String, // "Open in <name>"
    terminal: PathBuf,
    args: Vec<String>
}

impl Config {

    pub fn load() -> Result<Self, ()> {

        todo!()

    }

}
