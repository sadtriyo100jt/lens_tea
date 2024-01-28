use crate::app::App;
use ratatui::{
    layout::Alignment,
    style::{Color, Modifier, Style},
    text::Span,
    widgets::{Block, BorderType, Borders, List, Padding, Paragraph},
};
use tui_textarea::{CursorMove, TextArea};

pub fn text_area(app: &mut App) -> TextArea {
    let mut text_area = TextArea::default();
    text_area.set_cursor_line_style(Style::default());
    text_area.set_style(Style::default().fg(Color::LightRed));
    text_area.set_block(
        Block::default()
            .borders(Borders::ALL)
            .padding(Padding::new(3, 0, 1, 0)),
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
                .padding(Padding::new(1, 0, 1, 0)),
        )
        .style(Style::default().fg(color).bg(Color::Black))
        .alignment(Alignment::Left)
}

pub fn options(color: Color) -> List<'static> {
    let title = vec![
        Span::styled("O", Style::default().fg(Color::Red)),
        Span::raw("ptions"),
    ];

    List::new([
        "hidden files",
        "files without match",
        "case sensitive",
        "stop on nonmatch",
    ])
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

pub fn results(app: &mut App) -> List<'static> {
List::new(app.result.clone())
            .block(
                Block::default()
                    .title("Results")
                    .title_alignment(Alignment::Center)
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded),
            )
            .style(Style::default().fg(Color::Cyan).bg(Color::Black))
            .highlight_style(Style::default().add_modifier(Modifier::BOLD))
            .highlight_symbol(" > ")
}

pub fn preview(app: &mut App) -> Paragraph {
Paragraph::new(&*app.preview)
            .block(
                Block::default()
                    .title("Preview")
                    .title_alignment(Alignment::Center)
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded),
            )
            .style(Style::default().fg(Color::Cyan).bg(Color::Black))
            .alignment(Alignment::Left)
}
