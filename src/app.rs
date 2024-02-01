use serde::{Deserialize, Serialize};
use std::{env, error, fs, path::PathBuf};

pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum Mode {
    Normal,
    Insert,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum Window {
    Options,
    Search,
    Command,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Search {
    pub cursor: usize,
    pub query: Vec<char>,
    pub mode: Mode,
    pub result: Vec<String>,
    pub preview: String,
    pub line: usize,
    pub scroll: usize,
}

impl Default for Search {
    fn default() -> Self {
        Self {
            cursor: 0,
            query: Vec::new(),
            mode: Mode::Normal,
            result: Vec::new(),
            preview: String::new(),
            line: 0,
            scroll: 0,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Command {
    pub cursor: usize,
    pub query: Vec<char>,
}

impl Default for Command {
    fn default() -> Self {
        Self {
            cursor: 0,
            query: Vec::new(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct App {
    pub vi_command: String,
    pub running: bool,
    pub window: Window,
    pub command: Command,
    pub search: Search,
    pub args: Vec<String>,
}

impl Default for App {
    fn default() -> Self {
        Self {
            running: true,
            vi_command: String::new(),
            window: Window::Search,
            command: Command::default(),
            search: Search::default(),
            args: Vec::new(),
        }
    }
}

impl App {
    pub fn new() -> anyhow::Result<Self> {
        let home = env::var("HOME")?;
        let config = PathBuf::from(format!("{}/.config/lens", home));
        let session = fs::read_to_string(config.join("session.json"))?;

        Ok(serde_json::from_str(&session)?)
    }

    pub fn tick(&self) {}

    pub fn quit(&mut self) {
        self.running = false;
    }

    pub fn save(&self) -> anyhow::Result<()> {
        let home = env::var("HOME")?;
        let config = PathBuf::from(format!("{}/.config/lens", home));
        let _ = fs::create_dir(&config);
        fs::write(config.join("session.json"), serde_json::to_string(&self)?)?;

        Ok(())
    }

    pub fn delete_session(&self) -> anyhow::Result<()> {
        let home = env::var("HOME")?;
        let config = PathBuf::from(format!("{}/.config/lens", home));
        let _ = fs::remove_file(config.join("session.json"));
        Ok(())
    }
}
