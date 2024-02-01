mod helpers;

use crate::{
    app::{App, AppResult, Mode, Window},
    tui::Tui,
};
use crossterm::event::{KeyCode, KeyEvent};
use helpers::{get_preview, get_results, handle_exit_commands, handle_vi_command, open_editor};
use ratatui::backend::CrosstermBackend;
use std::io;

pub fn handle_key_events(
    key_event: KeyEvent,
    app: &mut App,
    tui: &mut Tui<CrosstermBackend<io::Stderr>>,
) -> AppResult<()> {
    match (key_event.code, &app.search.mode, &app.window) {
        // Search
        (KeyCode::Char(c), Mode::Insert, Window::Search) => {
            if app.search.cursor > app.search.query.len() {
                app.search.cursor = app.search.query.len();
            }
            app.search.query.insert(app.search.cursor, c);
            app.search.cursor += 1;
            app.search.scroll = 0;
            get_results(app)?;
        }
        (KeyCode::Backspace, Mode::Insert, Window::Search) => {
            if app.search.cursor > 0 {
                app.search.query.remove(app.search.cursor - 1);
                app.search.cursor -= 1;
            }

            get_results(app)?;
        }
        (KeyCode::Esc, Mode::Insert, Window::Search) => {
            app.search.mode = Mode::Normal;
            if app.search.cursor > 0 {
                app.search.cursor -= 1
            }
        }
        (KeyCode::Char('G'), Mode::Normal, Window::Search) => {
            app.search.scroll = app.search.result.len() - 1;
        }
        (KeyCode::Enter, _, Window::Search) => {
            if app.search.result.len() > 0 {
                open_editor(app, tui)?;
            }
        }
        (KeyCode::Char('k') | KeyCode::Up, _, Window::Search) => {
            if app.search.scroll == 0 {
                app.search.scroll = app.search.result.len();
            }
            app.search.scroll -= 1;
        }
        (KeyCode::Char('j') | KeyCode::Down, _, Window::Search) => {
            app.search.scroll += 1;
            if app.search.scroll == app.search.result.len() {
                app.search.scroll = 0;
            }
        }
        (KeyCode::Char('D'), Mode::Normal, Window::Search) => {
            if app.search.query.len() > 0 {
                app.search
                    .query
                    .drain(app.search.cursor..app.search.query.len());
                app.search.cursor -= 1;
            }
        }
        (KeyCode::Char('I'), Mode::Normal, Window::Search) => {
            app.search.cursor = 0;
            app.search.mode = Mode::Insert
        }
        (KeyCode::Char('A'), Mode::Normal, Window::Search) => {
            app.search.cursor = app.search.query.len();
            app.search.mode = Mode::Insert;
        }
        (KeyCode::Char('h'), Mode::Normal, Window::Search) => {
            if app.search.cursor > 0 {
                app.search.cursor -= 1;
            }
        }
        (KeyCode::Char('l'), Mode::Normal, Window::Search) => {
            if app.search.cursor < app.search.query.len() - 1 {
                app.search.cursor += 1;
            }
        }
        (KeyCode::Char('i'), Mode::Normal, Window::Search) => {
            app.search.mode = Mode::Insert;
        }
        (KeyCode::Char('a'), Mode::Normal, Window::Search) => {
            app.search.mode = Mode::Insert;
            if app.search.cursor < app.search.query.len() {
                app.search.cursor += 1;
            }
        }
        (KeyCode::Char('x'), Mode::Normal, Window::Search) => {
            if app.search.cursor <= app.search.query.len() && app.search.query.len() > 0 {
                app.search.query.remove(app.search.cursor);
            }

            if app.search.cursor >= app.search.query.len() {
                app.search.cursor -= 1;
            }
            get_results(app)?;
        }

        // Command
        (KeyCode::Esc, _, Window::Command) => {
            app.command.query.clear();
            app.window = Window::Search;
        }
        (KeyCode::Char(c), _, Window::Command) => {
            app.command.query.push(c);
            app.command.cursor += 1;
        }
        (KeyCode::Backspace, _, Window::Command) => {
            if app.command.query.len() <= 1 {
                app.window = Window::Search;
            }

            app.command.query.pop();
            app.command.cursor -= 1;
        }
        (KeyCode::Enter, _, Window::Command) => {
            app.window = Window::Search;
            handle_exit_commands(app)?;
        }

        // General
        (KeyCode::Char(':'), Mode::Normal, _) => {
            app.window = Window::Command;
            app.command.query.clear();
            app.command.query.push(':');
            app.command.cursor += 1;
        }
        (KeyCode::Char(c), _, _) => {
            app.vi_command.push_str(&c.to_string());
            handle_vi_command(app)?;
        }
        (KeyCode::Esc, _, _) => app.vi_command = String::new(),
        _ => {}
    }

    get_preview(app)?;

    Ok(())
}
