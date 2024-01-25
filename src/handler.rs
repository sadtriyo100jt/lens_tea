use std::u16;

use crate::app::{App, AppResult, Mode, Window};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

pub fn handle_key_events(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
    match (key_event.code, &app.mode, &app.window) {
        (KeyCode::Char('I'), Mode::Normal, Window::Search) => {
            app.cursor_pos = 0;
            app.mode = Mode::Insert
        }
        (KeyCode::Char('A'), Mode::Normal, Window::Search) => {
            app.cursor_pos = app.query.len() as u16;
            app.mode = Mode::Insert;
        }
        (KeyCode::Char('c') | KeyCode::Char('C'), _, _)
            if key_event.modifiers == KeyModifiers::CONTROL =>
        {
            app.quit();
        }
        (KeyCode::Backspace, Mode::Insert, Window::Search) => {
            if app.cursor_pos > 0 {
                app.query.remove(app.cursor_pos as usize - 1);
                app.cursor_pos -= 1;
            }
        }
        (KeyCode::Char('x'), Mode::Normal, Window::Search) => {
            if app.cursor_pos <= app.query.len() as u16 && app.query.len() > 0 {
                app.query.remove(app.cursor_pos as usize);
                app.cursor_pos -= 1;
            }
        }
        (KeyCode::Char(c), Mode::Insert, Window::Search) => {
            if app.cursor_pos > app.query.len() as u16 {
                app.cursor_pos = app.query.len() as u16;
            }
            app.query.insert(app.cursor_pos as usize, c);
            app.cursor_pos += 1;
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
            if app.cursor_pos < app.query.len() as u16 - 1 {
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
            if app.cursor_pos < app.query.len() as u16 {
                app.cursor_pos += 1;
            }
        }
        (KeyCode::Char('q'), Mode::Normal, _) => app.quit(),
        _ => {}
    }
    Ok(())
}
