use crate::app::App;
use std::{env, process::Command};
use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
};

pub fn open_editor(app: &mut App) -> anyhow::Result<()> {
    let editor = match env::var("EDITOR") {
        Ok(editor) => editor,
        Err(_) => return Ok(()),
    };
    let result = app
        .result
        .get(app.scroll.result)
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
        .arg(result[0])
        .spawn()?
        .wait()?;

    Ok(())
}

pub fn get_results(app: &mut App) -> anyhow::Result<()> {
    if app.query.len() > 0 {
        app.result = String::from_utf8_lossy(
            &Command::new("rg")
                .arg("--color=never")
                .arg("--no-heading")
                .arg("--with-filename")
                .arg("--line-number")
                .arg("--column")
                .arg("--smart-case")
                .arg("--hidden")
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

pub fn get_preview(app: &mut App) -> anyhow::Result<()> {
    if app.result.len() == 0 {
        return Ok(());
    }
    let result = app
        .result
        .get(app.scroll.result)
        .unwrap()
        .split(":")
        .collect::<Vec<&str>>();
    let file = File::open(Path::new(result[0]));

    if let Ok(file) = file {
        let reader = io::BufReader::new(file);

        let x = result[1].parse::<usize>()?;

        let start = if x > 25_usize { x - 25 } else { 0 };
        let end = start + 50;
        app.searched_line = x - start;

        app.preview = reader
            .lines()
            .enumerate()
            .skip(start)
            .take_while(|&(index, _)| index + 1 <= end)
            .map(|(_, line)| line.unwrap_or("".to_string()))
            .collect::<Vec<String>>()
            .join("\n");
        return Ok(());
    }

    app.preview = String::new();
    Ok(())
}
