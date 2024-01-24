use crate::app::{App, AppResult, Mode, Window};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

/// Handles the key events and updates the state of [`App`].
pub fn handle_key_events(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
    match (key_event.code, &app.mode, &app.window) {
        (KeyCode::Char('c') | KeyCode::Char('C'), _, _)
            if key_event.modifiers == KeyModifiers::CONTROL =>
        {
            app.quit()
        }
        (KeyCode::Backspace, Mode::Insert, Window::Search) => {
            app.query.pop();
        }
        (KeyCode::Char(c), Mode::Insert, Window::Search) => {
            app.query += &c.to_string();
        }
        (KeyCode::Char('o') | KeyCode::Char('O'), Mode::Normal, Window::Search) => {
            app.window = Window::Options
        }
        (KeyCode::Char('s') | KeyCode::Char('S'), _, Window::Options) => {
            app.window = Window::Search
        }
        (KeyCode::Esc, Mode::Insert, Window::Search) => app.mode = Mode::Normal,
        (KeyCode::Char('i'), Mode::Normal, Window::Search) => app.mode = Mode::Insert,
        (KeyCode::Char('q'), Mode::Normal, _) => app.quit(),
        _ => {}
    }
    Ok(())
}
