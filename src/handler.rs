mod helpers;

use crate::{
    app::{App, AppResult, Mode, Window},
    tui::Tui,
};
use crossterm::event::{KeyCode, KeyEvent};
use helpers::{check_commands, get_preview, get_results, open_editor};
use ratatui::backend::CrosstermBackend;
use std::io;

pub fn handle_key_events(
    key_event: KeyEvent,
    app: &mut App,
    tui: &mut Tui<CrosstermBackend<io::Stderr>>,
) -> AppResult<()> {
    match (key_event.code, &app.mode, &app.window) {
        // Search
        (KeyCode::Char(c), Mode::Insert, Window::Search) => {
            if app.cursor_pos > app.query.len() {
                app.cursor_pos = app.query.len();
            }
            app.query.insert(app.cursor_pos, c);
            app.cursor_pos += 1;
            app.scroll.result = 0;
            get_results(app)?;
        }
        (KeyCode::Backspace, Mode::Insert, Window::Search) => {
            if app.cursor_pos > 0 {
                app.query.remove(app.cursor_pos - 1);
                app.cursor_pos -= 1;
            }

            get_results(app)?;
        }
        (KeyCode::Esc, Mode::Insert, Window::Search) => {
            app.mode = Mode::Normal;
            if app.cursor_pos > 0 {
                app.cursor_pos -= 1
            }
        }
        (KeyCode::Char('G'), Mode::Normal, Window::Search) => {
            app.scroll.result = app.result.len() - 1;
        }
        (KeyCode::Enter, _, Window::Search) => {
            if app.result.len() > 0 {
                open_editor(app, tui)?;
            }
        }
        (KeyCode::Char('k') | KeyCode::Up, _, Window::Search) => {
            if app.scroll.result == 0 {
                app.scroll.result = app.result.len();
            }
            app.scroll.result -= 1;
        }
        (KeyCode::Char('j') | KeyCode::Down, _, Window::Search) => {
            app.scroll.result += 1;
            if app.scroll.result == app.result.len() {
                app.scroll.result = 0;
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
        (KeyCode::Char('i'), Mode::Normal, Window::Search) => {
            app.mode = Mode::Insert;
        }
        (KeyCode::Char('a'), Mode::Normal, Window::Search) => {
            app.mode = Mode::Insert;
            if app.cursor_pos < app.query.len() {
                app.cursor_pos += 1;
            }
        }
        (KeyCode::Char('x'), Mode::Normal, Window::Search) => {
            if app.cursor_pos <= app.query.len() && app.query.len() > 0 {
                app.query.remove(app.cursor_pos);
            }

            if app.cursor_pos >= app.query.len() {
                app.cursor_pos -= 1;
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
            if app.command.query.len() > 1 {
                app.command.query.pop();
                app.command.cursor -= 1;
            }
        }
        (KeyCode::Enter, _, Window::Command) => {
            app.window = Window::Search;
            match app.command.query.iter().collect::<String>().as_ref() {
                ":q" => {
                    app.command.query.clear();
                    app.quit();
                }
                ":w" => {
                    app.command.query.clear();
                    app.save()?;
                }
                ":wq" => {
                    app.command.query.clear();
                    app.save()?;
                    app.quit()
                }
                _ => {}
            }
        }

        // Options
        (KeyCode::Char('j') | KeyCode::Down, Mode::Normal, Window::Options) => {
            app.scroll.options += 1;
            if app.scroll.options == app.options.len() {
                app.scroll.options = 0
            }
        }
        (KeyCode::Char('k') | KeyCode::Up, Mode::Normal, Window::Options) => {
            if app.scroll.options == 0 {
                app.scroll.options = app.options.len();
            }
            app.scroll.options -= 1;
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
            check_commands(app)?;
        }
        (KeyCode::Esc, _, _) => app.vi_command = String::new(),
        _ => {}
    }

    get_preview(app)?;

    Ok(())
}
