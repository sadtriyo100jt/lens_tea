use ratatui::{
    layout::{Alignment, Rect},
    style::{Color, Style, Styled, Stylize},
    text::{Span, Text},
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame,
};

use crate::{app::App, app::Window};

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
    let title = "Search";
    let spans = vec![
        Span::styled(&title[..1], Style::default().fg(Color::Red)),
        Span::raw(&title[1..]),
    ];

    let colors = Colors::new(&app.window);

    frame.render_widget(
        Paragraph::new(Span::styled(
            format!(">"),
            Style::default().fg(Color::Red).bold(),
        ))
        .block(
            Block::default()
                .title(spans)
                .title_alignment(Alignment::Center)
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded),
        )
        .style(Style::default().fg(colors.search).bg(Color::Black))
        .alignment(Alignment::Left),
        Rect::new(52, 0, frame.size().width - 53, 5),
    );

    let title = "Options";
    let spans = vec![
        Span::styled(&title[..1], Style::default().fg(Color::Red)),
        Span::raw(&title[1..]),
    ];

    frame.render_widget(
        Paragraph::new(format!("",))
            .block(
                Block::default()
                    .title(spans)
                    .title_alignment(Alignment::Center)
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded),
            )
            .style(Style::default().fg(colors.options).bg(Color::Black))
            .alignment(Alignment::Left),
        Rect::new(1, 0, 50, frame.size().height),
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
        Rect::new(52, 5, frame.size().width - 53, frame.size().height - 5),
    );
}
