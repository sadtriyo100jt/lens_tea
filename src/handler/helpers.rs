use ratatui::backend::CrosstermBackend;

use crate::app::{App, Window};
use crate::tui::Tui;
use std::{env, process::Command};
use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
};

pub fn open_editor(
    app: &mut App,
    tui: &mut Tui<CrosstermBackend<io::Stderr>>,
) -> anyhow::Result<()> {
    let editor = match env::var("EDITOR") {
        Ok(editor) => editor,
        Err(_) => return Ok(()),
    };
    let result = app
        .search
        .result
        .get(app.search.scroll)
        .unwrap()
        .split(":")
        .collect::<Vec<&str>>();

    let (line, column) = (result[1], result[2]);
    let command = match editor.as_ref() {
        "vim" | "nvim" => format!("+normal {}G{}|", line, column),
        "emacs" => format!("+{}:{}", line, column),
        _ => format!(""),
    };

    let _ = tui.pause();
    Command::new(editor)
        .arg(command)
        .arg(result[0])
        .spawn()?
        .wait()?;
    let _ = tui.resume();

    Ok(())
}

pub fn get_results(app: &mut App) -> anyhow::Result<()> {
    if app.search.query.len() > 0 {
        app.search.result = String::from_utf8_lossy(
            &Command::new("rg")
                .args(&app.args)
                .arg("--color=never")
                .arg("--no-heading")
                .arg("--with-filename")
                .arg("--line-number")
                .arg("--column")
                .arg("--smart-case")
                .arg(app.search.query.iter().collect::<String>())
                .output()?
                .stdout,
        )
        .lines()
        .map(|line| line.to_string())
        .collect::<Vec<String>>();

        return Ok(());
    }

    app.search.result = Vec::new();
    Ok(())
}

pub fn get_preview(app: &mut App) -> anyhow::Result<()> {
    if app.search.result.len() == 0 {
        app.search.preview = String::new();
        return Ok(());
    }
    let result = app
        .search
        .result
        .get(app.search.scroll)
        .unwrap()
        .split(":")
        .collect::<Vec<&str>>();
    let file = File::open(Path::new(result[0]));

    if let Ok(file) = file {
        let reader = io::BufReader::new(file);

        let x = result[1].parse::<usize>()?;

        let start = if x > 25_usize { x - 25 } else { 0 };
        let end = start + 50;
        app.search.line = x - start;

        app.search.preview = reader
            .lines()
            .enumerate()
            .skip(start)
            .take_while(|&(index, _)| index + 1 <= end)
            .map(|(_, line)| line.unwrap_or("".to_string()))
            .collect::<Vec<String>>()
            .join("\n");
        return Ok(());
    }

    Ok(())
}

pub fn handle_vi_command(app: &mut App) -> anyhow::Result<()> {
    match app.vi_command.as_ref() {
        "gg" => {
            if app.window == Window::Search {
                app.search.scroll = 0;
            } else {
                app.search.scroll = 0;
            }
        }
        "dd" => {
            app.search.query.clear();
            get_results(app)?;
        }
        _ => return Ok(()),
    };

    app.vi_command.clear();
    Ok(())
}

pub fn handle_exit_commands(app: &mut App) -> anyhow::Result<()> {
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
        ":q!" => {
            app.delete_session()?;
            app.quit();
        }
        _ => {}
    }

    Ok(())
}
