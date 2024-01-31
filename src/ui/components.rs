use crate::app::{App, Mode, Window};
use ratatui::{
    layout::Alignment,
    style::{Color, Modifier, Style, Stylize},
    text::Span,
    widgets::{Block, BorderType, Borders, List, ListItem, Padding, Paragraph},
};
use tui_textarea::{CursorMove, TextArea};

pub fn text_area(app: &mut App) -> TextArea {
    let mut text_area = TextArea::default();
    text_area.set_cursor_line_style(Style::default());
    text_area.set_style(Style::default().fg(Color::LightRed));
    text_area.set_block(
        Block::default()
            .borders(Borders::ALL)
            .padding(Padding::new(3, 0, 0, 0)),
    );
    text_area.insert_str(app.query.iter().collect::<String>());
    text_area.move_cursor(CursorMove::Jump(0, app.cursor_pos as u16));
    text_area.set_cursor_style(
        Style::default()
            .fg(Color::LightBlue)
            .add_modifier(Modifier::REVERSED),
    );

    text_area
}

pub fn search(color: Color) -> Paragraph<'static> {
    let title = vec![
        Span::styled("S", Style::default().fg(Color::Red)),
        Span::raw("earch"),
    ];

    Paragraph::new(Span::styled(">", Style::default().fg(Color::Magenta)))
        .block(
            Block::default()
                .title(title)
                .title_alignment(Alignment::Center)
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .padding(Padding::new(1, 0, 0, 0)),
        )
        .style(Style::default().fg(color).bg(Color::Black))
        .alignment(Alignment::Left)
}

pub fn options<'a>(color: Color, app: &'a mut App) -> List<'a> {
    let title = vec![
        Span::styled("O", Style::default().fg(Color::Red)),
        Span::raw("ptions"),
    ];

    List::new(
        app.options
            .iter()
            .map(|item| ListItem::new(Span::from(item))),
    )
    .block(
        Block::default()
            .title(title)
            .title_alignment(Alignment::Center)
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .padding(Padding::new(1, 0, 0, 0)),
    )
    .style(Style::default().fg(color).bg(Color::Black))
    .highlight_style(Style::default().add_modifier(Modifier::BOLD))
    .highlight_symbol("> ")
}

pub fn results<'a>(app: &'a mut App) -> List<'a> {
    let items = app
        .result
        .iter()
        .map(|item| ListItem::new(Span::from(item)).style(Style::default().fg(Color::White)))
        .collect::<Vec<_>>();

    List::new(items)
        .block(
            Block::default()
                .title("Results")
                .title_alignment(Alignment::Center)
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded),
        )
        .style(Style::default().fg(Color::Blue).bg(Color::Black))
        .highlight_style(
            Style::default()
                .add_modifier(Modifier::BOLD)
                .fg(Color::Yellow),
        )
        .highlight_symbol(" > ")
}

pub fn preview<'a>(app: &'a mut App) -> List<'a> {
    List::new(app.preview.lines().enumerate().map(|(index, line)| {
        let item = ListItem::new(Span::from(line).style(Style::default().fg(Color::White)));
        if index + 1 == app.searched_line {
            return item.add_modifier(Modifier::REVERSED);
        }

        item
    }))
    .block(
        Block::default()
            .title("Preview")
            .title_alignment(Alignment::Center)
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded),
    )
    .style(Style::default().fg(Color::Blue).bg(Color::Black))
}

pub fn vi_bar(app: &mut App, color: Color) -> TextArea {
    let mut text_area = TextArea::default();
    text_area.set_cursor_line_style(Style::default());
    text_area.set_style(Style::default().fg(Color::White));
    text_area.set_block(
        Block::default()
            .borders(Borders::BOTTOM)
            .style(Style::default().fg(color)),
    );
    text_area.insert_str(app.command.query.iter().collect::<String>());
    text_area.move_cursor(CursorMove::Jump(0, app.command.cursor as u16));
    text_area.set_cursor_style(
        Style::default()
            .fg(Color::LightBlue)
            .add_modifier(Modifier::REVERSED),
    );

    if app.window != Window::Command {
        text_area.set_cursor_style(
            Style::default()
                .fg(Color::LightBlue)
                .add_modifier(Modifier::HIDDEN),
        );
    }

    text_area
}

pub fn mode(app: &mut App) -> Paragraph<'static> {
    if app.window != Window::Search {
        return Paragraph::new("");
    }
    let mode = match app.mode {
        Mode::Normal => "NORMAL",
        Mode::Insert => "INSERT",
    };

    Paragraph::new(mode).block(Block::default().padding(Padding::new(1, 0, 0, 0)))
}

pub fn current_command(app: &mut App) -> Paragraph<'static> {
    Paragraph::new(app.vi_command.clone())
        .block(Block::default().padding(Padding::new(0, 10, 0, 0)))
        .alignment(Alignment::Right)
}
