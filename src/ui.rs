use crate::{app::App, app::Window};
use ratatui::{
    layout::{Alignment, Rect},
    style::{Color, Modifier, Style},
    text::Span,
    widgets::{Block, BorderType, Borders, List, ListState, Padding, Paragraph},
    Frame,
};
use tui_textarea::{CursorMove, TextArea};

struct Colors {
    search: Color,
    options: Color,
}

impl Colors {
    fn new(chosen_window: &Window) -> Self {
        Self {
            search: if chosen_window == &Window::Search {
                Color::Blue
            } else {
                Color::Cyan
            },
            options: if chosen_window == &Window::Options {
                Color::Blue
            } else {
                Color::Cyan
            },
        }
    }
}

pub fn render(app: &mut App, frame: &mut Frame) {
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
    frame.render_widget(text_area.widget(), Rect::new(27, 0, 90, 5));

    let colors = Colors::new(&app.window);
    let title = vec![
        Span::styled("S", Style::default().fg(Color::Red)),
        Span::raw("earch"),
    ];
    frame.render_widget(
        Paragraph::new(Span::styled(">", Style::default().fg(Color::Magenta)))
            .block(
                Block::default()
                    .title(title)
                    .title_alignment(Alignment::Center)
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .padding(Padding::new(1, 0, 1, 0)),
            )
            .style(Style::default().fg(colors.search).bg(Color::Black))
            .alignment(Alignment::Left),
        Rect::new(27, 0, 90, 5),
    );

    let title = vec![
        Span::styled("O", Style::default().fg(Color::Red)),
        Span::raw("ptions"),
    ];

    frame.render_widget(
        List::new([
            "- hidden files",
            "- files without match",
            "- case sensitive",
            "- stop on nonmatch",
        ])
        .block(
            Block::default()
                .title(title)
                .title_alignment(Alignment::Center)
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .padding(Padding::new(1, 0, 0, 0)),
        )
        .style(Style::default().fg(colors.options).bg(Color::Black)),
        Rect::new(1, 0, 25, frame.size().height),
    );

    let mut list_state = ListState::default();
    list_state.select(Some(app.result_scroll));

    frame.render_stateful_widget(
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
            .highlight_symbol("> "),
        Rect::new(27, 5, 90, frame.size().height - 5),
        &mut list_state,
    );

    frame.render_widget(
        Paragraph::new(app.preview.clone())
            .block(
                Block::default()
                    .title("Preview")
                    .title_alignment(Alignment::Center)
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded),
            )
            .style(Style::default().fg(Color::Cyan).bg(Color::Black))
            .alignment(Alignment::Left),
        Rect::new(118, 0, 91, frame.size().height),
    );
}
