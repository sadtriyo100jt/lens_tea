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
pub struct Scroll {
    pub result: usize,
    pub options: usize,
}

impl Default for Scroll {
    fn default() -> Self {
        Self {
            result: 0,
            options: 0,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Search {
    pub cursor: usize,
    pub query: Vec<char>,
}

impl Default for Search {
    fn default() -> Self {
        Self {
            cursor: 0,
            query: Vec::new(),
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
    pub options: Vec<String>,
    pub running: bool,
    pub cursor_pos: usize,
    pub searched_line: usize,
    pub preview: String,
    pub vi_command: String,
    pub mode: Mode,
    pub window: Window,
    pub result: Vec<String>,
    pub query: Vec<char>,
    pub scroll: Scroll,
    pub command: Command,
}

impl Default for App {
    fn default() -> Self {
        let options = vec!["hidden files", "test"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        Self {
            options,
            running: true,
            cursor_pos: 0,
            searched_line: 0,
            preview: String::new(),
            vi_command: String::new(),
            mode: Mode::Normal,
            window: Window::Search,
            result: Vec::new(),
            query: Vec::new(),
            scroll: Scroll::default(),
            command: Command::default(),
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
}
