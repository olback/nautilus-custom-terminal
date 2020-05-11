use dirs;
use serde::{Serialize, Deserialize};
use std::{
    path::PathBuf,
    fs
};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub name: String, // "Open in <name>"
    pub terminal: PathBuf,
    pub args: Vec<String>
}

impl Config {

    pub fn load() -> Self {

        fs::read_to_string(Self::path()).map(|s| toml::from_str(&s).unwrap()).unwrap_or_else(|_| {
            Self::default().save();
            Self::default()
        })

    }

    pub fn save(&self) {

        let dir = Self::path().parent().map(|p| p.to_path_buf()).unwrap();

        if !dir.exists() {
            fs::create_dir_all(dir).unwrap();
        }

        fs::write(Self::path(), toml::to_string_pretty(&self).unwrap()).unwrap();

    }

    fn path() -> PathBuf {

        dirs::config_dir()
        .unwrap()
        .join("nautilus")
        .join("terminal.toml")

    }

}

impl Default for Config {

    fn default() -> Self {

        Self {
            name: "Terminal".into(),
            terminal: "gnome-terminal".into(),
            args: vec!["{DIR}".into()]
        }

    }

}