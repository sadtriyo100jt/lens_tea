use std::error;

pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

#[derive(Debug, PartialEq)]
pub enum Mode {
    Normal,
    Insert,
}

#[derive(Debug, PartialEq)]
pub enum Window {
    Options,
    Search,
}

#[derive(Debug)]
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

#[derive(Debug)]
pub struct App {
    pub running: bool,
    pub window: Window,
    pub mode: Mode,
    pub scroll: Scroll,
    pub query: Vec<char>,
    pub cursor_pos: usize,
    pub result: Vec<String>,
    pub args: Vec<String>,
    pub preview: String,
    pub searched_line: usize,
}

impl Default for App {
    fn default() -> Self {
        Self {
            running: true,
            mode: Mode::Normal,
            window: Window::Search,
            query: Vec::new(),
            cursor_pos: 0,
            result: Vec::new(),
            preview: String::new(),
            args: Vec::new(),
            searched_line: 0,
            scroll: Scroll::default(),
        }
    }
}

impl App {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn tick(&self) {}

    pub fn quit(&mut self) {
        self.running = false;
    }
}
