use crate::{app::App, app::Window};
use ratatui::{
    layout::{Alignment, Rect},
    style::{Color, Style},
    text::Span,
    widgets::{Block, BorderType, Borders, Padding, Paragraph},
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
            .padding(Padding::new(8, 0, 1, 0)),
    );
    text_area.insert_str(app.query.iter().collect::<String>());
    text_area.move_cursor(CursorMove::Jump(0, app.cursor_pos));
    frame.render_widget(text_area.widget(), Rect::new(17, 0, 100, 5));

    let colors = Colors::new(&app.window);
    let title = vec![
        Span::styled("S", Style::default().fg(Color::Red)),
        Span::raw("earch"),
    ];
    frame.render_widget(
        Paragraph::new(Span::styled("> grep", Style::default().fg(Color::White)))
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
        Rect::new(17, 0, 100, 5),
    );

    let title = vec![
        Span::styled("O", Style::default().fg(Color::Red)),
        Span::raw("ptions"),
    ];

    frame.render_widget(
        Paragraph::new("")
            .block(
                Block::default()
                    .title(title)
                    .title_alignment(Alignment::Center)
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded),
            )
            .style(Style::default().fg(colors.options).bg(Color::Black))
            .alignment(Alignment::Left),
        Rect::new(1, 0, 15, frame.size().height),
    );

    frame.render_widget(
        Paragraph::new(format!("",))
            .block(
                Block::default()
                    .title("Results")
                    .title_alignment(Alignment::Center)
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded),
            )
            .style(Style::default().fg(Color::Cyan).bg(Color::Black))
            .alignment(Alignment::Center),
        Rect::new(17, 5, 100, frame.size().height - 5),
    );

    frame.render_widget(
        Paragraph::new(format!("",))
            .block(
                Block::default()
                    .title("Preview")
                    .title_alignment(Alignment::Center)
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded),
            )
            .style(Style::default().fg(Color::Cyan).bg(Color::Black))
            .alignment(Alignment::Center),
        Rect::new(118, 0, 91, frame.size().height),
    );
}
