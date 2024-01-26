use std::process::Command;

use crate::app::{App, AppResult, Mode, Window};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

pub fn handle_key_events(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
    match (key_event.code, &app.mode, &app.window) {
        (KeyCode::Char('k'), Mode::Normal, Window::Search) => {
            if app.result_scroll > 0 {
                app.result_scroll -= 1;
            }
        }
        (KeyCode::Char('j'), Mode::Normal, Window::Search) => {
            if app.result_scroll < app.result.len() - 1 {
                app.result_scroll += 1;
            }
        }
        (KeyCode::Char('D'), Mode::Normal, Window::Search) => {
            if app.query.len() > 0 {
                app.query.drain(app.cursor_pos..app.query.len());
                app.cursor_pos -= 1;
            }
        }
        (KeyCode::Char('I'), Mode::Normal, Window::Search) => {
            app.cursor_pos = 0;
            app.mode = Mode::Insert
        }
        (KeyCode::Char('A'), Mode::Normal, Window::Search) => {
            app.cursor_pos = app.query.len();
            app.mode = Mode::Insert;
        }
        (KeyCode::Char('c') | KeyCode::Char('C'), _, _)
            if key_event.modifiers == KeyModifiers::CONTROL =>
        {
            app.quit();
        }
        (KeyCode::Backspace, Mode::Insert, Window::Search) => {
            if app.cursor_pos > 0 {
                app.query.remove(app.cursor_pos - 1);
                app.cursor_pos -= 1;
            }
        }
        (KeyCode::Char('x'), Mode::Normal, Window::Search) => {
            if app.cursor_pos <= app.query.len() && app.query.len() > 0 {
                app.query.remove(app.cursor_pos);
            }

            if app.cursor_pos > 0 {
                app.cursor_pos -= 1;
            }
        }
        (KeyCode::Char(c), Mode::Insert, Window::Search) => {
            if app.cursor_pos > app.query.len() {
                app.cursor_pos = app.query.len();
            }
            app.query.insert(app.cursor_pos, c);
            app.cursor_pos += 1;
            app.result_scroll = 0;
        }
        (KeyCode::Char('o') | KeyCode::Char('O'), Mode::Normal, Window::Search) => {
            app.window = Window::Options
        }
        (KeyCode::Char('s') | KeyCode::Char('S'), _, Window::Options) => {
            app.window = Window::Search
        }
        (KeyCode::Char('h'), Mode::Normal, Window::Search) => {
            if app.cursor_pos > 0 {
                app.cursor_pos -= 1;
            }
        }
        (KeyCode::Char('l'), Mode::Normal, Window::Search) => {
            if app.cursor_pos < app.query.len() - 1 {
                app.cursor_pos += 1;
            }
        }
        (KeyCode::Esc, Mode::Insert, Window::Search) => {
            app.mode = Mode::Normal;
            if app.cursor_pos > 0 {
                app.cursor_pos -= 1
            }
        }
        (KeyCode::Char('i'), Mode::Normal, Window::Search) => {
            app.mode = Mode::Insert;
        }
        (KeyCode::Char('a'), Mode::Normal, Window::Search) => {
            app.mode = Mode::Insert;
            if app.cursor_pos < app.query.len() {
                app.cursor_pos += 1;
            }
        }
        (KeyCode::Char('q'), Mode::Normal, _) => app.quit(),
        _ => {}
    }

    if app.query.len() > 0 {
        app.result = String::from_utf8_lossy(
            &Command::new("rg")
                .arg("-l")
                .arg("--sort")
                .arg("modified")
                .arg(app.query.iter().collect::<String>())
                .output()?
                .stdout,
        )
        .lines()
        .map(|line| line.to_string())
        .collect::<Vec<String>>();

        app.preview = String::from_utf8_lossy(
            &Command::new("cat")
                .arg(&app.result.get(app.result_scroll).unwrap_or(&"".to_string()))
                .output()?
                .stdout,
        )
        .to_string();

        return Ok(());
    }

    app.result = Vec::new();
    app.preview = String::new();

    Ok(())
}
