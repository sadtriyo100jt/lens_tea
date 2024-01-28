use crate::app::{App, AppResult, Mode, Window};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use std::{env, process::Command};

fn scroll(scroll: &mut usize, direction: i32, limit: usize) {
    let new_scroll = (*scroll as i32) + direction;
    if new_scroll >= 0 && new_scroll < limit as i32 {
        *scroll = new_scroll as usize;
    }
}

fn open_editor(app: &mut App) -> anyhow::Result<()> {
    let editor = env::var("EDITOR")?;
    let result = app
        .result
        .get(app.result_scroll)
        .unwrap()
        .split(":")
        .collect::<Vec<&str>>();

    let (line, column) = (result[1], result[2]);
    let command = match editor.as_ref() {
        "vim" | "nvim" => format!("+normal {}G{}|", line, column),
        "emacs" => format!("+{}:{}", line, column),
        _ => format!(""),
    };
    Command::new(editor)
        .arg(command)
        .arg(
            &app.result
                .get(app.result_scroll)
                .unwrap()
                .split_once(":")
                .unwrap()
                .0,
        )
        .spawn()?
        .wait()?;

    Ok(())
}

pub fn handle_key_events(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
    match (key_event.code, &app.mode, &app.window) {
        (KeyCode::Char('g'), Mode::Normal, Window::Search) => {
            app.result_scroll = 0;
        }
        (KeyCode::Char('G'), Mode::Normal, Window::Search) => {
            app.result_scroll = app.result.len() - 1;
        }
        (KeyCode::Char('e'), Mode::Normal, Window::Search) => {
            open_editor(app)?;
        }
        (KeyCode::Char('j'), Mode::Normal, Window::Options) => {
            scroll(&mut app.options_scroll, 1, 4)
        }
        (KeyCode::Char('k'), Mode::Normal, Window::Options) => {
            scroll(&mut app.options_scroll, -1, 4)
        }
        (KeyCode::Char('k'), Mode::Normal, Window::Search) => {
            scroll(&mut app.result_scroll, -1, app.result.len())
        }
        (KeyCode::Char('j'), Mode::Normal, Window::Search) => {
            scroll(&mut app.result_scroll, 1, app.result.len())
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

            get_results(app)?;
        }
        (KeyCode::Char('x'), Mode::Normal, Window::Search) => {
            if app.cursor_pos <= app.query.len() && app.query.len() > 0 {
                app.query.remove(app.cursor_pos);
            }

            if app.cursor_pos > 0 {
                app.cursor_pos -= 1;
            }
            get_results(app)?;
        }
        (KeyCode::Char(c), Mode::Insert, Window::Search) => {
            if app.cursor_pos > app.query.len() {
                app.cursor_pos = app.query.len();
            }
            app.query.insert(app.cursor_pos, c);
            app.cursor_pos += 1;
            app.result_scroll = 0;
            get_results(app)?;
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
        app.preview = String::from_utf8_lossy(
            &Command::new("rg")
                .arg("-m")
                .arg("1")
                .arg("-C")
                .arg("50")
                .arg("-F")
                .arg(
                    &app.result
                        .get(app.result_scroll)
                        .unwrap_or(&":".to_string())
                        .split(":")
                        .collect::<Vec<&str>>()[3],
                )
                .arg(
                    &app.result
                        .get(app.result_scroll)
                        .unwrap_or(&":".to_string())
                        .split_once(":")
                        .unwrap()
                        .0,
                )
                .output()?
                .stdout,
        )
        .to_string();
        return Ok(());
    }

    app.preview = String::new();

    Ok(())
}

fn get_results(app: &mut App) -> anyhow::Result<()> {
    if app.query.len() > 0 {
        app.result = String::from_utf8_lossy(
            &Command::new("rg")
                .arg("--color=never")
                .arg("--no-heading")
                .arg("--with-filename")
                .arg("--line-number")
                .arg("--column")
                .arg("--smart-case")
                .arg(app.query.iter().collect::<String>())
                .output()?
                .stdout,
        )
        .lines()
        .map(|line| line.to_string())
        .collect::<Vec<String>>();

        return Ok(());
    }

    app.result = Vec::new();
    Ok(())
}
